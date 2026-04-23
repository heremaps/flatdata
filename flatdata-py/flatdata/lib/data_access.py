'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import numpy as np

# Sign bits cache for the value reading.
_SIGN_BITS = [0] + [(1 << (bits - 1)) for bits in range(1, 65)]


def make_field_reader(offset_bits, num_bits, is_signed):
    """Build a specialized closure for reading a single field from a structure.

    Returns a function reader(data, pos_bytes) that reads the field value
    from ``data`` at byte position ``pos_bytes``.  All constants (byte offset,
    bit shift, mask, sign handling) are pre-computed and captured by the
    closure so the hot path does minimal work.
    """
    offset_bytes, offset_extra = divmod(offset_bits, 8)
    total_bytes = (num_bits + 7) // 8
    end_byte = offset_bytes + total_bytes
    mask = (1 << num_bits) - 1
    needs_extra = (total_bytes * 8 - offset_extra) < num_bits
    extra_shift = total_bytes * 8 - offset_extra

    if num_bits == 1:
        bit_mask = 1 << offset_extra
        def reader(data, pos):
            return int((data[pos + offset_bytes] & bit_mask) != 0)
        return reader

    if is_signed:
        sign_bit = _SIGN_BITS[num_bits]
        sign_mask = sign_bit - 1
        if needs_extra:
            def reader(data, pos):
                result = int.from_bytes(
                    data[pos + offset_bytes: pos + end_byte], byteorder="little")
                result >>= offset_extra
                result |= data[pos + end_byte] << extra_shift
                result &= mask
                return (result & sign_mask) - (result & sign_bit)
        elif offset_extra:
            def reader(data, pos):
                result = (int.from_bytes(
                    data[pos + offset_bytes: pos + end_byte],
                    byteorder="little") >> offset_extra) & mask
                return (result & sign_mask) - (result & sign_bit)
        else:
            def reader(data, pos):
                result = int.from_bytes(
                    data[pos + offset_bytes: pos + end_byte],
                    byteorder="little") & mask
                return (result & sign_mask) - (result & sign_bit)
        return reader

    # Unsigned paths
    if needs_extra:
        def reader(data, pos):
            result = int.from_bytes(
                data[pos + offset_bytes: pos + end_byte], byteorder="little")
            result >>= offset_extra
            result |= data[pos + end_byte] << extra_shift
            return result & mask
    elif offset_extra:
        def reader(data, pos):
            return (int.from_bytes(
                data[pos + offset_bytes: pos + end_byte],
                byteorder="little") >> offset_extra) & mask
    else:
        def reader(data, pos):
            return int.from_bytes(
                data[pos + offset_bytes: pos + end_byte],
                byteorder="little") & mask
    return reader


def read_field_vectorized(raw_bytes_2d, field_offset_bits, field_width_bits, is_signed):
    """Read a bit-packed field from all elements at once, returning a numpy array.

    :param raw_bytes_2d: numpy uint8 array shaped (num_elements, struct_size_bytes)
    :param field_offset_bits: bit offset of the field within each element
    :param field_width_bits: width of the field in bits (max 64)
    :param is_signed: whether to sign-extend the result
    :return: numpy array of field values
    """
    if field_width_bits == 1:
        byte_idx = field_offset_bits // 8
        bit_idx = field_offset_bits % 8
        return ((raw_bytes_2d[:, byte_idx].astype(np.uint64) >> np.uint64(bit_idx)) &
                np.uint64(1))

    byte_start = field_offset_bits // 8
    bit_shift = field_offset_bits % 8
    bytes_needed = (bit_shift + field_width_bits + 7) // 8

    # Use Python int arithmetic for the shift to avoid numpy overflow,
    # then broadcast back to the array.
    result = np.zeros(raw_bytes_2d.shape[0], dtype=np.uint64)
    for b in range(min(bytes_needed, 8)):
        result |= raw_bytes_2d[:, byte_start + b].astype(np.uint64) << np.uint64(b * 8)
    result >>= np.uint64(bit_shift)

    # If the field spans more than 8 bytes (unaligned 64-bit field), merge the extra byte.
    bits_so_far = 8 * min(bytes_needed, 8) - bit_shift
    if bits_so_far < field_width_bits and bytes_needed > 8:
        extra = raw_bytes_2d[:, byte_start + 8].astype(np.uint64)
        result |= extra << np.uint64(bits_so_far)

    if field_width_bits < 64:
        result &= np.uint64((1 << field_width_bits) - 1)

    if is_signed:
        if field_width_bits == 64:
            return result.view(np.int64)
        sign_bit = np.uint64(1 << (field_width_bits - 1))
        offset = -(1 << field_width_bits)
        signed = result.astype(np.int64) + np.int64(offset)
        result = np.where(result & sign_bit, signed, result.astype(np.int64))

    return result


def read_value(data, offset_bits, num_bits, is_signed):
    """Read a bit-packed value from data at the given bit offset.

    This is a convenience wrapper around :func:`make_field_reader` for one-off
    reads.  For repeated reads of the same field, prefer building a reader once
    with ``make_field_reader`` and reusing it.
    """
    reader = make_field_reader(offset_bits, num_bits, is_signed)
    return reader(data, 0)


def write_value(data, offset_bits, num_bits, is_signed, value):
    assert num_bits <= 64, f'Number of bits to write is greater than 64'

    offset_bytes, offset_extra_bits = divmod(offset_bits, 8)
    total_bytes = (num_bits + 7) // 8

    if num_bits == 1:
        if value == 1:
            data[offset_bytes] |= 1 << offset_extra_bits
        else:
            data[offset_bytes] &= ~(1 << offset_extra_bits)
        return

    mask = (1 << num_bits) - 1
    value <<= offset_extra_bits
    value &= mask << offset_extra_bits
    value_in_little_endian = value.to_bytes(total_bytes + 1, byteorder="little", signed=is_signed)
    surrounding_bits = data[offset_bytes] & ((1 << offset_bits) - 1)

    byte_idx = 0
    data[offset_bytes] = value_in_little_endian[byte_idx]
    data[offset_bytes] |= surrounding_bits

    byte_idx += 1
    while byte_idx < total_bytes:
        data[offset_bytes + byte_idx] = value_in_little_endian[byte_idx]
        byte_idx += 1

    bits_written = total_bytes * 8 - offset_extra_bits
    if bits_written < num_bits:
        surrounding_bits = data[offset_bytes + byte_idx] & ~((1 << offset_bits) - 1)
        data[offset_bytes + byte_idx] = value_in_little_endian[byte_idx] & ((1 << (8 - (bits_written % 8))) - 1)
        data[offset_bytes + byte_idx] |= surrounding_bits

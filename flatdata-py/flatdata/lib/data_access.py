'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

# Sign bits cache for the value reading.
_SIGN_BITS = [0] + [(1 << (bits - 1)) for bits in range(1, 65)]


def read_value(data, offset_bits, num_bits, is_signed):
    offset_bytes, offset_extra_bits = divmod(offset_bits, 8)
    total_bytes = (num_bits + 7) // 8

    if num_bits == 1:
        return int((data[offset_bytes] & (1 << offset_extra_bits)) != 0)

    result = int.from_bytes(data[offset_bytes: offset_bytes + total_bytes], byteorder="little")
    result >>= offset_extra_bits
    if (total_bytes * 8 - offset_extra_bits) < num_bits:
        remainder = data[offset_bytes + total_bytes]
        result |= remainder << (total_bytes * 8 - offset_extra_bits)

    if num_bits < 64:
        result = result & ((1 << num_bits) - 1)

    if not is_signed:
        return result

    return (result & (_SIGN_BITS[num_bits] - 1)) - (result & _SIGN_BITS[num_bits])


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

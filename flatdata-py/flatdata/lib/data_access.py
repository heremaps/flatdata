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
    offset_bytes, offset_extra_bits = divmod(offset_bits, 8)
    total_bytes = (num_bits + 7) // 8

    if num_bits == 1:
        # TODO: check that value is either 0 or 1?
        data[offset_bytes] |= value << offset_extra_bits
        return
    
    byte_value = bytearray(value.to_bytes(total_bytes+1, byteorder="little", signed=is_signed))
    # extra byte to avoid extra logic in the loop
    byte_value[total_bytes] = 0
    # mask out bits we don't need
    byte_value[total_bytes-1] &= 0xff >> (num_bits % 8)
    for idx in range(total_bytes):
        data[offset_bytes + idx] |= (
                (byte_value[idx] << offset_extra_bits) & 0xff
            ) | (
                (byte_value[idx+1] >> (8-offset_extra_bits) ) & 0xff
            )

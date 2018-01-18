'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

"""
Sign bits cache for the value reading.
"""
_SIGN_BITS = [0] + [(1 << (bits - 1)) for bits in range(1, 65)]


def read_value(data, offset_bits, num_bits, is_signed):
    """
    Reads numeric value from a stream of bytes, at the given bit-offset, with given length.
    :param data: Array of bytes
    :param offset_bits: Offset of the value in bits
    :param num_bits: Number of bits in the value
    :param is_signed: Interpret the value as a signed numeric value or unsigned.
    """

    current_index = offset_bits // 8
    current_byte = data[current_index]

    bits_left = num_bits
    local_offset = offset_bits % 8

    result = 0
    if local_offset != 0:
        result = current_byte >> local_offset
        if bits_left <= (8 - local_offset):
            result &= (1 << bits_left) - 1
            return result
        bits_left -= 8 - local_offset
        current_index += 1
        current_byte = data[current_index]

    while bits_left >= 8:
        temp = current_byte
        temp <<= num_bits - bits_left
        result |= temp

        bits_left -= 8
        current_index += 1
        current_byte = data[current_index]

    if bits_left != 0:
        temp = current_byte & ((1 << bits_left) - 1)
        temp <<= num_bits - bits_left
        result |= temp

    if not is_signed:
        return result
    return (result & (_SIGN_BITS[num_bits] - 1)) - (result & _SIGN_BITS[num_bits])

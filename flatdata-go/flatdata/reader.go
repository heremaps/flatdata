/*
Copyright (c) 2017 HERE Europe B.V.
See the LICENSE file in the root of this project for license details.
*/

package flatdata

import "math"

var signBits []uint

func init() {
	signBits = make([]uint, 64)
	for i := 0; i < 64; i++ {
		signBits[i] = uint(math.Exp2(float64(i)))
	}
}

// Read allows to read data from flatdata memory layout
func Read(descriptor MemoryDescriptor, offset, size uint, signed bool) int {
	currentIndex := offset / 8
	currentByte := uint(descriptor.At(int(currentIndex)))
	bitsLeft := size
	localOffset := offset % 8
	var result uint

	if localOffset != 0 {
		result = currentByte >> localOffset
		if bitsLeft <= (8 - localOffset) {
			result &= (1 << bitsLeft) - 1
			return int(result)
		}
		bitsLeft -= 8 - localOffset
		currentIndex++
		currentByte = uint(descriptor.At(int(currentIndex)))
	}

	for bitsLeft >= 8 {
		temp := currentByte
		temp <<= size - bitsLeft
		result |= temp

		bitsLeft -= 8
		currentIndex++
		currentByte = uint(descriptor.At(int(currentIndex)))
	}

	if bitsLeft != 0 {
		temp := currentByte & ((1 << bitsLeft) - 1)
		temp <<= size - bitsLeft
		result |= temp
	}

	if signed {
		return int((result & (signBits[size-1] - 1)) - (result & signBits[size-1]))
	}

	return int(result)
}

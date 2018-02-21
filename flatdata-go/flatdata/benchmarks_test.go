/*
Copyright (c) 2017 HERE Europe B.V.
See the LICENSE file in the root of this project for license details.
*/

package flatdata

import (
	"testing"

	"math/rand"
)

const arraySize = 8192

func BenchmarkFileResourceReadUnsigned(b *testing.B) {
	runBenchmarkOnFileResource(false, b)
}

func BenchmarkFileResourceReadSigned(b *testing.B) {
	runBenchmarkOnFileResource(true, b)
}

func runBenchmarkOnFileResource(signed bool, b *testing.B) {
	b.StopTimer()

	handle := &TestMemoryDescriptor{Array: generateRandomByteArray()}

	b.StartTimer()

	for n := 0; n < b.N; n++ {
		Read(handle, 4092, 64, signed)
	}
}

func generateRandomByteArray() []byte {
	bytes := make([]byte, arraySize)
	rand.Read(bytes)
	return bytes
}

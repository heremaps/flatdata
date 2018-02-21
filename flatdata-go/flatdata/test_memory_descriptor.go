/*
Copyright (c) 2017 HERE Europe B.V.
See the LICENSE file in the root of this project for license details.
*/

package flatdata

import (
	"fmt"
	"io"
)

// TestMemoryDescriptor implements MemoryDescriptor interface with reading date from []byte for internal usage
type TestMemoryDescriptor struct {
	Array []byte
}

// ReadAt number of bytes equal to p starting from offset off
func (b *TestMemoryDescriptor) ReadAt(p []byte, off int64) (n int, err error) {
	if off < 0 || int64(len(b.Array)) < off {
		return 0, fmt.Errorf("invalid offset %d", off)
	}
	n = copy(p, b.Array[off:])
	if n < len(p) {
		return n, io.EOF
	}
	return n, nil
}

// Close should close memory descriptor
func (b *TestMemoryDescriptor) Close() error {
	return nil
}

// Len returns length of internal byte array
func (b *TestMemoryDescriptor) Len() int {
	return len(b.Array)
}

// At return byte on position i
func (b *TestMemoryDescriptor) At(i int) byte {
	return b.Array[i]
}

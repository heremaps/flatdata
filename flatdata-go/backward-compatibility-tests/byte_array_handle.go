/*
Copyright (c) 2017 HERE Europe B.V.
See the LICENSE file in the root of this project for license details.
*/

package backwardcompatibility

import (
	"fmt"
	"io"
)

// byteArrayHandle implements ResourceHandle interface with reading date from []byte for internal usage
type byteArrayHandle struct {
	Array    []byte
	fakeSize bool
}

func (b *byteArrayHandle) ReadAt(p []byte, off int64) (n int, err error) {
	if off < 0 || int64(len(b.Array)) < off {
		return 0, fmt.Errorf("invalid offset %d", off)
	}
	n = copy(p, b.Array[off:])
	if n < len(p) {
		return n, io.EOF
	}
	return n, nil
}

func (b *byteArrayHandle) Close() error {
	return nil
}

func (b *byteArrayHandle) Len() int {
	if b.fakeSize {
		if len(b.Array) < 16 {
			return 16
		}
	}
	return len(b.Array)
}

func (b *byteArrayHandle) At(i int) byte {
	return b.Array[i]
}

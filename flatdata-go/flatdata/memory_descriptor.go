/*
Copyright (c) 2017 HERE Europe B.V.
See the LICENSE file in the root of this project for license details.
*/

package flatdata

import "io"

// MemoryDescriptor represents internal low level data access
type MemoryDescriptor interface {
	io.ReaderAt
	Close() error
	Len() int
	At(i int) byte
}

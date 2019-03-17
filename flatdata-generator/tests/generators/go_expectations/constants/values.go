package n

import (
	"bytes"
	"encoding/binary"
	"errors"
	"fmt"
	"log"

	"github.com/heremaps/flatdata/flatdata-go/flatdata"
)

const (
    flatdataOffsetSizeInBytes uint = 8
    flatdataPaddingSizeInBytes uint = 8
    FooI8Neg int8 = -128
    FooI8Pos int8 = 127
    FooI8Zero int8 = 0
    FooI8NegHex int8 = -128
    FooI8PosHex int8 = 127
    FooI8ZeroHex int8 = 0
    FooU8Pos uint8 = 255
    FooU8Zero uint8 = 0
    FooU8PosHex uint8 = 255
    FooU8ZeroHex uint8 = 0
    FooI16Neg int16 = -32768
    FooI16Pos int16 = 32767
    FooI16Zero int16 = 0
    FooI16NegHex int16 = -32768
    FooI16PosHex int16 = 32767
    FooI16ZeroHex int16 = 0
    FooU16Pos uint16 = 65535
    FooU16Zero uint16 = 0
    FooU16PosHex uint16 = 65535
    FooU16ZeroHex uint16 = 0
    FooI32Neg int32 = -2147483648
    FooI32Pos int32 = 2147483647
    FooI32Zero int32 = 0
    FooI32NegHex int32 = -2147483648
    FooI32PosHex int32 = 2147483647
    FooI32ZeroHex int32 = 0
    FooU32Pos uint32 = 4294967295
    FooU32Zero uint32 = 0
    FooU32PosHex uint32 = 4294967295
    FooU32ZeroHex uint32 = 0
    FooI64Neg int64 = -9223372036854775808
    FooI64Pos int64 = 9223372036854775807
    FooI64Zero int64 = 0
    FooI64NegHex int64 = -9223372036854775808
    FooI64PosHex int64 = 9223372036854775807
    FooI64ZeroHex int64 = 0
    FooU64Pos uint64 = 18446744073709551615
    FooU64Zero uint64 = 0
    FooU64PosHex uint64 = 18446744073709551615
    FooU64ZeroHex uint64 = 0
)

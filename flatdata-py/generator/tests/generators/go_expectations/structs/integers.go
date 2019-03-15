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
    u8SizeInBytes = 1
    i8SizeInBytes = 1
    u16SizeInBytes = 2
    i16SizeInBytes = 2
    u32SizeInBytes = 4
    i32SizeInBytes = 4
    u64SizeInBytes = 8
    i64SizeInBytes = 8
)

type U8 struct {
    descriptor flatdata.MemoryDescriptor
	position int
}

func (v *U8) GetF() uint8 {
    elementSizeInBits := uint(8)
    elementOffset := uint(0)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, false)
    return uint8(result)
}
    
    
func (v *U8) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "U8", "position": %d, "attributes": {`, v.position))
    if v.descriptor != nil {
        buffer.WriteString(fmt.Sprintf(`"f": %v`, v.GetF()))
	}
    buffer.WriteString("}}")
	return buffer.String()
}
    

type I8 struct {
    descriptor flatdata.MemoryDescriptor
	position int
}

func (v *I8) GetF() int8 {
    elementSizeInBits := uint(8)
    elementOffset := uint(0)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, true)
    return int8(result)
}
    
    
func (v *I8) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "I8", "position": %d, "attributes": {`, v.position))
    if v.descriptor != nil {
        buffer.WriteString(fmt.Sprintf(`"f": %v`, v.GetF()))
	}
    buffer.WriteString("}}")
	return buffer.String()
}
    

type U16 struct {
    descriptor flatdata.MemoryDescriptor
	position int
}

func (v *U16) GetF() uint16 {
    elementSizeInBits := uint(16)
    elementOffset := uint(0)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, false)
    return uint16(result)
}
    
    
func (v *U16) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "U16", "position": %d, "attributes": {`, v.position))
    if v.descriptor != nil {
        buffer.WriteString(fmt.Sprintf(`"f": %v`, v.GetF()))
	}
    buffer.WriteString("}}")
	return buffer.String()
}
    

type I16 struct {
    descriptor flatdata.MemoryDescriptor
	position int
}

func (v *I16) GetF() int16 {
    elementSizeInBits := uint(16)
    elementOffset := uint(0)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, true)
    return int16(result)
}
    
    
func (v *I16) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "I16", "position": %d, "attributes": {`, v.position))
    if v.descriptor != nil {
        buffer.WriteString(fmt.Sprintf(`"f": %v`, v.GetF()))
	}
    buffer.WriteString("}}")
	return buffer.String()
}
    

type U32 struct {
    descriptor flatdata.MemoryDescriptor
	position int
}

func (v *U32) GetF() uint32 {
    elementSizeInBits := uint(32)
    elementOffset := uint(0)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, false)
    return uint32(result)
}
    
    
func (v *U32) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "U32", "position": %d, "attributes": {`, v.position))
    if v.descriptor != nil {
        buffer.WriteString(fmt.Sprintf(`"f": %v`, v.GetF()))
	}
    buffer.WriteString("}}")
	return buffer.String()
}
    

type I32 struct {
    descriptor flatdata.MemoryDescriptor
	position int
}

func (v *I32) GetF() int32 {
    elementSizeInBits := uint(32)
    elementOffset := uint(0)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, true)
    return int32(result)
}
    
    
func (v *I32) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "I32", "position": %d, "attributes": {`, v.position))
    if v.descriptor != nil {
        buffer.WriteString(fmt.Sprintf(`"f": %v`, v.GetF()))
	}
    buffer.WriteString("}}")
	return buffer.String()
}
    

type U64 struct {
    descriptor flatdata.MemoryDescriptor
	position int
}

func (v *U64) GetF() uint64 {
    elementSizeInBits := uint(64)
    elementOffset := uint(0)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, false)
    return uint64(result)
}
    
    
func (v *U64) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "U64", "position": %d, "attributes": {`, v.position))
    if v.descriptor != nil {
        buffer.WriteString(fmt.Sprintf(`"f": %v`, v.GetF()))
	}
    buffer.WriteString("}}")
	return buffer.String()
}
    

type I64 struct {
    descriptor flatdata.MemoryDescriptor
	position int
}

func (v *I64) GetF() int64 {
    elementSizeInBits := uint(64)
    elementOffset := uint(0)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, true)
    return int64(result)
}
    
    
func (v *I64) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "I64", "position": %d, "attributes": {`, v.position))
    if v.descriptor != nil {
        buffer.WriteString(fmt.Sprintf(`"f": %v`, v.GetF()))
	}
    buffer.WriteString("}}")
	return buffer.String()
}
    

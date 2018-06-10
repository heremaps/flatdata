'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from generator.generators.GoGenerator import GoGenerator

from .assertions import *


def test_imports_and_constants_generation():
    generate_and_assert_in("""namespace xyz{
        
}
    """, GoGenerator, """/////////////////////////////////////////////////////////////////////////
//    ATTENTION!
//    This code is automatically generated by flatdata generator.
//    Any modifications to this file will be lost upon next regeneration.
/////////////////////////////////////////////////////////////////////////
package xyz

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
)""")


def test_constants_are_declared_correctly():
    generate_and_assert_in("""
        namespace n{
        const i8 foo = 17;
        const u16 bar = 0x42;
        }
    """, GoGenerator, """
    Bar uint16 = 0x42
    Foo int8 = 17
    """)


def test_structures_are_declared_correctly():
    expected_lines = [
        """sSizeInBytes = 3""",
        """type S struct {
    descriptor flatdata.MemoryDescriptor
	position int
}""",
        """func (v *S) GetF0() uint8 {
    elementSizeInBits := uint(3)
    elementOffset := uint(0)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, false)
    return uint8(result)
}""",
        """func (v *S) GetF1() uint16 {
    elementSizeInBits := uint(15)
    elementOffset := uint(3)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, false)
    return uint16(result)
}""",
        """func (v *S) ToString() string {"""
    ]

    generate_and_assert_in("""
        namespace n{
        struct S {
            f0 : u8 : 3;
            f1 : u16 : 15;
        }
        }
    """, GoGenerator, *expected_lines)


def test_archives_are_declared_correctly():
    expected_lines = [
        """type AArchive struct {
    IsOptional bool
    IsOpen bool
    R0Instance *AR0Instance
}

func (v *AArchive) Close() {
    if v.R0Instance.IsOpen {
        v.R0Instance.Close()
    }
}

func (v *AArchive) GetSizeInBytes() int {
    var size int
    if v.R0Instance.IsOpen {
        size += v.R0Instance.GetSizeInBytes()
    }
    return size
}""",
        """func OpenAArchive(resource flatdata.ResourceStorage) (*AArchive, error) {
    v := &AArchive{}
    // Initialize resources
	r0IsOpen := true
	r0MemoryDescriptor, schema, err := resource.GetMemoryDescriptor("r0")
	if err != nil {
        log.Println(err)
	    if err.Error() == flatdata.ErrorCantAccessResource {
	        r0IsOpen = false
	    } else {
		    return v, err
		}
	}""",
        """// Add resources to archive
    v.R0Instance = &AR0Instance {
        descriptor: r0MemoryDescriptor, 
        IsOptional: false,
        IsOpen: r0IsOpen,
    }
	return v, nil
}"""
    ]

    generate_and_assert_in("""
    namespace n{
    struct S {
            f0 : u8 : 3;
            f1 : u16 : 15;
    }
    archive A {
        r0 : S;
    }
    }
    """, GoGenerator, *expected_lines)


def test_vector_resource_is_declared_correctly():
    expected_lines = [
        """type AVectorResourceVector struct {
    descriptor flatdata.MemoryDescriptor
    IsOptional bool
    IsOpen bool
}

func (v *AVectorResourceVector) Get(i int) *T {
	return &T{
		descriptor: v.descriptor,
		position: int(uint(i*tSizeInBytes) + flatdataOffsetSizeInBytes),
	}
}

func (v *AVectorResourceVector) GetSize() int {
	size := make([]byte, 8)
	_, err := v.descriptor.ReadAt(size, 0)
	if err != nil {
		return 0
	}
	intSize := binary.LittleEndian.Uint64(size)
	return int(intSize) / tSizeInBytes
}

func (v *AVectorResourceVector) GetSlice(start, end, step int) []*T {
	var result []*T	
	for start <= end {
		result = append(result, &T{
			descriptor: v.descriptor,
			position: int(uint(start*tSizeInBytes) + flatdataOffsetSizeInBytes),
		})
		start += step
	}
	return result
}

func (v *AVectorResourceVector) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *AVectorResourceVector) GetSizeInBytes() int {
    return v.descriptor.Len()
}""" 
    ]

    generate_and_assert_in("""
    namespace n{
    struct T {
        f0 : u8 : 3;
    }
    archive A {
        vector_resource : vector< T >;
    }
    }
    """, GoGenerator, *expected_lines)


def test_multi_vector_resource_is_declared_correctly():
    expected_lines = [
        """indexType33SizeInBytes = 5""",
        """type IndexType33 struct {
    descriptor flatdata.MemoryDescriptor
	position int
}

func (v *IndexType33) GetValue() uint64 {
    elementSizeInBits := uint(33)
    elementOffset := uint(0)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, false)
    return uint64(result)
}""",
        """type AMultivectorResourceVector struct {
    descriptor flatdata.MemoryDescriptor
    IsOptional bool
    IsOpen bool
}

func (v *AMultivectorResourceVector) Get(i int) *IndexType33 {
	return &IndexType33{
		descriptor: v.descriptor,
		position: int(uint(i*indexType33SizeInBytes) + flatdataOffsetSizeInBytes),
	}
}

func (v *AMultivectorResourceVector) GetSize() int {
	size := make([]byte, 8)
	_, err := v.descriptor.ReadAt(size, 0)
	if err != nil {
		return 0
	}
	intSize := binary.LittleEndian.Uint64(size)
	return int(intSize) / indexType33SizeInBytes
}

func (v *AMultivectorResourceVector) GetSlice(start, end, step int) []*IndexType33 {
	var result []*IndexType33	
	for start <= end {
		result = append(result, &IndexType33{
			descriptor: v.descriptor,
			position: int(uint(start*indexType33SizeInBytes) + flatdataOffsetSizeInBytes),
		})
		start += step
	}
	return result
}

func (v *AMultivectorResourceVector) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *AMultivectorResourceVector) GetSizeInBytes() int {
    return v.descriptor.Len()
}""",
        """type AMultivectorResourceMultivector struct {
    descriptor flatdata.MemoryDescriptor
    index      *AMultivectorResourceVector
	types      map[int]interface{}
    IsOptional bool
    IsOpen bool
}

func (v *AMultivectorResourceMultivector) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *AMultivectorResourceMultivector) GetSize() int {
	return v.index.GetSize()
}

func (v *AMultivectorResourceMultivector) GetSizeInBytes() int {
    return v.descriptor.Len()
}

func (v *AMultivectorResourceMultivector) getBucketOffset(i int) int {
	if i == v.index.GetSize() {
		return v.descriptor.Len() - int(flatdataPaddingSizeInBytes)
	} 
	return int(v.index.Get(i).GetValue()) + int(flatdataOffsetSizeInBytes)
}

func (v *AMultivectorResourceMultivector) Get(i int) []interface{} {
    offset := v.getBucketOffset(i)
	nextOffset := v.getBucketOffset(i + 1)
	var result []interface{}

	for offset < nextOffset {
	    elementType := flatdata.Read(v.descriptor, uint(offset*8), 8, false)
		offset++
		abstractElement, ok := v.types[elementType]
		if !ok {
			//TODO: How to process case, then type of element is not found?
			log.Println("Can't get type of element")
		}
		
		switch element := abstractElement.(type) {
		case *T:
			element.position = offset
			result = append(result, element)
			offset += tSizeInBytes
		case *U:
			element.position = offset
			result = append(result, element)
			offset += uSizeInBytes
		default:
			//TODO: How to react in case if it's impossible to cast?
			log.Println("Can't cast element. Type is unknown...")
		}
	}
	
	return result
}"""
    ]

    generate_and_assert_in("""
    namespace n{
    struct U {
        f0 : u8 : 3;
    }
    struct T {
        f0 : u8 : 3;
    }
    archive A {
        multivector_resource : multivector< 33, T, U >;
    }
    }
    """, GoGenerator, *expected_lines)


def test_raw_data_resource_is_declared_correctly():
    expected_lines = [
        """type ARawDataResourceRawData struct {
    descriptor flatdata.MemoryDescriptor
    IsOptional bool
    IsOpen bool
}

func (v *ARawDataResourceRawData) GetValue() []byte {
	data := make([]byte, v.GetSize())
	_, err := v.descriptor.ReadAt(data, 8)
	if err != nil {
		return make([]byte, 0)
	}
	return data
}

func (v *ARawDataResourceRawData) GetSize() int {
	size := make([]byte, 8)
	_, err := v.descriptor.ReadAt(size, 0)
	if err != nil {
		return 0
	}
	return int(binary.LittleEndian.Uint64(size))
}

func (v *ARawDataResourceRawData) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *ARawDataResourceRawData) GetSizeInBytes() int {
    return v.descriptor.Len()
}"""
    ]

    generate_and_assert_in("""
    namespace n{
    archive A {
        raw_data_resource : raw_data;
    }
    }""", GoGenerator, *expected_lines)


def test_optional_resource_is_declared_correctly():
    expected_lines = [
        """type ARawDataResourceRawData struct {
    descriptor flatdata.MemoryDescriptor
    IsOptional bool
    IsOpen bool
}

func (v *ARawDataResourceRawData) GetValue() []byte {
	data := make([]byte, v.GetSize())
	_, err := v.descriptor.ReadAt(data, 8)
	if err != nil {
		return make([]byte, 0)
	}
	return data
}

func (v *ARawDataResourceRawData) GetSize() int {
	size := make([]byte, 8)
	_, err := v.descriptor.ReadAt(size, 0)
	if err != nil {
		return 0
	}
	return int(binary.LittleEndian.Uint64(size))
}

func (v *ARawDataResourceRawData) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *ARawDataResourceRawData) GetSizeInBytes() int {
    return v.descriptor.Len()
}""",
    """func OpenAArchive(resource flatdata.ResourceStorage) (*AArchive, error) {
    v := &AArchive{}
    // Initialize resources
	rawDataResourceIsOpen := true
	rawDataResourceMemoryDescriptor, schema, err := resource.GetMemoryDescriptor("raw_data_resource")
	if err != nil {
        log.Println(err)
	    if err.Error() == flatdata.ErrorCantAccessResource {
	        rawDataResourceIsOpen = false
	    } else {
		    return v, err
		}
	}""",
        """v.RawDataResourceRawData = &ARawDataResourceRawData{
        descriptor: rawDataResourceMemoryDescriptor, 
        IsOptional: true,
        IsOpen: rawDataResourceIsOpen,
    }"""
    ]

    generate_and_assert_in("""
    namespace n{
    archive A {
        @optional
        raw_data_resource : raw_data;
    }
    }
    """, GoGenerator, *expected_lines)


def test_check_schema_validation_generation():
    expected_lines = [
        "if vectorResourceIsOpen {",
        "vectorResourceSchema := ",
        "if vectorResourceSchema != schema {"
    ]

    generate_and_assert_in("""
    namespace n{
    struct T {
        f0 : u8 : 3;
    }
    archive A {
        vector_resource : vector< T >;
    }
    }
    """, GoGenerator, *expected_lines)


def test_check_tostring_generation():
    expected_lines = [
        "func (v *IndexType32) ToString() string {",
        "func (v *T) ToString() string {",
        "func (v *AVectorResourceVector) ToString() string {",
        "func (v *ARawDataResourceRawData) ToString() string {",
        "func (v *AMultivectorResourceVector) ToString() string {",
        "func (v *AMultivectorResourceMultivector) ToString() string {",
        "func (v *AInstanceResourceInstance) ToString() string {",
        "func (v *AArchive) ToString() string {"
    ]

    generate_and_assert_in("""namespace xyz{
    struct T {
        f0 : u8 : 3;
    }
    archive A {
        vector_resource : vector< T >;
        raw_data_resource : raw_data;
        multivector_resource : multivector< 32, T >;
        instance_resource : T;
    }
}""", GoGenerator, *expected_lines)


def test_import_added_for_archive_resource():
    generate_and_assert_in("""namespace xyz{
    struct T {
        f0 : u8 : 3;
    }
    archive A {
        vector_resource : vector< T >;
    }
    archive B {
        archive_resource : archive A;
    }
}""", GoGenerator, "\"path/filepath\"")


def test_comments_added_to_generated_sources():
    expected_lines = [
        "// // Simple comment",
        "// /** Builtin type to for MultiVector index */",
        """// /* Complex comment
         // * Multiline
         // */"""
    ]

    generate_and_assert_in("""namespace xyz{
        // Simple comment
        struct A {
            f0 : u8 : 3;
        }
        /* Complex comment
         * Multiline
         */
        struct B {
            f0 : u8 : 3;
        }
        archive Test {
            multivector_resource : multivector< 32, A >;
        }
    }""", GoGenerator, *expected_lines)

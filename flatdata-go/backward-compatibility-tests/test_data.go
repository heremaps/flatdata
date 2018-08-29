/*
Copyright (c) 2017 HERE Europe B.V.
See the LICENSE file in the root of this project for license details.
*/

package backwardcompatibility

import (
	"bytes"

	"github.com/heremaps/flatdata/flatdata-go/flatdata"
)

func createInMemoryResourceStorage() flatdata.ResourceStorage {
	return &flatdata.InMemoryResourceStorage{
		Descriptors: map[string]flatdata.MemoryDescriptor{
			"resource_b":       &flatdata.TestMemoryDescriptor{Array: getVectorPayload()},
			"resource_c":       &flatdata.TestMemoryDescriptor{Array: getMultivectorResourcePayload()},
			"resource_c_index": &flatdata.TestMemoryDescriptor{Array: getMultivectorIndexPayload()},
			"resource_d":       &flatdata.TestMemoryDescriptor{Array: getRawDataPayload()},
			"resource_a":       &flatdata.TestMemoryDescriptor{Array: getInstanceDataPayload()},
		},
		Schemas: map[string]string{
			"resource_b":       getVectorSchema(),
			"resource_c":       getMultivectorSchema(),
			"resource_c_index": getMultivectorSchema(),
			"resource_d":       getRawDataSchema(),
			"resource_a":       getInstanceSchema(),
		},
	}
}

func getVectorPayload() []byte {
	var bb bytes.Buffer
	bb.WriteString("\x14\x00\x00\x00\x00\x00\x00\x00") // Payload size in bytes
	bb.WriteString("\xff\xac\x68\x24\x00\x0b\x00\x00") // Payload
	bb.WriteString("\x00\x00\xff\xac\x68\x24\x00\x0b") // Payload
	bb.WriteString("\x00\x00\x00\x00")                 // Payload
	bb.WriteString("\x00\x00\x00\x00\x00\x00\x00\x00") // Padding
	return []byte(bb.String())
}

func getVectorSchema() string {
	return `namespace backwardcompatibility {
struct SignedStruct
{
    a : i16 : 5;
    b : u32 : 32;
    c : i32 : 7;
    d : u32 : 32;
}
}

namespace backwardcompatibility {
archive BackwardCompatibilityTest
{
    resource_b : vector< .backwardcompatibility.SignedStruct >;
}
}

`
}

func getMultivectorResourcePayload() []byte {
	var bb bytes.Buffer
	bb.WriteString("\x31\x00\x00\x00\x00\x00\x00\x00")             // Payload size in bytes
	bb.WriteString("\x01\xff\xac\x68\x24\x00\x0b\x00\x00\x00\x00") // Payload
	bb.WriteString("\x00\xff\xff\xff\xff\xef\xbe\xad\xde")         // Payload
	bb.WriteString("\x00\xff\xff\xff\xff\xef\xbe\xad\xde")         // Payload
	bb.WriteString("\x01\xff\xac\x68\x24\x00\x0b\x00\x00\x00\x00") // Payload
	bb.WriteString("\x00\xff\xff\xff\xff\xef\xbe\xad\xde")         // Payload
	bb.WriteString("\x00\x00\x00\x00\x00\x00\x00\x00")             // Padding
	return []byte(bb.String())
}

func getMultivectorIndexPayload() []byte {
	var bb bytes.Buffer
	bb.WriteString("\x14\x00\x00\x00\x00\x00\x00\x00") // Payload size in bytes
	bb.WriteString("\x00\x00\x00\x00\x00")             // Payload
	bb.WriteString("\x14\x00\x00\x00\x00")             // Payload
	bb.WriteString("\x14\x00\x00\x00\x00")             // Payload
	bb.WriteString("\x28\x00\x00\x00\x00")             // Payload
	bb.WriteString("\x00\x00\x00\x00\x00\x00\x00\x00") // Padding
	return []byte(bb.String())
}

func getMultivectorSchema() string {
	return `namespace backwardcompatibility {
struct SimpleStruct
{
    a : u32 : 32;
    b : u32 : 32;
}
}

namespace backwardcompatibility {
struct SignedStruct
{
    a : i16 : 5;
    b : u32 : 32;
    c : i32 : 7;
    d : u32 : 32;
}
}

namespace backwardcompatibility {
archive BackwardCompatibilityTest
{
    resource_c : multivector< 33, .backwardcompatibility.SimpleStruct, .backwardcompatibility.SignedStruct >;
}
}

`
}

func getRawDataPayload() []byte {
	var bb bytes.Buffer
	bb.WriteString("\x05\x00\x00\x00\x00\x00\x00\x00") // Payload size in bytes
	bb.WriteString("\xff\xef\xbe\xad\xde")             // Payload
	bb.WriteString("\x00\x00\x00\x00\x00\x00\x00\x00") // Padding
	return []byte(bb.String())
}

func getRawDataSchema() string {
	return `namespace backwardcompatibility {
archive BackwardCompatibilityTest
{
    resource_d : raw_data;
}
}

`
}

func getInstanceDataPayload() []byte {
	var bb bytes.Buffer
	bb.WriteString("\x0a\x00\x00\x00\x00\x00\x00\x00") // Size of payload in bytes
	bb.WriteString("\xff\xac\x68\x24\x00\x0b\x00\x00") // Payload
	bb.WriteString("\x00\x00")                         // Payload
	bb.WriteString("\x00\x00\x00\x00\x00\x00\x00\x00") // Padding
	return []byte(bb.String())
}

func getInstanceSchema() string {
	return `namespace backwardcompatibility {
struct SignedStruct
{
    a : i16 : 5;
    b : u32 : 32;
    c : i32 : 7;
    d : u32 : 32;
}
}

namespace backwardcompatibility {
archive BackwardCompatibilityTest
{
    resource_a : .backwardcompatibility.SignedStruct;
}
}

`
}

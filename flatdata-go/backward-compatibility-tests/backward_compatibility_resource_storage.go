/*
Copyright (c) 2017 HERE Europe B.V.
See the LICENSE file in the root of this project for license details.
*/

package backwardcompatibility

import (
	"errors"

	"github.com/heremaps/flatdata/flatdata-go/flatdata"
)

type byteArrayResourceStorage struct {
}

func (v *byteArrayResourceStorage) GetMemoryDescriptor(name string) (flatdata.MemoryDescriptor, string, error) {
	switch name {
	case "resource_b":
		return &flatdata.TestMemoryDescriptor{Array: getVectorPayload()}, getVectorSchema(), nil
	case "resource_c":
		return &flatdata.TestMemoryDescriptor{Array: getMultivectorResourcePayload()}, getMultivectorSchema(), nil
	case "resource_c_index":
		return &flatdata.TestMemoryDescriptor{Array: getMultivectorIndexPayload()}, getMultivectorSchema(), nil
	case "resource_d":
		return &flatdata.TestMemoryDescriptor{Array: getRawDataPayload()}, getRawDataSchema(), nil
	case "resource_a":
		return &flatdata.TestMemoryDescriptor{Array: getInstanceDataPayload()}, getInstanceSchema(), nil
	default:
		return &flatdata.TestMemoryDescriptor{Array: []byte("")}, "", errors.New(flatdata.ErrorCantAccessResource)
	}
}

func (v *byteArrayResourceStorage) GetBasePath() string {
	return ""
}

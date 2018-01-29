/*
Copyright (c) 2017 HERE Europe B.V.
See the LICENSE file in the root of this project for license details.
*/

package backwardcompatibility

import (
	"github.com/heremaps/flatdata/flatdata-go"
	"github.com/pkg/errors"
)

type byteArrayResourceProvider struct {
}

func (v *byteArrayResourceProvider) GetHandle(name string) (flatdata.ResourceHandle, string, error) {
	switch name {
	case "resource_vector":
		return &byteArrayHandle{Array: getVectorPayload()}, getVectorSchema(), nil
	case "resource_multivector":
		return &byteArrayHandle{Array: getMultivectorResourcePayload()}, getMultivectorSchema(), nil
	case "resource_multivector_index":
		return &byteArrayHandle{Array: getMultivectorIndexPayload()}, getMultivectorSchema(), nil
	case "resource_rawdata":
		return &byteArrayHandle{Array: getRawDataPayload()}, getRawDataSchema(), nil
	case "resource_instance":
		return &byteArrayHandle{Array: getInstanceDataPayload()}, getInstanceSchema(), nil
	default:
		return &byteArrayHandle{Array: []byte("")}, "", errors.New(flatdata.ErrorCantAccessResource)
	}
}

func (v *byteArrayResourceProvider) GetBasePath() string {
	return ""
}

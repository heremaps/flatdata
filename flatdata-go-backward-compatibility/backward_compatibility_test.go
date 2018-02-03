/*
Copyright (c) 2017 HERE Europe B.V.
See the LICENSE file in the root of this project for license details.
*/

package backwardcompatibility

import (
	"log"
	"testing"

	"github.com/stretchr/testify/assert"
)

func init() {
	log.SetFlags(log.LstdFlags | log.Lshortfile)
}

func TestBackwardCompatibilityVector(t *testing.T) {
	arc, err := OpenArchiveArchive(&byteArrayResourceProvider{})

	if assert.NoError(t, err) {
		defer arc.Close()
		assert.Equal(t, 36, arc.ResourceVectorVector.GetSizeInBytes())
		assert.Equal(t, 2, arc.ResourceVectorVector.GetSize())
		assert.Equal(t, int16(-0x1), arc.ResourceVectorVector.Get(0).GetA())
		assert.Equal(t, uint32(0x01234567), arc.ResourceVectorVector.Get(0).GetB())
		assert.Equal(t, int32(-0x28), arc.ResourceVectorVector.Get(0).GetC())
		assert.Equal(t, uint32(0), arc.ResourceVectorVector.Get(0).GetD())
		assert.Equal(t, int16(-0x1), arc.ResourceVectorVector.Get(1).GetA())
		assert.Equal(t, uint32(0x01234567), arc.ResourceVectorVector.Get(1).GetB())
		assert.Equal(t, int32(-0x28), arc.ResourceVectorVector.Get(1).GetC())
		assert.Equal(t, uint32(0), arc.ResourceVectorVector.Get(1).GetD())
	}
}

func TestBackwardCompatibilityMultivector(t *testing.T) {
	arc, err := OpenArchiveArchive(&byteArrayResourceProvider{})

	if assert.NoError(t, err) {
		defer arc.Close()
		assert.Equal(t, 65, arc.ResourceMultivectorMultivector.GetSizeInBytes())
		assert.Equal(t, 4, arc.ResourceMultivectorMultivector.GetSize())
		assert.Equal(t, 2, len(arc.ResourceMultivectorMultivector.Get(0)))
	}
}

func TestBackwardCompatibilityRawData(t *testing.T) {
	arc, err := OpenArchiveArchive(&byteArrayResourceProvider{})

	if assert.NoError(t, err) {
		defer arc.Close()
		assert.Equal(t, 21, arc.ResourceRawdataRawData.GetSizeInBytes())
		assert.Equal(t, 5, arc.ResourceRawdataRawData.GetSize())
		assert.Equal(t, []byte("\xff\xef\xbe\xad\xde"), arc.ResourceRawdataRawData.GetValue())
	}
}

func TestBackwardCompatibilityInstance(t *testing.T) {
	arc, err := OpenArchiveArchive(&byteArrayResourceProvider{})

	if assert.NoError(t, err) {
		defer arc.Close()
		assert.Equal(t, 26, arc.ResourceInstanceInstance.GetSizeInBytes())
		assert.Equal(t, 1, arc.ResourceInstanceInstance.GetSize())
		assert.Equal(t, int16(-0x1), arc.ResourceInstanceInstance.Get().GetA())
		assert.Equal(t, uint32(0x01234567), arc.ResourceInstanceInstance.Get().GetB())
		assert.Equal(t, int32(-0x28), arc.ResourceInstanceInstance.Get().GetC())
		assert.Equal(t, uint32(0), arc.ResourceInstanceInstance.Get().GetD())
	}
}

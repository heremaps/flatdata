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
		v1, ok := arc.ResourceMultivectorMultivector.Get(0)[0].(*SignedStruct)
		if assert.True(t, ok) {
			assert.Equal(t, int16(-0x1), v1.GetA())
			assert.Equal(t, uint32(0x01234567), v1.GetB())
			assert.Equal(t, int32(-0x28), v1.GetC())
			assert.Equal(t, uint32(0), v1.GetD())
		}
		v2, ok := arc.ResourceMultivectorMultivector.Get(0)[1].(*SimpleStruct)
		if assert.True(t, ok) {
			assert.Equal(t, uint32(0xFFFFFFFF), v2.GetA())
			assert.Equal(t, uint32(0xDEADBEEF), v2.GetB())
		}

		assert.Equal(t, 0, len(arc.ResourceMultivectorMultivector.Get(1)))

		assert.Equal(t, 2, len(arc.ResourceMultivectorMultivector.Get(2)))
		v3, ok := arc.ResourceMultivectorMultivector.Get(2)[0].(*SimpleStruct)
		if assert.True(t, ok) {
			assert.Equal(t, uint32(0xFFFFFFFF), v3.GetA())
			assert.Equal(t, uint32(0xDEADBEEF), v3.GetB())
		}
		v4, ok := arc.ResourceMultivectorMultivector.Get(2)[1].(*SignedStruct)
		if assert.True(t, ok) {
			assert.Equal(t, int16(-0x1), v4.GetA())
			assert.Equal(t, uint32(0x01234567), v4.GetB())
			assert.Equal(t, int32(-0x28), v4.GetC())
			assert.Equal(t, uint32(0), v4.GetD())
		}

		assert.Equal(t, 1, len(arc.ResourceMultivectorMultivector.Get(3)))
		v5, ok := arc.ResourceMultivectorMultivector.Get(3)[0].(*SimpleStruct)
		if assert.True(t, ok) {
			assert.Equal(t, uint32(0xFFFFFFFF), v5.GetA())
			assert.Equal(t, uint32(0xDEADBEEF), v5.GetB())
		}
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

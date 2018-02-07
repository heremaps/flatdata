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
	arc, err := OpenBackwardCompatibilityTestArchive(&byteArrayResourceProvider{})

	if assert.NoError(t, err) {
		defer arc.Close()
		assert.Equal(t, 36, arc.ResourceBVector.GetSizeInBytes())
		assert.Equal(t, 2, arc.ResourceBVector.GetSize())

		assert.Equal(t, int16(-0x1), arc.ResourceBVector.Get(0).GetA())
		assert.Equal(t, uint32(0x01234567), arc.ResourceBVector.Get(0).GetB())
		assert.Equal(t, int32(-0x28), arc.ResourceBVector.Get(0).GetC())
		assert.Equal(t, uint32(0), arc.ResourceBVector.Get(0).GetD())

		assert.Equal(t, int16(-0x1), arc.ResourceBVector.Get(1).GetA())
		assert.Equal(t, uint32(0x01234567), arc.ResourceBVector.Get(1).GetB())
		assert.Equal(t, int32(-0x28), arc.ResourceBVector.Get(1).GetC())
		assert.Equal(t, uint32(0), arc.ResourceBVector.Get(1).GetD())
	}
}

func TestBackwardCompatibilityMultivector(t *testing.T) {
	arc, err := OpenBackwardCompatibilityTestArchive(&byteArrayResourceProvider{})

	if assert.NoError(t, err) {
		defer arc.Close()
		assert.Equal(t, 65, arc.ResourceCMultivector.GetSizeInBytes())
		assert.Equal(t, 4, arc.ResourceCMultivector.GetSize())

		assert.Equal(t, 2, len(arc.ResourceCMultivector.Get(0)))
		v1, ok := arc.ResourceCMultivector.Get(0)[0].(*SignedStruct)
		if assert.True(t, ok) {
			assert.Equal(t, int16(-0x1), v1.GetA())
			assert.Equal(t, uint32(0x01234567), v1.GetB())
			assert.Equal(t, int32(-0x28), v1.GetC())
			assert.Equal(t, uint32(0), v1.GetD())
		}
		v2, ok := arc.ResourceCMultivector.Get(0)[1].(*SimpleStruct)
		if assert.True(t, ok) {
			assert.Equal(t, uint32(0xFFFFFFFF), v2.GetA())
			assert.Equal(t, uint32(0xDEADBEEF), v2.GetB())
		}

		assert.Equal(t, 0, len(arc.ResourceCMultivector.Get(1)))

		assert.Equal(t, 2, len(arc.ResourceCMultivector.Get(2)))
		v3, ok := arc.ResourceCMultivector.Get(2)[0].(*SimpleStruct)
		if assert.True(t, ok) {
			assert.Equal(t, uint32(0xFFFFFFFF), v3.GetA())
			assert.Equal(t, uint32(0xDEADBEEF), v3.GetB())
		}
		v4, ok := arc.ResourceCMultivector.Get(2)[1].(*SignedStruct)
		if assert.True(t, ok) {
			assert.Equal(t, int16(-0x1), v4.GetA())
			assert.Equal(t, uint32(0x01234567), v4.GetB())
			assert.Equal(t, int32(-0x28), v4.GetC())
			assert.Equal(t, uint32(0), v4.GetD())
		}

		assert.Equal(t, 1, len(arc.ResourceCMultivector.Get(3)))
		v5, ok := arc.ResourceCMultivector.Get(3)[0].(*SimpleStruct)
		if assert.True(t, ok) {
			assert.Equal(t, uint32(0xFFFFFFFF), v5.GetA())
			assert.Equal(t, uint32(0xDEADBEEF), v5.GetB())
		}
	}
}

func TestBackwardCompatibilityRawData(t *testing.T) {
	arc, err := OpenBackwardCompatibilityTestArchive(&byteArrayResourceProvider{})

	if assert.NoError(t, err) {
		defer arc.Close()
		assert.Equal(t, 21, arc.ResourceDRawData.GetSizeInBytes())
		assert.Equal(t, 5, arc.ResourceDRawData.GetSize())
		assert.Equal(t, []byte("\xff\xef\xbe\xad\xde"), arc.ResourceDRawData.GetValue())
	}
}

func TestBackwardCompatibilityInstance(t *testing.T) {
	arc, err := OpenBackwardCompatibilityTestArchive(&byteArrayResourceProvider{})

	if assert.NoError(t, err) {
		defer arc.Close()
		assert.Equal(t, 26, arc.ResourceAInstance.GetSizeInBytes())
		assert.Equal(t, 1, arc.ResourceAInstance.GetSize())

		assert.Equal(t, int16(-0x1), arc.ResourceAInstance.Get().GetA())
		assert.Equal(t, uint32(0x01234567), arc.ResourceAInstance.Get().GetB())
		assert.Equal(t, int32(-0x28), arc.ResourceAInstance.Get().GetC())
		assert.Equal(t, uint32(0), arc.ResourceAInstance.Get().GetD())
	}
}

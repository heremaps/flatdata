/*
Copyright (c) 2017 HERE Europe B.V.
See the LICENSE file in the root of this project for license details.
*/

package backwardcompatibility

import (
	"bytes"
	"encoding/json"
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
		assertSigned(t, arc.ResourceBVector.Get(0))
		assertSigned(t, arc.ResourceBVector.Get(1))
		assert.True(t, validJSON(arc.ResourceBVector.ToString()))
	}
}

func TestBackwardCompatibilityMultivector(t *testing.T) {
	arc, err := OpenBackwardCompatibilityTestArchive(&byteArrayResourceProvider{})
	if assert.NoError(t, err) {
		defer arc.Close()
		assert.Equal(t, 65, arc.ResourceCMultivector.GetSizeInBytes())
		assert.Equal(t, 4, arc.ResourceCMultivector.GetSize())
		assert.True(t, validJSON(arc.ResourceCMultivector.ToString()))

		assert.Equal(t, 2, len(arc.ResourceCMultivector.Get(0)))
		v1, ok := arc.ResourceCMultivector.Get(0)[0].(*SignedStruct)
		if assert.True(t, ok) {
			assertSigned(t, v1)
		}
		v2, ok := arc.ResourceCMultivector.Get(0)[1].(*SimpleStruct)
		if assert.True(t, ok) {
			assertSimple(t, v2)
		}

		assert.Equal(t, 0, len(arc.ResourceCMultivector.Get(1)))

		assert.Equal(t, 2, len(arc.ResourceCMultivector.Get(2)))
		v3, ok := arc.ResourceCMultivector.Get(2)[0].(*SimpleStruct)
		if assert.True(t, ok) {
			assertSimple(t, v3)
		}
		v4, ok := arc.ResourceCMultivector.Get(2)[1].(*SignedStruct)
		if assert.True(t, ok) {
			assertSigned(t, v4)
		}

		assert.Equal(t, 1, len(arc.ResourceCMultivector.Get(3)))
		v5, ok := arc.ResourceCMultivector.Get(3)[0].(*SimpleStruct)
		if assert.True(t, ok) {
			assertSimple(t, v5)
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
		assert.True(t, validJSON(arc.ResourceDRawData.ToString()))
	}
}

func TestBackwardCompatibilityInstance(t *testing.T) {
	arc, err := OpenBackwardCompatibilityTestArchive(&byteArrayResourceProvider{})
	if assert.NoError(t, err) {
		defer arc.Close()
		assert.Equal(t, 26, arc.ResourceAInstance.GetSizeInBytes())
		assert.Equal(t, 1, arc.ResourceAInstance.GetSize())
		assertSigned(t, arc.ResourceAInstance.Get())
		assert.True(t, validJSON(arc.ResourceAInstance.ToString()))
	}
}

func assertSigned(t *testing.T, str *SignedStruct) {
	assert.Equal(t, int16(-0x1), str.GetA())
	assert.Equal(t, uint32(0x01234567), str.GetB())
	assert.Equal(t, int32(-0x28), str.GetC())
	assert.Equal(t, uint32(0), str.GetD())
}

func assertSimple(t *testing.T, str *SimpleStruct) {
	assert.Equal(t, uint32(0xFFFFFFFF), str.GetA())
	assert.Equal(t, uint32(0xDEADBEEF), str.GetB())
}

func validJSON(s string) bool {
	out := bytes.Buffer{}
	err := json.Indent(&out, []byte(s), "", "\t")
	if err != nil {
		log.Println(err)
		return false
	}
	return true
}
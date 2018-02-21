/*
Copyright (c) 2017 HERE Europe B.V.
See the LICENSE file in the root of this project for license details.
*/

package flatdata

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestOpenNonExistingFileResourceShouldReturnError(t *testing.T) {
	r := NewFileResourceStorage("unexisted_archive")
	_, _, err := r.GetMemoryDescriptor("unexisted_resource")
	assert.EqualError(t, err, ErrorCantAccessResource, "Should return error for an unexisted resource")
}

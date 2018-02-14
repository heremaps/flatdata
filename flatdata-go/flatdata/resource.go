/*
Copyright (c) 2017 HERE Europe B.V.
See the LICENSE file in the root of this project for license details.
*/

package flatdata

import (
	"errors"
	"io/ioutil"
	"path/filepath"

	"golang.org/x/exp/mmap"
)

const (
	flatdataResourcePaddingBytes = 8
	flatdataSizeOffsetBytes = 8
	// ErrorInvalidResource returns in case of size of resource is smaller than minimal
	ErrorInvalidResource = "invalid flatdata resource"
	// ErrorCantAccessResource returns if resource wasn't been memory-mapped
	ErrorCantAccessResource = "can't open flatdata resource"
	// ErrorCantOpenSchemaForResource returns if schema for resource wasn't open
	ErrorCantOpenSchemaForResource = "can't open schema for resource"
	// ErrorSchemaEmpty returns in case when schema is empty
	ErrorSchemaEmpty = "schema for resource is empty"
	// ErrorSchemaDidntMatch returns if provided schema is not equal to schema in generated sources
	ErrorSchemaDidntMatch = "schemas didn't match"
)

// ResourceProvider represents abstraction for getting resource handle
type ResourceProvider interface {
	GetHandle(name string) (ResourceHandle, string, error)
	GetBasePath() string
}

// NewFileResourceProvider - constructor for FileResourceProvider
func NewFileResourceProvider(path string) *FileResourceProvider {
	dir := filepath.Dir(path)
	return &FileResourceProvider{basePath: dir}
}

// FileResourceProvider implements ResourceProvider interface for memory-mapped file
type FileResourceProvider struct {
	basePath string
}

// GetHandle returns handle for specified resource
func (r *FileResourceProvider) GetHandle(name string) (ResourceHandle, string, error) {
	path := filepath.Join(r.basePath, name)

	handle, err := mmap.Open(path)
	if err != nil {
		return nil, "", errors.New(ErrorCantAccessResource)
	}
	if handle.Len() < (flatdataResourcePaddingBytes + flatdataSizeOffsetBytes) {
		return nil, "", errors.New(ErrorInvalidResource)
	}

	bytes, err := ioutil.ReadFile(path + ".schema")
	if err != nil {
		return nil, "", errors.New(ErrorCantOpenSchemaForResource)
	}
	if len(bytes) == 0 {
		return nil, "", errors.New(ErrorSchemaEmpty)
	}

	return handle, string(bytes), nil
}

// GetBasePath returns base path to opened archive
func (r *FileResourceProvider) GetBasePath() string {
	return r.basePath
}

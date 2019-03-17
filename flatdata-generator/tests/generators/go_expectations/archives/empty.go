type AArchive struct {
    IsOptional bool
    IsOpen bool
}

func (v *AArchive) Close() {
}

func (v *AArchive) GetSizeInBytes() int {
    var size int
    return size
}

func (v *AArchive) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "A", "container_type": "Archive", "size_in_bytes": %d, "resources": [`, v.GetSizeInBytes()))
    buffer.WriteString("]}")
	return buffer.String()
}

func OpenAArchive(resource flatdata.ResourceStorage) (*AArchive, error) {
    v := &AArchive{}
    // Initialize resources
	// Add resources to archive
	return v, nil
}
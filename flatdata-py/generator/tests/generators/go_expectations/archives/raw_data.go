type ADataRawData struct {
    descriptor flatdata.MemoryDescriptor
    IsOptional bool
    IsOpen bool
}

func (v *ADataRawData) GetValue() []byte {
	data := make([]byte, v.GetSize())
	_, err := v.descriptor.ReadAt(data, 8)
	if err != nil {
		return make([]byte, 0)
	}
	return data
}

func (v *ADataRawData) GetSize() int {
	size := make([]byte, 8)
	_, err := v.descriptor.ReadAt(size, 0)
	if err != nil {
		return 0
	}
	return int(binary.LittleEndian.Uint64(size))
}

func (v *ADataRawData) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *ADataRawData) GetSizeInBytes() int {
    return v.descriptor.Len()
}

func (v *ADataRawData) ToString() string {
    return fmt.Sprintf(`{"container_type": "RawData", "size": %d, "size_in_bytes": %d, "element_types": []}`, v.GetSize(), v.GetSizeInBytes())
}
    
type AOptionalDataRawData struct {
    descriptor flatdata.MemoryDescriptor
    IsOptional bool
    IsOpen bool
}

func (v *AOptionalDataRawData) GetValue() []byte {
	data := make([]byte, v.GetSize())
	_, err := v.descriptor.ReadAt(data, 8)
	if err != nil {
		return make([]byte, 0)
	}
	return data
}

func (v *AOptionalDataRawData) GetSize() int {
	size := make([]byte, 8)
	_, err := v.descriptor.ReadAt(size, 0)
	if err != nil {
		return 0
	}
	return int(binary.LittleEndian.Uint64(size))
}

func (v *AOptionalDataRawData) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *AOptionalDataRawData) GetSizeInBytes() int {
    return v.descriptor.Len()
}

func (v *AOptionalDataRawData) ToString() string {
    return fmt.Sprintf(`{"container_type": "RawData", "size": %d, "size_in_bytes": %d, "element_types": []}`, v.GetSize(), v.GetSizeInBytes())
}
    
type AArchive struct {
    IsOptional bool
    IsOpen bool
    DataRawData *ADataRawData
    OptionalDataRawData *AOptionalDataRawData
}

func (v *AArchive) Close() {
    if v.DataRawData.IsOpen {
        v.DataRawData.Close()
    }
    if v.OptionalDataRawData.IsOpen {
        v.OptionalDataRawData.Close()
    }
}

func (v *AArchive) GetSizeInBytes() int {
    var size int
    if v.DataRawData.IsOpen {
        size += v.DataRawData.GetSizeInBytes()
    }
    if v.OptionalDataRawData.IsOpen {
        size += v.OptionalDataRawData.GetSizeInBytes()
    }
    return size
}

func (v *AArchive) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "A", "container_type": "Archive", "size_in_bytes": %d, "resources": [`, v.GetSizeInBytes()))
    buffer.WriteString(v.DataRawData.ToString())
    buffer.WriteString(",")
      
    buffer.WriteString(v.OptionalDataRawData.ToString())
      
    buffer.WriteString("]}")
	return buffer.String()
}

func OpenAArchive(resource flatdata.ResourceStorage) (*AArchive, error) {
    v := &AArchive{}
    // Initialize resources
	dataIsOpen := true
	dataMemoryDescriptor, schema, err := resource.GetMemoryDescriptor("data")
	if err != nil {
        log.Println(err)
	    if err.Error() == flatdata.ErrorCantAccessResource {
	        dataIsOpen = false
	    } else {
		    return v, err
		}
	}
	if dataIsOpen {
	    dataSchema := `namespace n {
archive A
{
    data : raw_data;
}
}

`
	    if dataSchema != schema {
	        log.Println(fmt.Sprintf("Schemas didn't match, expected: \n%s\n, actual: \n%s\n", dataSchema, schema))
	        return v, errors.New(flatdata.ErrorSchemaDidntMatch)
	    }
	}
	optionalDataIsOpen := true
	optionalDataMemoryDescriptor, schema, err := resource.GetMemoryDescriptor("optional_data")
	if err != nil {
        log.Println(err)
	    if err.Error() == flatdata.ErrorCantAccessResource {
	        optionalDataIsOpen = false
	    } else {
		    return v, err
		}
	}
	if optionalDataIsOpen {
	    optionalDataSchema := `namespace n {
archive A
{
    @optional
    optional_data : raw_data;
}
}

`
	    if optionalDataSchema != schema {
	        log.Println(fmt.Sprintf("Schemas didn't match, expected: \n%s\n, actual: \n%s\n", optionalDataSchema, schema))
	        return v, errors.New(flatdata.ErrorSchemaDidntMatch)
	    }
	}
	// Add resources to archive
    v.DataRawData = &ADataRawData{
        descriptor: dataMemoryDescriptor,
        IsOptional: false,
        IsOpen: dataIsOpen,
    }
    v.OptionalDataRawData = &AOptionalDataRawData{
        descriptor: optionalDataMemoryDescriptor,
        IsOptional: true,
        IsOpen: optionalDataIsOpen,
    }
	return v, nil
}



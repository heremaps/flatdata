type XPayloadRawData struct {
    descriptor flatdata.MemoryDescriptor
    IsOptional bool
    IsOpen bool
}

func (v *XPayloadRawData) GetValue() []byte {
	data := make([]byte, v.GetSize())
	_, err := v.descriptor.ReadAt(data, 8)
	if err != nil {
		return make([]byte, 0)
	}
	return data
}

func (v *XPayloadRawData) GetSize() int {
	size := make([]byte, 8)
	_, err := v.descriptor.ReadAt(size, 0)
	if err != nil {
		return 0
	}
	return int(binary.LittleEndian.Uint64(size))
}

func (v *XPayloadRawData) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *XPayloadRawData) GetSizeInBytes() int {
    return v.descriptor.Len()
}

func (v *XPayloadRawData) ToString() string {
    return fmt.Sprintf(`{"container_type": "RawData", "size": %d, "size_in_bytes": %d, "element_types": []}`, v.GetSize(), v.GetSizeInBytes())
}
    
type XArchive struct {
    IsOptional bool
    IsOpen bool
    PayloadRawData *XPayloadRawData
}

func (v *XArchive) Close() {
    if v.PayloadRawData.IsOpen {
        v.PayloadRawData.Close()
    }
}

func (v *XArchive) GetSizeInBytes() int {
    var size int
    if v.PayloadRawData.IsOpen {
        size += v.PayloadRawData.GetSizeInBytes()
    }
    return size
}

func (v *XArchive) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "X", "container_type": "Archive", "size_in_bytes": %d, "resources": [`, v.GetSizeInBytes()))
    buffer.WriteString(v.PayloadRawData.ToString())
    buffer.WriteString(",")
      
    buffer.WriteString("]}")
	return buffer.String()
}

func OpenXArchive(resource flatdata.ResourceStorage) (*XArchive, error) {
    v := &XArchive{}
    // Initialize resources
	payloadIsOpen := true
	payloadMemoryDescriptor, schema, err := resource.GetMemoryDescriptor("payload")
	if err != nil {
        log.Println(err)
	    if err.Error() == flatdata.ErrorCantAccessResource {
	        payloadIsOpen = false
	    } else {
		    return v, err
		}
	}
	if payloadIsOpen {
	    payloadSchema := `namespace n {
archive X
{
    payload : raw_data;
}
}

`
	    if payloadSchema != schema {
	        log.Println(fmt.Sprintf("Schemas didn't match, expected: \n%s\n, actual: \n%s\n", payloadSchema, schema))
	        return v, errors.New(flatdata.ErrorSchemaDidntMatch)
	    }
	}
	// Add resources to archive
    v.PayloadRawData = &XPayloadRawData{
        descriptor: payloadMemoryDescriptor,
        IsOptional: false,
        IsOpen: payloadIsOpen,
    }
	return v, nil
}


type AArchive struct {
    IsOptional bool
    IsOpen bool
    DataArchive *DataArchive
    OptionalDataArchive *OptionalDataArchive
}

func (v *AArchive) Close() {
    if v.DataArchive.IsOpen {
        v.DataArchive.Close()
    }
    if v.OptionalDataArchive.IsOpen {
        v.OptionalDataArchive.Close()
    }
}

func (v *AArchive) GetSizeInBytes() int {
    var size int
    if v.DataArchive.IsOpen {
        size += v.DataArchive.GetSizeInBytes()
    }
    if v.OptionalDataArchive.IsOpen {
        size += v.OptionalDataArchive.GetSizeInBytes()
    }
    return size
}

func (v *AArchive) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "A", "container_type": "Archive", "size_in_bytes": %d, "resources": [`, v.GetSizeInBytes()))
    buffer.WriteString(v.DataArchive.ToString())
    buffer.WriteString(",")
      
    buffer.WriteString(v.OptionalDataArchive.ToString())
      
    buffer.WriteString("]}")
	return buffer.String()
}

func OpenAArchive(resource flatdata.ResourceStorage) (*AArchive, error) {
    v := &AArchive{}
    // Initialize resources
	dataArchive, err := OpenDataArchive(flatdata.NewFileResourceStorage(filepath.Join(resource.GetBasePath(), "data/X.archive")))
	dataArchive.IsOptional = false
	dataArchive.IsOpen = true
	if err != nil {
	    log.Println(err)
	    if err.Error() == flatdata.ErrorCantAccessResource {
	        dataArchive.IsOpen = false
	    } else {
		    return v, err
		}
	}
	optionalDataArchive, err := OpenOptionalDataArchive(flatdata.NewFileResourceStorage(filepath.Join(resource.GetBasePath(), "optional_data/X.archive")))
	optionalDataArchive.IsOptional = true
	optionalDataArchive.IsOpen = true
	if err != nil {
	    log.Println(err)
	    if err.Error() == flatdata.ErrorCantAccessResource {
	        optionalDataArchive.IsOpen = false
	    } else {
		    return v, err
		}
	}
	// Add resources to archive
    v.DataArchive = dataArchive
    v.OptionalDataArchive = optionalDataArchive
	return v, nil
}



const (
    flatdataOffsetSizeInBytes uint = 8
    flatdataPaddingSizeInBytes uint = 8
    sSizeInBytes = 8
)

type S struct {
    descriptor flatdata.MemoryDescriptor
	position int
}

func (v *S) GetX() uint64 {
    elementSizeInBits := uint(64)
    elementOffset := uint(0)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, false)
    return uint64(result)
}
    
    
func (v *S) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "S", "position": %d, "attributes": {`, v.position))
    if v.descriptor != nil {
        buffer.WriteString(fmt.Sprintf(`"x": %v`, v.GetX()))
	}
    buffer.WriteString("}}")
	return buffer.String()
}
    
type ADataInstance struct {
    descriptor flatdata.MemoryDescriptor
    IsOptional bool
    IsOpen bool
}

func (v *ADataInstance) Get() *S {
	return &S{
	    descriptor: v.descriptor,
        position: int(flatdataOffsetSizeInBytes),
	}
}

func (v *ADataInstance) GetSize() int {
	return 1
}

func (v *ADataInstance) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *ADataInstance) GetSizeInBytes() int {
    return v.descriptor.Len()
}

func (v *ADataInstance) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"container_type": "Instance", "size": %d, "size_in_bytes": %d, `, v.GetSize(), v.GetSizeInBytes()))
    buffer.WriteString(`"element_types": [{ "name": "S", "attributes": [`)
    buffer.WriteString(`{"name": "x", "offset": 0, "width": 64, "is_signed": false}`)
   
    buffer.WriteString("]}]}")
	return buffer.String()
}
    
type AOptionalDataInstance struct {
    descriptor flatdata.MemoryDescriptor
    IsOptional bool
    IsOpen bool
}

func (v *AOptionalDataInstance) Get() *S {
	return &S{
	    descriptor: v.descriptor,
        position: int(flatdataOffsetSizeInBytes),
	}
}

func (v *AOptionalDataInstance) GetSize() int {
	return 1
}

func (v *AOptionalDataInstance) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *AOptionalDataInstance) GetSizeInBytes() int {
    return v.descriptor.Len()
}

func (v *AOptionalDataInstance) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"container_type": "Instance", "size": %d, "size_in_bytes": %d, `, v.GetSize(), v.GetSizeInBytes()))
    buffer.WriteString(`"element_types": [{ "name": "S", "attributes": [`)
    buffer.WriteString(`{"name": "x", "offset": 0, "width": 64, "is_signed": false}`)
   
    buffer.WriteString("]}]}")
	return buffer.String()
}
    
type AArchive struct {
    IsOptional bool
    IsOpen bool
    DataInstance *ADataInstance
    OptionalDataInstance *AOptionalDataInstance
}

func (v *AArchive) Close() {
    if v.DataInstance.IsOpen {
        v.DataInstance.Close()
    }
    if v.OptionalDataInstance.IsOpen {
        v.OptionalDataInstance.Close()
    }
}

func (v *AArchive) GetSizeInBytes() int {
    var size int
    if v.DataInstance.IsOpen {
        size += v.DataInstance.GetSizeInBytes()
    }
    if v.OptionalDataInstance.IsOpen {
        size += v.OptionalDataInstance.GetSizeInBytes()
    }
    return size
}

func (v *AArchive) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "A", "container_type": "Archive", "size_in_bytes": %d, "resources": [`, v.GetSizeInBytes()))
    buffer.WriteString(v.DataInstance.ToString())
    buffer.WriteString(",")
      
    buffer.WriteString(v.OptionalDataInstance.ToString())
      
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
struct S
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    data : .n.S;
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
struct S
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    @optional
    optional_data : .n.S;
}
}

`
	    if optionalDataSchema != schema {
	        log.Println(fmt.Sprintf("Schemas didn't match, expected: \n%s\n, actual: \n%s\n", optionalDataSchema, schema))
	        return v, errors.New(flatdata.ErrorSchemaDidntMatch)
	    }
	}
	// Add resources to archive
    v.DataInstance = &ADataInstance {
        descriptor: dataMemoryDescriptor,
        IsOptional: false,
        IsOpen: dataIsOpen,
    }
    v.OptionalDataInstance = &AOptionalDataInstance {
        descriptor: optionalDataMemoryDescriptor,
        IsOptional: true,
        IsOpen: optionalDataIsOpen,
    }
	return v, nil
}



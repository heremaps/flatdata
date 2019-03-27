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
    
    
type ADataVector struct {
    descriptor flatdata.MemoryDescriptor
    IsOptional bool
    IsOpen bool
}

func (v *ADataVector) Get(i int) *S {
	return &S{
		descriptor: v.descriptor,
		position: int(uint(i*sSizeInBytes) + flatdataOffsetSizeInBytes),
	}
}

func (v *ADataVector) GetSize() int {
	size := make([]byte, 8)
	_, err := v.descriptor.ReadAt(size, 0)
	if err != nil {
		return 0
	}
	intSize := binary.LittleEndian.Uint64(size)
	return int(intSize) / sSizeInBytes
}

func (v *ADataVector) GetSlice(start, end, step int) []*S {
	var result []*S	
    for start <= end {
		result = append(result, &S{
			descriptor: v.descriptor,
			position: int(uint(start*sSizeInBytes) + flatdataOffsetSizeInBytes),
	    })
		start += step
	}
	return result
}

func (v *ADataVector) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *ADataVector) GetSizeInBytes() int {
    return v.descriptor.Len()
}

func (v *ADataVector) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"container_type": "Vector", "size": %d, "size_in_bytes": %d, `, v.GetSize(), v.GetSizeInBytes()))
    buffer.WriteString(`"element_types": [{ "name": "S", "attributes": [`)
    buffer.WriteString(`{"name": "x", "offset": 0, "width": 64, "is_signed": false}`)
   
    buffer.WriteString("]}]}")
	return buffer.String()
}

    
type AOptionalDataVector struct {
    descriptor flatdata.MemoryDescriptor
    IsOptional bool
    IsOpen bool
}

func (v *AOptionalDataVector) Get(i int) *S {
	return &S{
		descriptor: v.descriptor,
		position: int(uint(i*sSizeInBytes) + flatdataOffsetSizeInBytes),
	}
}

func (v *AOptionalDataVector) GetSize() int {
	size := make([]byte, 8)
	_, err := v.descriptor.ReadAt(size, 0)
	if err != nil {
		return 0
	}
	intSize := binary.LittleEndian.Uint64(size)
	return int(intSize) / sSizeInBytes
}

func (v *AOptionalDataVector) GetSlice(start, end, step int) []*S {
	var result []*S	
    for start <= end {
		result = append(result, &S{
			descriptor: v.descriptor,
			position: int(uint(start*sSizeInBytes) + flatdataOffsetSizeInBytes),
	    })
		start += step
	}
	return result
}

func (v *AOptionalDataVector) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *AOptionalDataVector) GetSizeInBytes() int {
    return v.descriptor.Len()
}

func (v *AOptionalDataVector) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"container_type": "Vector", "size": %d, "size_in_bytes": %d, `, v.GetSize(), v.GetSizeInBytes()))
    buffer.WriteString(`"element_types": [{ "name": "S", "attributes": [`)
    buffer.WriteString(`{"name": "x", "offset": 0, "width": 64, "is_signed": false}`)
   
    buffer.WriteString("]}]}")
	return buffer.String()
}

type AArchive struct {
    IsOptional bool
    IsOpen bool
    DataVector *ADataVector
    OptionalDataVector *AOptionalDataVector
}

func (v *AArchive) Close() {
    if v.DataVector.IsOpen {
        v.DataVector.Close()
    }
    if v.OptionalDataVector.IsOpen {
        v.OptionalDataVector.Close()
    }
}

func (v *AArchive) GetSizeInBytes() int {
    var size int
    if v.DataVector.IsOpen {
        size += v.DataVector.GetSizeInBytes()
    }
    if v.OptionalDataVector.IsOpen {
        size += v.OptionalDataVector.GetSizeInBytes()
    }
    return size
}

func (v *AArchive) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "A", "container_type": "Archive", "size_in_bytes": %d, "resources": [`, v.GetSizeInBytes()))
    buffer.WriteString(v.DataVector.ToString())
    buffer.WriteString(",")
      
    buffer.WriteString(v.OptionalDataVector.ToString())
      
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
    data : vector< .n.S >;
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
    optional_data : vector< .n.S >;
}
}

`
	    if optionalDataSchema != schema {
	        log.Println(fmt.Sprintf("Schemas didn't match, expected: \n%s\n, actual: \n%s\n", optionalDataSchema, schema))
	        return v, errors.New(flatdata.ErrorSchemaDidntMatch)
	    }
	}
	// Add resources to archive
    v.DataVector = &ADataVector{
        descriptor: dataMemoryDescriptor,
        IsOptional: false,
        IsOpen: dataIsOpen,
    }
    v.OptionalDataVector = &AOptionalDataVector{
        descriptor: optionalDataMemoryDescriptor,
        IsOptional: true,
        IsOpen: optionalDataIsOpen,
    }
	return v, nil
}



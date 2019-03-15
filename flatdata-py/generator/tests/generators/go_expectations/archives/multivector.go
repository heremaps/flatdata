const (
    flatdataOffsetSizeInBytes uint = 8
    flatdataPaddingSizeInBytes uint = 8
    sSizeInBytes = 8
    tSizeInBytes = 8
    indexType8SizeInBytes = 1
    indexType16SizeInBytes = 2
    indexType64SizeInBytes = 8
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
    

type T struct {
    descriptor flatdata.MemoryDescriptor
	position int
}

func (v *T) GetX() uint64 {
    elementSizeInBits := uint(64)
    elementOffset := uint(0)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, false)
    return uint64(result)
}
    
    
func (v *T) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "T", "position": %d, "attributes": {`, v.position))
    if v.descriptor != nil {
        buffer.WriteString(fmt.Sprintf(`"x": %v`, v.GetX()))
	}
    buffer.WriteString("}}")
	return buffer.String()
}
    
// /** Builtin type to for MultiVector index */
type IndexType8 struct {
    descriptor flatdata.MemoryDescriptor
	position int
}

func (v *IndexType8) GetValue() uint64 {
    elementSizeInBits := uint(8)
    elementOffset := uint(0)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, false)
    return uint64(result)
}
    
    
func (v *IndexType8) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "IndexType8", "position": %d, "attributes": {`, v.position))
    if v.descriptor != nil {
        buffer.WriteString(fmt.Sprintf(`"value": %v`, v.GetValue()))
	}
    buffer.WriteString("}}")
	return buffer.String()
}
    
// /** Builtin type to for MultiVector index */
type IndexType16 struct {
    descriptor flatdata.MemoryDescriptor
	position int
}

func (v *IndexType16) GetValue() uint64 {
    elementSizeInBits := uint(16)
    elementOffset := uint(0)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, false)
    return uint64(result)
}
    
    
func (v *IndexType16) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "IndexType16", "position": %d, "attributes": {`, v.position))
    if v.descriptor != nil {
        buffer.WriteString(fmt.Sprintf(`"value": %v`, v.GetValue()))
	}
    buffer.WriteString("}}")
	return buffer.String()
}
    
// /** Builtin type to for MultiVector index */
type IndexType64 struct {
    descriptor flatdata.MemoryDescriptor
	position int
}

func (v *IndexType64) GetValue() uint64 {
    elementSizeInBits := uint(64)
    elementOffset := uint(0)
    result := flatdata.Read(v.descriptor, (uint(v.position)*8)+elementOffset, elementSizeInBits, false)
    return uint64(result)
}
    
    
func (v *IndexType64) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "IndexType64", "position": %d, "attributes": {`, v.position))
    if v.descriptor != nil {
        buffer.WriteString(fmt.Sprintf(`"value": %v`, v.GetValue()))
	}
    buffer.WriteString("}}")
	return buffer.String()
}
    
    
type ADataVector struct {
    descriptor flatdata.MemoryDescriptor
    IsOptional bool
    IsOpen bool
}

func (v *ADataVector) Get(i int) *IndexType8 {
	return &IndexType8{
		descriptor: v.descriptor,
		position: int(uint(i*indexType8SizeInBytes) + flatdataOffsetSizeInBytes),
	}
}

func (v *ADataVector) GetSize() int {
	size := make([]byte, 8)
	_, err := v.descriptor.ReadAt(size, 0)
	if err != nil {
		return 0
	}
	intSize := binary.LittleEndian.Uint64(size)
	return int(intSize) / indexType8SizeInBytes
}

func (v *ADataVector) GetSlice(start, end, step int) []*IndexType8 {
	var result []*IndexType8	
    for start <= end {
		result = append(result, &IndexType8{
			descriptor: v.descriptor,
			position: int(uint(start*indexType8SizeInBytes) + flatdataOffsetSizeInBytes),
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
    buffer.WriteString(`"element_types": [{ "name": "IndexType8", "attributes": [`)
    buffer.WriteString(`{"name": "value", "offset": 0, "width": 8, "is_signed": false}`)
   
    buffer.WriteString("]}]}")
	return buffer.String()
}


type ADataMultivector struct {
    descriptor flatdata.MemoryDescriptor
    index      *ADataVector
	types      map[int]interface{}
    IsOptional bool
    IsOpen bool
}

func (v *ADataMultivector) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *ADataMultivector) GetSize() int {
	return v.index.GetSize()
}

func (v *ADataMultivector) GetSizeInBytes() int {
    return v.descriptor.Len()
}

func (v *ADataMultivector) getBucketOffset(i int) int {
    if i == v.index.GetSize() {
		return v.descriptor.Len() - int(flatdataPaddingSizeInBytes)
	} 
	return int(v.index.Get(i).GetValue()) + int(flatdataOffsetSizeInBytes)
}

func (v *ADataMultivector) Get(i int) []interface{} {
    offset := v.getBucketOffset(i)
	nextOffset := v.getBucketOffset(i + 1)
	var result []interface{}

	for offset < nextOffset {
	    elementType := flatdata.Read(v.descriptor, uint(offset*8), 8, false)
		offset++
		abstractElement, ok := v.types[elementType]
		if !ok {
			//TODO: How to process case, then type of element is not found?
			log.Println("Can't get type of element")
		}
    
		switch element := abstractElement.(type) {
		case *S:
			element.position = offset
			result = append(result, element)
			offset += sSizeInBytes
		case *T:
			element.position = offset
			result = append(result, element)
			offset += tSizeInBytes
		default:
			//TODO: How to react in case if it's impossible to cast?
			log.Println("Can't cast element. Type is unknown...")
		}
	}
	
	return result
}

func (v *ADataMultivector) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"container_type": "Multivector", "size": %d, "size_in_bytes": %d, "element_types": [`, v.GetSize(), v.GetSizeInBytes()))
    buffer.WriteString(`{"name": "S", "attributes": [`)
    buffer.WriteString(`{"name": "x", "offset": 0, "width": 64, "is_signed": false}`)
  
    buffer.WriteString("]}")
    buffer.WriteString(",")
  
    buffer.WriteString(`{"name": "T", "attributes": [`)
    buffer.WriteString(`{"name": "x", "offset": 0, "width": 64, "is_signed": false}`)
  
    buffer.WriteString("]}")
  
    buffer.WriteString(`], "index_type": {"name": "IndexType8", "attributes": [`)
    buffer.WriteString(`{"name": "value", "offset": 0, "width": 8, "is_signed": false}`)
 
    buffer.WriteString("]}}")
	return buffer.String()
}
    
    
type AOptionalDataVector struct {
    descriptor flatdata.MemoryDescriptor
    IsOptional bool
    IsOpen bool
}

func (v *AOptionalDataVector) Get(i int) *IndexType16 {
	return &IndexType16{
		descriptor: v.descriptor,
		position: int(uint(i*indexType16SizeInBytes) + flatdataOffsetSizeInBytes),
	}
}

func (v *AOptionalDataVector) GetSize() int {
	size := make([]byte, 8)
	_, err := v.descriptor.ReadAt(size, 0)
	if err != nil {
		return 0
	}
	intSize := binary.LittleEndian.Uint64(size)
	return int(intSize) / indexType16SizeInBytes
}

func (v *AOptionalDataVector) GetSlice(start, end, step int) []*IndexType16 {
	var result []*IndexType16	
    for start <= end {
		result = append(result, &IndexType16{
			descriptor: v.descriptor,
			position: int(uint(start*indexType16SizeInBytes) + flatdataOffsetSizeInBytes),
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
    buffer.WriteString(`"element_types": [{ "name": "IndexType16", "attributes": [`)
    buffer.WriteString(`{"name": "value", "offset": 0, "width": 16, "is_signed": false}`)
   
    buffer.WriteString("]}]}")
	return buffer.String()
}


type AOptionalDataMultivector struct {
    descriptor flatdata.MemoryDescriptor
    index      *AOptionalDataVector
	types      map[int]interface{}
    IsOptional bool
    IsOpen bool
}

func (v *AOptionalDataMultivector) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *AOptionalDataMultivector) GetSize() int {
	return v.index.GetSize()
}

func (v *AOptionalDataMultivector) GetSizeInBytes() int {
    return v.descriptor.Len()
}

func (v *AOptionalDataMultivector) getBucketOffset(i int) int {
    if i == v.index.GetSize() {
		return v.descriptor.Len() - int(flatdataPaddingSizeInBytes)
	} 
	return int(v.index.Get(i).GetValue()) + int(flatdataOffsetSizeInBytes)
}

func (v *AOptionalDataMultivector) Get(i int) []interface{} {
    offset := v.getBucketOffset(i)
	nextOffset := v.getBucketOffset(i + 1)
	var result []interface{}

	for offset < nextOffset {
	    elementType := flatdata.Read(v.descriptor, uint(offset*8), 8, false)
		offset++
		abstractElement, ok := v.types[elementType]
		if !ok {
			//TODO: How to process case, then type of element is not found?
			log.Println("Can't get type of element")
		}
    
		switch element := abstractElement.(type) {
		case *S:
			element.position = offset
			result = append(result, element)
			offset += sSizeInBytes
		case *T:
			element.position = offset
			result = append(result, element)
			offset += tSizeInBytes
		default:
			//TODO: How to react in case if it's impossible to cast?
			log.Println("Can't cast element. Type is unknown...")
		}
	}
	
	return result
}

func (v *AOptionalDataMultivector) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"container_type": "Multivector", "size": %d, "size_in_bytes": %d, "element_types": [`, v.GetSize(), v.GetSizeInBytes()))
    buffer.WriteString(`{"name": "S", "attributes": [`)
    buffer.WriteString(`{"name": "x", "offset": 0, "width": 64, "is_signed": false}`)
  
    buffer.WriteString("]}")
    buffer.WriteString(",")
  
    buffer.WriteString(`{"name": "T", "attributes": [`)
    buffer.WriteString(`{"name": "x", "offset": 0, "width": 64, "is_signed": false}`)
  
    buffer.WriteString("]}")
  
    buffer.WriteString(`], "index_type": {"name": "IndexType16", "attributes": [`)
    buffer.WriteString(`{"name": "value", "offset": 0, "width": 16, "is_signed": false}`)
 
    buffer.WriteString("]}}")
	return buffer.String()
}
    
    
type ADataU64IndexVector struct {
    descriptor flatdata.MemoryDescriptor
    IsOptional bool
    IsOpen bool
}

func (v *ADataU64IndexVector) Get(i int) *IndexType64 {
	return &IndexType64{
		descriptor: v.descriptor,
		position: int(uint(i*indexType64SizeInBytes) + flatdataOffsetSizeInBytes),
	}
}

func (v *ADataU64IndexVector) GetSize() int {
	size := make([]byte, 8)
	_, err := v.descriptor.ReadAt(size, 0)
	if err != nil {
		return 0
	}
	intSize := binary.LittleEndian.Uint64(size)
	return int(intSize) / indexType64SizeInBytes
}

func (v *ADataU64IndexVector) GetSlice(start, end, step int) []*IndexType64 {
	var result []*IndexType64	
    for start <= end {
		result = append(result, &IndexType64{
			descriptor: v.descriptor,
			position: int(uint(start*indexType64SizeInBytes) + flatdataOffsetSizeInBytes),
	    })
		start += step
	}
	return result
}

func (v *ADataU64IndexVector) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *ADataU64IndexVector) GetSizeInBytes() int {
    return v.descriptor.Len()
}

func (v *ADataU64IndexVector) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"container_type": "Vector", "size": %d, "size_in_bytes": %d, `, v.GetSize(), v.GetSizeInBytes()))
    buffer.WriteString(`"element_types": [{ "name": "IndexType64", "attributes": [`)
    buffer.WriteString(`{"name": "value", "offset": 0, "width": 64, "is_signed": false}`)
   
    buffer.WriteString("]}]}")
	return buffer.String()
}


type ADataU64IndexMultivector struct {
    descriptor flatdata.MemoryDescriptor
    index      *ADataU64IndexVector
	types      map[int]interface{}
    IsOptional bool
    IsOpen bool
}

func (v *ADataU64IndexMultivector) Close() {
    v.descriptor.Close()
    v.IsOpen = false
}

func (v *ADataU64IndexMultivector) GetSize() int {
	return v.index.GetSize()
}

func (v *ADataU64IndexMultivector) GetSizeInBytes() int {
    return v.descriptor.Len()
}

func (v *ADataU64IndexMultivector) getBucketOffset(i int) int {
    if i == v.index.GetSize() {
		return v.descriptor.Len() - int(flatdataPaddingSizeInBytes)
	} 
	return int(v.index.Get(i).GetValue()) + int(flatdataOffsetSizeInBytes)
}

func (v *ADataU64IndexMultivector) Get(i int) []interface{} {
    offset := v.getBucketOffset(i)
	nextOffset := v.getBucketOffset(i + 1)
	var result []interface{}

	for offset < nextOffset {
	    elementType := flatdata.Read(v.descriptor, uint(offset*8), 8, false)
		offset++
		abstractElement, ok := v.types[elementType]
		if !ok {
			//TODO: How to process case, then type of element is not found?
			log.Println("Can't get type of element")
		}
    
		switch element := abstractElement.(type) {
		case *S:
			element.position = offset
			result = append(result, element)
			offset += sSizeInBytes
		case *T:
			element.position = offset
			result = append(result, element)
			offset += tSizeInBytes
		default:
			//TODO: How to react in case if it's impossible to cast?
			log.Println("Can't cast element. Type is unknown...")
		}
	}
	
	return result
}

func (v *ADataU64IndexMultivector) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"container_type": "Multivector", "size": %d, "size_in_bytes": %d, "element_types": [`, v.GetSize(), v.GetSizeInBytes()))
    buffer.WriteString(`{"name": "S", "attributes": [`)
    buffer.WriteString(`{"name": "x", "offset": 0, "width": 64, "is_signed": false}`)
  
    buffer.WriteString("]}")
    buffer.WriteString(",")
  
    buffer.WriteString(`{"name": "T", "attributes": [`)
    buffer.WriteString(`{"name": "x", "offset": 0, "width": 64, "is_signed": false}`)
  
    buffer.WriteString("]}")
  
    buffer.WriteString(`], "index_type": {"name": "IndexType64", "attributes": [`)
    buffer.WriteString(`{"name": "value", "offset": 0, "width": 64, "is_signed": false}`)
 
    buffer.WriteString("]}}")
	return buffer.String()
}
    
type AArchive struct {
    IsOptional bool
    IsOpen bool
    DataMultivector *ADataMultivector
    OptionalDataMultivector *AOptionalDataMultivector
    DataU64IndexMultivector *ADataU64IndexMultivector
}

func (v *AArchive) Close() {
    if v.DataMultivector.IsOpen {
        v.DataMultivector.Close()
    }
    if v.OptionalDataMultivector.IsOpen {
        v.OptionalDataMultivector.Close()
    }
    if v.DataU64IndexMultivector.IsOpen {
        v.DataU64IndexMultivector.Close()
    }
}

func (v *AArchive) GetSizeInBytes() int {
    var size int
    if v.DataMultivector.IsOpen {
        size += v.DataMultivector.GetSizeInBytes()
    }
    if v.OptionalDataMultivector.IsOpen {
        size += v.OptionalDataMultivector.GetSizeInBytes()
    }
    if v.DataU64IndexMultivector.IsOpen {
        size += v.DataU64IndexMultivector.GetSizeInBytes()
    }
    return size
}

func (v *AArchive) ToString() string {
    buffer := bytes.Buffer{}
    buffer.WriteString(fmt.Sprintf(`{"name": "A", "container_type": "Archive", "size_in_bytes": %d, "resources": [`, v.GetSizeInBytes()))
    buffer.WriteString(v.DataMultivector.ToString())
    buffer.WriteString(",")
      
    buffer.WriteString(v.OptionalDataMultivector.ToString())
    buffer.WriteString(",")
      
    buffer.WriteString(v.DataU64IndexMultivector.ToString())
      
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
	dataIndexMemoryDescriptor, _, err := resource.GetMemoryDescriptor("data_index")
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
struct T
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    data : multivector< 8, .n.S, .n.T >;
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
	optionalDataIndexMemoryDescriptor, _, err := resource.GetMemoryDescriptor("optional_data_index")
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
struct T
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    @optional
    optional_data : multivector< 16, .n.S, .n.T >;
}
}

`
	    if optionalDataSchema != schema {
	        log.Println(fmt.Sprintf("Schemas didn't match, expected: \n%s\n, actual: \n%s\n", optionalDataSchema, schema))
	        return v, errors.New(flatdata.ErrorSchemaDidntMatch)
	    }
	}
	dataU64IndexIsOpen := true
	dataU64IndexMemoryDescriptor, schema, err := resource.GetMemoryDescriptor("data_u64_index")
	if err != nil {
        log.Println(err)
	    if err.Error() == flatdata.ErrorCantAccessResource {
	        dataU64IndexIsOpen = false
	    } else {
		    return v, err
		}
	}
	dataU64IndexIndexMemoryDescriptor, _, err := resource.GetMemoryDescriptor("data_u64_index_index")
	if err != nil {
	    log.Println(err)
	    if err.Error() == flatdata.ErrorCantAccessResource {
	        dataU64IndexIsOpen = false
	    } else {
		    return v, err
		}
	}
	if dataU64IndexIsOpen {
	    dataU64IndexSchema := `namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace n {
struct T
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    data_u64_index : multivector< 64, .n.S, .n.T >;
}
}

`
	    if dataU64IndexSchema != schema {
	        log.Println(fmt.Sprintf("Schemas didn't match, expected: \n%s\n, actual: \n%s\n", dataU64IndexSchema, schema))
	        return v, errors.New(flatdata.ErrorSchemaDidntMatch)
	    }
	}
	// Add resources to archive
    v.DataMultivector = &ADataMultivector{
        descriptor: dataMemoryDescriptor,
        index: &ADataVector{descriptor: dataIndexMemoryDescriptor},
        types: map[int]interface{}{
            0: &S{descriptor: dataMemoryDescriptor},
            1: &T{descriptor: dataMemoryDescriptor},
        },
        IsOptional: false,
        IsOpen: dataIsOpen,
    }
    v.OptionalDataMultivector = &AOptionalDataMultivector{
        descriptor: optionalDataMemoryDescriptor,
        index: &AOptionalDataVector{descriptor: optionalDataIndexMemoryDescriptor},
        types: map[int]interface{}{
            0: &S{descriptor: optionalDataMemoryDescriptor},
            1: &T{descriptor: optionalDataMemoryDescriptor},
        },
        IsOptional: true,
        IsOpen: optionalDataIsOpen,
    }
    v.DataU64IndexMultivector = &ADataU64IndexMultivector{
        descriptor: dataU64IndexMemoryDescriptor,
        index: &ADataU64IndexVector{descriptor: dataU64IndexIndexMemoryDescriptor},
        types: map[int]interface{}{
            0: &S{descriptor: dataU64IndexMemoryDescriptor},
            1: &T{descriptor: dataU64IndexMemoryDescriptor},
        },
        IsOptional: false,
        IsOpen: dataU64IndexIsOpen,
    }
	return v, nil
}



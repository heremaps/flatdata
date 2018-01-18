instance_test_schema = """
namespace backward_compatibility {
    struct SignedStruct {
        a : i16 : 5;
        b : u32 : 32;
        c : i32 : 7;
        d : u32 : 32;
    }
    archive Archive {
        resource: SignedStruct;
    }
}
"""
resource_payload = (
    b"\x0a\x00\x00\x00\x00\x00\x00\x00"  # size of payload in bytes
    b"\xff\xac\x68\x24\x00\x0b\x00\x00"  # Payload
    b"\x00\x00"  # Payload
    b"\x00\x00\x00\x00\x00\x00\x00\x00"  # Padding
)

archive_signature_payload = (
    b"\x00\x00\x00\x00\x00\x00\x00\x00"
    b"\x00\x00\x00\x00\x00\x00\x00\x00"
)

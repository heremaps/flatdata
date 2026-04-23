"""Tests for vectorized numpy access paths."""

import numpy as np
import pytest

from flatdata.generator.engine import Engine
from flatdata.lib.data_access import read_field_vectorized
from common import (
    DictResourceStorage,
    ARCHIVE_SIGNATURE_PAYLOAD,
    VECTOR_TEST_SCHEMA,
    RESOURCE_VECTOR_PAYLOAD,
)


def _make_vector_archive():
    """Create a test archive with a vector of SignedStructs."""
    module = Engine(VECTOR_TEST_SCHEMA).render_python_module()
    valid_data = {
        "Archive.archive": ARCHIVE_SIGNATURE_PAYLOAD,
        "Archive.archive.schema": module.backward_compatibility_Archive.schema().encode(),
        "resource": RESOURCE_VECTOR_PAYLOAD,
        "resource.schema": module.backward_compatibility_Archive.resource_schema('resource').encode()
    }
    archive = module.backward_compatibility_Archive(DictResourceStorage(valid_data))
    return archive, module


class TestReadFieldVectorized:
    """Tests for the read_field_vectorized function."""

    def test_all_fields_match_element_access(self):
        archive, module = _make_vector_archive()
        vector = archive.resource
        raw_2d = vector._as_numpy_2d()

        from flatdata.lib.data_access import read_field_vectorized

        for name, field in vector._element_type._FIELDS.items():
            values = read_field_vectorized(
                raw_2d, field.offset, field.width, field.is_signed
            )
            for i in range(len(vector)):
                expected = getattr(vector[i], name)
                actual = int(values[i])
                assert expected == actual, \
                    f"Mismatch in {name}[{i}]: expected={expected}, actual={actual}"

    def test_signed_fields_read_correctly(self):
        archive, module = _make_vector_archive()
        vector = archive.resource
        raw_2d = vector._as_numpy_2d()

        from flatdata.lib.data_access import read_field_vectorized

        # Field 'a' is i16:5 (signed, 5 bits), expected value: -1
        field_a = vector._element_type._FIELDS['a']
        values_a = read_field_vectorized(raw_2d, field_a.offset, field_a.width, field_a.is_signed)
        assert int(values_a[0]) == -1
        assert int(values_a[1]) == -1

        # Field 'c' is i32:7 (signed, 7 bits), expected value: -0x28 = -40
        field_c = vector._element_type._FIELDS['c']
        values_c = read_field_vectorized(raw_2d, field_c.offset, field_c.width, field_c.is_signed)
        assert int(values_c[0]) == -0x28
        assert int(values_c[1]) == -0x28


class TestVectorToNumpy:
    """Tests for vectorized Vector.to_numpy()."""

    def test_to_numpy_matches_element_access(self):
        archive, module = _make_vector_archive()
        vector = archive.resource
        arr = vector.to_numpy()

        assert len(arr) == len(vector)
        for name in vector._element_type._FIELDS:
            for i in range(len(vector)):
                expected = getattr(vector[i], name)
                actual = int(arr[name][i])
                assert expected == actual

    def test_to_numpy_dtype(self):
        archive, module = _make_vector_archive()
        vector = archive.resource
        arr = vector.to_numpy()
        assert arr.dtype == np.dtype(vector._element_type.dtype())

    def test_to_data_frame(self):
        archive, module = _make_vector_archive()
        vector = archive.resource
        df = vector.to_data_frame()
        assert len(df) == len(vector)
        assert list(df.columns) == list(vector._element_type._FIELDS.keys())


class TestVectorSliceToNumpy:
    """Tests for vectorized _VectorSlice.to_numpy()."""

    def test_slice_to_numpy(self):
        archive, module = _make_vector_archive()
        vector = archive.resource
        s = vector[0:1]
        arr = s.to_numpy()

        assert len(arr) == 1
        for name in vector._element_type._FIELDS:
            expected = getattr(vector[0], name)
            actual = int(arr[name][0])
            assert expected == actual

    def test_slice_to_data_frame(self):
        archive, module = _make_vector_archive()
        vector = archive.resource
        df = vector[0:2].to_data_frame()
        assert len(df) == 2


class TestVectorColumnAccess:
    """Tests for vectorized Vector.__getattr__ column access."""

    def test_column_access_returns_dataframe(self):
        archive, module = _make_vector_archive()
        vector = archive.resource
        df = vector.a
        assert len(df) == len(vector)
        assert 'a' in df.columns

    def test_column_values_match(self):
        archive, module = _make_vector_archive()
        vector = archive.resource
        df = vector.b
        for i in range(len(vector)):
            expected = getattr(vector[i], 'b')
            actual = int(df['b'].iloc[i])
            assert expected == actual


class TestNumpyCache:
    """Tests for the _as_numpy_2d() cache."""

    def test_cache_returns_same_object(self):
        archive, module = _make_vector_archive()
        vector = archive.resource
        arr1 = vector._as_numpy_2d()
        arr2 = vector._as_numpy_2d()
        assert arr1 is arr2

    def test_shape(self):
        archive, module = _make_vector_archive()
        vector = archive.resource
        arr = vector._as_numpy_2d()
        assert arr.shape == (len(vector), vector._element_type._SIZE_IN_BYTES)
        assert arr.dtype == np.uint8


class TestStructureSlots:
    """Tests that Structure uses __slots__."""

    def test_has_slots(self):
        from flatdata.lib.structure import Structure
        assert hasattr(Structure, '__slots__')
        assert '_mem' in Structure.__slots__
        assert '_pos' in Structure.__slots__


class TestReadFieldVectorizedEdgeCases:
    """Tests for boundary conditions in vectorized field reading."""

    def test_1bit_unsigned(self):
        raw = np.array([[0x01], [0x00], [0x03]], dtype=np.uint8)
        result = read_field_vectorized(raw, 0, 1, False)
        assert list(result) == [1, 0, 1]

    def test_1bit_signed_matches_scalar(self):
        """1-bit signed fields should return 0 or 1, matching read_value behavior."""
        from flatdata.lib.data_access import read_value
        raw = np.array([[0x01], [0x00]], dtype=np.uint8)
        result = read_field_vectorized(raw, 0, 1, True)
        assert int(result[0]) == read_value(b'\x01', 0, 1, True)
        assert int(result[1]) == read_value(b'\x00', 0, 1, True)

    def test_64bit_unsigned(self):
        raw = np.array([[0xFF] * 8], dtype=np.uint8)
        result = read_field_vectorized(raw, 0, 64, False)
        assert int(result[0]) == 0xFFFFFFFFFFFFFFFF

    def test_64bit_signed_negative(self):
        raw = np.array([[0xFF] * 8], dtype=np.uint8)
        result = read_field_vectorized(raw, 0, 64, True)
        assert int(result[0]) == -1

    def test_64bit_signed_positive(self):
        raw = np.array([[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]], dtype=np.uint8)
        result = read_field_vectorized(raw, 0, 64, True)
        assert int(result[0]) == 1

    def test_63bit_signed(self):
        raw = np.array([[0xFF] * 8], dtype=np.uint8)
        result = read_field_vectorized(raw, 0, 63, True)
        assert int(result[0]) == -1

    def test_unaligned_large_field(self):
        """Fields where offset%8 + width > 64 require extra byte merge."""
        raw = np.array([[0xFF] * 9], dtype=np.uint8)
        # 64 bits starting at bit 1, all set → should be 0xFFFFFFFFFFFFFFFF
        actual = int(read_field_vectorized(raw, 1, 64, False)[0])
        assert actual == 0xFFFFFFFFFFFFFFFF

    def test_empty_vector(self):
        raw = np.zeros((0, 8), dtype=np.uint8)
        result = read_field_vectorized(raw, 0, 32, False)
        assert len(result) == 0


class TestAttributeErrorContract:
    """Vector/slice __getattr__ must raise AttributeError for unknown fields."""

    def test_vector_unknown_field_raises_attribute_error(self):
        archive, _ = _make_vector_archive()
        with pytest.raises(AttributeError):
            archive.resource.nonexistent_field

    def test_vector_hasattr_returns_false(self):
        archive, _ = _make_vector_archive()
        assert not hasattr(archive.resource, "nonexistent_field")

    def test_slice_unknown_field_raises_attribute_error(self):
        archive, _ = _make_vector_archive()
        with pytest.raises(AttributeError):
            archive.resource[0:1].nonexistent_field

/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include <cstddef>
#include <cstdint>
#include <sstream>

namespace flatdata
{
struct MemoryDescriptor
{
public:
    MemoryDescriptor( ) = default;
    MemoryDescriptor( const uint8_t* ptr, size_t size )
        : m_ptr( ptr )
        , m_size( size )
    {
    }
    MemoryDescriptor( const char* ptr, size_t size )
        : MemoryDescriptor( reinterpret_cast< const uint8_t* >( ptr ), size )
    {
    }

    explicit operator bool( ) const
    {
        return m_ptr != nullptr;
    }

    std::string
    describe( size_t nest_level = 0 ) const
    {
        std::ostringstream ss;
        if ( this->operator bool( ) )
        {
            ss << "Raw data of size " << m_size;
        }
        else
        {
            ss << "Uninitialized Raw data";
        }
        return ss.str( );
    }

    const uint8_t*
    data( ) const
    {
        return m_ptr;
    }

    const char*
    char_ptr( ) const
    {
        return reinterpret_cast< const char* >( m_ptr );
    }

    size_t
    size_in_bytes( ) const
    {
        return m_size;
    }

    size_t
    size( ) const
    {
        return m_size;
    }

private:
    const uint8_t* m_ptr = nullptr;
    size_t m_size = 0;
};

}  // namespace flatdata

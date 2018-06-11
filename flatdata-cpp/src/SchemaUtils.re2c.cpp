/**
 * Copyright (c) 2018 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include <flatdata/internal/SchemaUtils.h>

#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <cassert>

namespace flatdata
{
namespace internal
{
namespace
{
struct Token
{
    enum class KIND
    {
        // Keywords, identifiers, etc.
        WORD,
        // Braces, parentheses, colon, semicolon etc.
        PUNCTUATION,
        // Numbers,
        NUMBER,
        // Unexpected byte
        INVALID,
        // End of input
        END,
    } kind;

    struct Word
    {
        const char* start;
        const char* end;
    };

    union {
        // WORD
        Word word;
        // PUNCTUATION or INVALID
        char character;
        // NUMBER
        int64_t number;
    };
};

bool
operator==( const Token& t1, const Token& t2 )
{
    if ( t1.kind == t2.kind )
    {
        switch ( t1.kind )
        {
        case Token::KIND::WORD:
            return ( t1.word.end - t1.word.start ) == ( t2.word.end - t2.word.start )
                   && strncmp( t1.word.start, t2.word.start, t1.word.end - t1.word.start ) == 0;
        case Token::KIND::PUNCTUATION:
        case Token::KIND::INVALID:
            return t1.character == t2.character;
        case Token::KIND::NUMBER:
            return t1.number == t2.number;
        case Token::KIND::END:
            return true;
        }
    }
    else
    {
        return false;
    }
}

bool
operator!=( const Token& t1, const Token& t2 )
{
    return !( t1 == t2 );
}

class Lexer
{
public:
    Lexer( const char* data, size_t size )
        : m_data( data )
        , m_end( data + size )
    {
    }

    Token next_token( );

private:
    const char* m_data;
    const char* m_end;
};

Token
Lexer::next_token( )
{
    /*!types:re2c*/

    const char* marker;
    const char* context_marker;
    YYCONDTYPE c = yycinit;

    for ( ;; )
    {
        if ( m_data == m_end )
        {
            Token t;
            t.kind = Token::KIND::END;
            return t;
        }

        const char* start = m_data;
        // enough to hold INT64_MIN as decimal or hex + nul byte
        char number_buffer[ 21 ];

        // Using the custom interface to be able to deal inputs that are not nul
        // terminated. The nul byte is still an invalid character so we can use
        // it as a sentinel.
#define YYCTYPE char
#define YYPEEK( ) ( ( m_data < m_end ) ? *m_data : 0 )
#define YYSKIP( ) ( ++m_data )
#define YYBACKUP( ) ( marker = m_data )
#define YYRESTORE( ) ( m_data = marker )
#define YYBACKUPCTX( ) ( context_marker = m_data )
#define YYRESTORECTX( ) ( m_data = context_marker )

        // clang-format off
        /*!re2c
        // input is complete, no buffer filling
        re2c:yyfill:enable = 0;
        re2c:indent:string = "    ";
        re2c:indent:top = 2;
        re2c:define:YYGETCONDITION = "c";
        re2c:define:YYGETCONDITION:naked = 1;
        re2c:define:YYSETCONDITION = "c = @@;";
        re2c:define:YYSETCONDITION:naked = 1;
        re2c:cond:divider = "// condition: @@";

        whitespace = [ \t\r\n];
        nul = '\000';
        word = "@"? [A-Za-z_] [A-Za-z0-9_]*;
        decimal = "-"? [0-9]{1,19};
        hex = "-"? "0x" [0-9a-fA-F]{1,19};
        punctuation = [;:{}<>\[\]];

        <*> * {
            Token t;
            t.kind = Token::KIND::INVALID;
            t.character = yych;
            return t;
        }

        <init> "//" :=> cpp_comment
        <init> "/*" :=> c_comment

        <init> whitespace { continue; }
        <init> word
        {
            Token t;
            t.kind = Token::KIND::WORD;
            t.word.start = start;
            t.word.end = m_data;
            return t;
        }
        <init> punctuation
        {
            Token t;
            t.kind = Token::KIND::PUNCTUATION;
            t.character = yych;
            return t;
        }
        <init> decimal / ( whitespace | punctuation | nul ) {
            assert( sizeof( number_buffer ) > m_data - start );
            memset(number_buffer, 0, sizeof(number_buffer));
            memcpy(number_buffer, start, m_data - start);
            Token t;
            t.kind = Token::KIND::NUMBER;
            t.number = strtoll( number_buffer, nullptr, 10 );
            return t;
        }
        <init> hex / ( whitespace | punctuation | nul ) {
            assert( sizeof( number_buffer ) > m_data - start );
            memset(number_buffer, 0, sizeof(number_buffer));
            memcpy(number_buffer, start, m_data - start);
            Token t;
            t.kind = Token::KIND::NUMBER;
            t.number = strtoll( number_buffer, nullptr, 16 );
            return t;
        }

        <cpp_comment> "\n" => init { continue; }
        <cpp_comment> * { continue; }

        <c_comment> "*" "/" => init { continue; }
        <c_comment> * { continue; }
        */
        // clang-format on
    }
}
}  // anonymous namespace

bool
schema_equal( const char* schema1, size_t size1, const char* schema2, size_t size2 )
{
    if ( size1 == size2 && strncmp( schema1, schema2, size1 ) == 0 )
    {
        return true;
    }

    Lexer l1{schema1, size1};
    Lexer l2{schema2, size2};

    for ( ;; )
    {
        Token t1 = l1.next_token( );
        Token t2 = l2.next_token( );

        if ( t1 != t2 )
        {
            return false;
        }
        else if ( t1.kind == Token::KIND::END )
        {
            return true;
        }
    }
}

}  // namespace internal
}  // namespace flatdata

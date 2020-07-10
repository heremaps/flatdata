class A : public flatdata::Archive
{
public:
    /// Archive schema
    static const char* schema_definition( );
    /// Archive name
    static const char* name_definition( );

public:
    /**
    * Create and open archive at path.
    * In case opening fails, is_open() or operator bool() returns false.
    *
    * @sa is_open
    * @sa operator bool()
    */
    static A open( std::shared_ptr< flatdata::ResourceStorage > storage );
    A( ) = default;


    const char* name( ) const override;
    const char* schema( ) const override;

private:
    explicit A( std::shared_ptr< flatdata::ResourceStorage > storage );

    bool load_contents( ) override;
    void describe_resources( std::ostream& stream, size_t nest_level ) const override;

private:
};

class ABuilder : public flatdata::ArchiveBuilder
{
public:
    /// Creates Archive builder
    static ABuilder open( std::shared_ptr< flatdata::ResourceStorage > storage );
    /// Archive schema
    static const char* schema_definition( );

public:  /// Common methods
    ABuilder( ) = default;
    const char* name( ) const override;
    const char* schema( ) const override;

public:  /// Resources


private:
    ABuilder( std::shared_ptr< flatdata::ResourceStorage > storage );

};
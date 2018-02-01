Building archives
=================

Archives are designed as efficient write-once read-many data storage.
Thus, they do not look or feel like conventional databases, instead,
when building, they provide efficient serializers which write data to
the underlying storage:

-  Once structure's data is set, it cannot be reset (serialization is
   optimized for write-once scenario).
-  Once resource is created, it cannot be altered anymore. Attempt to do
   so will result in error.
-  Once archive exists, all its resources with the exception of archive
   resources are final, even if missing. Only missing archive resources
   can be created within existing archive.

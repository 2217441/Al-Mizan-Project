use serde::Serializer;
use surrealdb::sql::Thing;

/// Serializes a SurrealDB Thing directly to a string, avoiding intermediate String allocation.
///
/// This uses `serializer.collect_str()` which leverages the `Display` implementation
/// of `Thing` to write directly to the output buffer.
pub fn serialize_thing_id<S>(thing: &Thing, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.collect_str(thing)
}

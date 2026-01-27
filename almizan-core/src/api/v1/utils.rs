use serde::Serializer;
use surrealdb::sql::Thing;

/// Formats a SurrealDB Thing as a simple "table:id" string.
pub fn format_surreal_id(thing: &Thing) -> String {
    match &thing.id {
        surrealdb::sql::Id::String(s) => format!("{}:{}", thing.tb, s),
        surrealdb::sql::Id::Number(n) => format!("{}:{}", thing.tb, n),
        _ => {
            let s = thing.to_string();
            if s.contains('⟨') || s.contains('⟩') {
                s.replace(['⟨', '⟩'], "")
            } else {
                s
            }
        }
    }
}

/// Serializes a SurrealDB Thing as a simple "table:id" string.
/// This avoids intermediate String allocation by writing directly to the serializer.
pub fn serialize_thing_id<S>(thing: &Thing, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match &thing.id {
        surrealdb::sql::Id::String(s) => serializer.collect_str(&format_args!("{}:{}", thing.tb, s)),
        surrealdb::sql::Id::Number(n) => serializer.collect_str(&format_args!("{}:{}", thing.tb, n)),
        _ => {
            let s = thing.to_string();
            if s.contains('⟨') || s.contains('⟩') {
                serializer.collect_str(&s.replace(['⟨', '⟩'], ""))
            } else {
                serializer.collect_str(&s)
            }
        }
    }
}

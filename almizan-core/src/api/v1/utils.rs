use serde::Serializer;
use std::fmt;
use surrealdb::sql::Thing;

pub struct DisplayThing<'a>(pub &'a Thing);

impl<'a> fmt::Display for DisplayThing<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let thing = self.0;
        match &thing.id {
            surrealdb::sql::Id::String(s) => write!(f, "{}:{}", thing.tb, s),
            surrealdb::sql::Id::Number(n) => write!(f, "{}:{}", thing.tb, n),
            _ => {
                let s = thing.to_string();
                if s.contains('⟨') || s.contains('⟩') {
                    write!(f, "{}", s.replace(['⟨', '⟩'], ""))
                } else {
                    write!(f, "{}", s)
                }
            }
        }
    }
}

/// Formats a SurrealDB Thing as a simple "table:id" string.
pub fn format_surreal_id(thing: &Thing) -> String {
    format!("{}", DisplayThing(thing))
}

/// Serializes a SurrealDB Thing as a simple "table:id" string.
/// This avoids intermediate String allocation by writing directly to the serializer.
pub fn serialize_thing_id<S>(thing: &Thing, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.collect_str(&DisplayThing(thing))
}

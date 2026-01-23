use surrealdb::sql::{Id, Thing};

/// Helper to get string representation of Thing without `SurrealQL` escaping (brackets)
/// Optimization: Avoids `to_string()` overhead (checking escaping) and `sanitize_id()` overhead (replacing)
/// for common cases (String and Number IDs).
#[must_use]
pub fn format_surreal_id(thing: &Thing) -> String {
    match &thing.id {
        Id::String(s) => format!("{}:{}", thing.tb, s),
        Id::Number(n) => format!("{}:{}", thing.tb, n),
        _ => {
            let s = thing.to_string();
            if s.contains('⟨') || s.contains('⟩') {
                s.replace(&['⟨', '⟩'][..], "")
            } else {
                s
            }
        }
    }
}

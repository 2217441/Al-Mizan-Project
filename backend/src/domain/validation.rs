use crate::domain::verse::Verse;

#[allow(dead_code)]
pub trait ChronologicalValidation {
    fn is_chronologically_valid(&self, other: &Self) -> bool;
}

impl ChronologicalValidation for Verse {
    /// Checks if `self` (the abrogating verse/Nasikh) was revealed AFTER `other` (the abrogated verse/Mansukh).
    /// Returns true if valid (Nasikh > Mansukh).
    fn is_chronologically_valid(&self, other: &Self) -> bool {
        self.revelation_order > other.revelation_order
    }
}

// Validation logic has been moved to the Graph Layer (The Mizan).
// This file is kept for future domain-specific validation rules.

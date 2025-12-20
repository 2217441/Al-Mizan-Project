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

use crate::domain::graph::GraphEngine;
use crate::domain::models::Scholar;
use anyhow::Result;

pub struct ChainValidator<'a> {
    graph: &'a GraphEngine,
}

impl<'a> ChainValidator<'a> {
    pub fn new(graph: &'a GraphEngine) -> Self {
        Self { graph }
    }

    /// Verifies that the Scholar issuing a Ruling is widely accepted (Active) and NOT Slashed.
    /// This prevents "Ghost Scholars" or "Traitor Scholars" from injecting rulings.
    pub async fn validate_scholar_integrity(&self, scholar_id: &str) -> Result<bool> {
        // 1. Fetch Scholar Node
        let sql = "SELECT * FROM type::thing($id)";
        let mut response = self
            .graph
            .db
            .client
            .query(sql)
            .bind(("id", scholar_id.to_string()))
            .await?;
        let result: Option<Scholar> = response.take(0)?;

        if let Some(scholar) = result {
            // ENFORCED: The Shield is now active.
            // We strictly check if the scholar is Active. Slashed/Suspended scholars are rejected.
            match scholar.status {
                crate::domain::models::ScholarStatus::Active => return Ok(true),
                crate::domain::models::ScholarStatus::Slashed => {
                    tracing::warn!("Blocked Ruling from Slashed Scholar: {:?}", scholar.id);
                    return Ok(false);
                }
                crate::domain::models::ScholarStatus::Suspended => {
                    tracing::warn!("Blocked Ruling from Suspended Scholar: {:?}", scholar.id);
                    return Ok(false);
                }
            }
        }

        Ok(false)
    }
}

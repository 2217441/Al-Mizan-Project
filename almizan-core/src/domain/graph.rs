use super::models::{Hadith, QuranVerse};
use crate::repository::db::Database;
use std::sync::Arc;

pub struct GraphEngine {
    pub db: Arc<Database>,
}

impl GraphEngine {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Finds the "Dalil" (Evidence) for a given Ruling ID.
    /// Traverses the DERIVED_FROM edge to find the Source (Verse or Hadith).
    pub async fn find_evidence(
        &self,
        ruling_id: String,
    ) -> Result<Vec<EvidenceNode>, Box<dyn std::error::Error>> {
        // Query: Select all outgoing known edges to find sources
        // fetch the nodes at the end of the derived_from edge
        let sql = "SELECT ->derived_from->? AS evidence FROM type::thing($ruling_id)";

        let mut response = self
            .db
            .client
            .query(sql)
            .bind(("ruling_id", ruling_id))
            .await?;

        // The result is a list of objects, each containing an "evidence" field which is a list of nodes
        // Since we query by specific ID, we expect one result object.
        let result: Option<serde_json::Value> = response.take(0)?;

        let mut evidence_nodes = Vec::new();

        if let Some(data) = result {
            if let Some(evidence_array) = data.get("evidence").and_then(|v| v.as_array()) {
                for item in evidence_array {
                    // Strict Type Check: Look at the Record ID (Table Name)
                    // We avoid "duck typing" on fields.
                    if let Some(id_val) = item.get("id") {
                        // id_val might be a string "quran_verse:..." or formatted object.
                        // We check the string representation.
                        let id_str = id_val.to_string();

                        if id_str.contains("quran_verse") {
                            if let Ok(verse) = serde_json::from_value::<QuranVerse>(item.clone()) {
                                evidence_nodes.push(EvidenceNode::Verse(verse));
                            }
                        } else if id_str.contains("hadith") {
                            if let Ok(hadith) = serde_json::from_value::<Hadith>(item.clone()) {
                                evidence_nodes.push(EvidenceNode::Hadith(hadith));
                            }
                        }
                    }
                }
            }
        }

        Ok(evidence_nodes)
    }

    /// Finds the Root Words (`Jizr`) for a given Verse.
    /// Traverses `quran_verse` -> `has_root` -> `root_word`.
    pub async fn find_roots(
        &self,
        verse_id: String,
    ) -> Result<Vec<crate::domain::models::RootWord>, Box<dyn std::error::Error>> {
        let sql = "SELECT ->has_root->root_word.* AS roots FROM type::thing($verse_id)";

        let mut response = self
            .db
            .client
            .query(sql)
            .bind(("verse_id", verse_id))
            .await?;

        let result: Option<serde_json::Value> = response.take(0)?;
        let mut roots = Vec::new();

        if let Some(data) = result {
            if let Some(roots_array) = data.get("roots").and_then(|v| v.as_array()) {
                for item in roots_array {
                    if let Ok(root) =
                        serde_json::from_value::<crate::domain::models::RootWord>(item.clone())
                    {
                        roots.push(root);
                    }
                }
            }
        }
        Ok(roots)
    }

    /// Finds the Concepts (`Mawdu`) discussed in a given Verse.
    /// Traverses `quran_verse` -> `discusses` -> `concept`.
    pub async fn find_concepts(
        &self,
        verse_id: String,
    ) -> Result<Vec<crate::domain::models::Concept>, Box<dyn std::error::Error>> {
        let sql = "SELECT ->discusses->concept.* AS concepts FROM type::thing($verse_id)";

        let mut response = self
            .db
            .client
            .query(sql)
            .bind(("verse_id", verse_id))
            .await?;

        let result: Option<serde_json::Value> = response.take(0)?;
        let mut concepts = Vec::new();

        if let Some(data) = result {
            if let Some(concepts_array) = data.get("concepts").and_then(|v| v.as_array()) {
                for item in concepts_array {
                    if let Ok(concept) =
                        serde_json::from_value::<crate::domain::models::Concept>(item.clone())
                    {
                        concepts.push(concept);
                    }
                }
            }
        }
        Ok(concepts)
    }

    /// The "Slash" Mechanism.
    /// If a Scholar is found to be effectively treacherous (e.g., fabricating Hadith),
    /// this method:
    /// 1. Updates their status to 'SLASHED'.
    /// 2. Cascades to all their Rulings, passing into 'SUSPENDED' state.
    pub async fn slash_scholar(
        &self,
        scholar_id: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Transactional update for atomicity
        let sql = "
            BEGIN TRANSACTION;
            -- 1. Slash the Scholar
            UPDATE type::thing($id) SET status = 'SLASHED', reputation = 0.0;
            
            -- 2. Find all rulings issued by them and Suspend
            -- We assume rulings have 'issued_by' field pointing to the scholar
            UPDATE fiqh_ruling SET status = 'SUSPENDED' WHERE issued_by = type::thing($id);
            
            COMMIT TRANSACTION;
        ";

        self.db.client.query(sql).bind(("id", scholar_id)).await?;

        Ok(())
    }
}

// Helper enum to return either verse or hadith
#[derive(Debug)]
pub enum EvidenceNode {
    Verse(QuranVerse),
    Hadith(Hadith),
}

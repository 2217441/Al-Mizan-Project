use crate::domain::traits::{Attributable, DigitalProvenance};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[allow(dead_code)]
pub struct Opinion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[validate(length(min = 1))]
    pub scholar_id: String,

    #[validate(length(min = 1))]
    pub text_evidence_id: String, // The Verse/Hadith being discussed

    #[validate(length(min = 1))]
    pub verdict: String, // e.g., "Abrogated", "Specific", "General"

    pub methodology: String, // e.g., "Naskh"

    #[validate(length(min = 1))]
    pub school: String, // Denormalized context

    #[validate(range(min = 0.0, max = 1.0))]
    pub weight: f32, // The Mizan (0.0 - 1.0)

    // Digital Provenance
    #[serde(default = "default_digitizer")]
    pub digitized_by: String,
    #[serde(default = "default_signature")]
    pub signature: String,
}

fn default_digitizer() -> String {
    "System".to_string()
}

fn default_signature() -> String {
    "unsigned".to_string()
}

impl Opinion {
    pub fn new(
        scholar_id: String,
        text_evidence_id: String,
        verdict: String,
        methodology: String,
        school: String,
        weight: f32,
    ) -> Self {
        Self {
            id: None,
            scholar_id,
            text_evidence_id,
            verdict,
            methodology,
            school,
            weight,
            digitized_by: "System".to_string(),
            signature: "unsigned".to_string(),
        }
    }
}

impl Attributable for Opinion {
    fn source(&self) -> String {
        // An opinion is attributed to the scholar who holds it.
        self.scholar_id.clone()
    }
}

impl DigitalProvenance for Opinion {
    fn digitized_by(&self) -> String {
        self.digitized_by.clone()
    }
    fn signature(&self) -> String {
        self.signature.clone()
    }
}

use crate::domain::traits::{Attributable, DigitalProvenance};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[allow(dead_code)]
pub struct Scholar {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[validate(length(min = 1))]
    pub name: String,

    pub death_date: String,
    pub school: String,

    // Digital Provenance
    #[serde(default = "default_digitizer")]
    pub digitized_by: String,
    #[serde(default = "default_signature")]
    pub signature: String,

    // Adversarial Defense
    #[serde(default = "default_reputation")]
    pub reputation: f32,
}

fn default_digitizer() -> String {
    "System".to_string()
}

fn default_signature() -> String {
    "unsigned".to_string()
}

fn default_reputation() -> f32 {
    0.5
}

impl Scholar {
    #[must_use] 
    pub fn new(name: String, death_date: String, school: String) -> Self {
        Self {
            id: None,
            name,
            death_date,
            school,
            digitized_by: "System".to_string(),
            signature: "unsigned".to_string(),
            reputation: 0.5,
        }
    }
}

impl Attributable for Scholar {
    fn source(&self) -> String {
        self.name.clone()
    }
}

impl DigitalProvenance for Scholar {
    fn digitized_by(&self) -> String {
        self.digitized_by.clone()
    }
    fn signature(&self) -> String {
        self.signature.clone()
    }
}

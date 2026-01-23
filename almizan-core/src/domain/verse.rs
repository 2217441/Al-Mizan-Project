use crate::domain::traits::{Attributable, DigitalProvenance};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
#[allow(dead_code)]
pub struct Verse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,

    #[validate(length(min = 1))]
    pub text: String,

    #[serde(default)]
    pub uthmani_script: String,
    #[serde(default)]
    pub indopak_script: String,
    #[serde(default)]
    pub simple_clean: String,
    #[serde(default)]
    pub buckwalter: String,

    #[validate(range(min = 1, max = 114))]
    pub revelation_order: u32,

    // Mappings are fetched separately or joined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<VerseMapping>>,

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

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct VerseMapping {
    pub system: String, // e.g., "kufic"
    pub surah: u8,
    pub ayah: u16,
}

#[allow(dead_code)]
impl Verse {
    #[must_use] 
    pub fn new(text: String, revelation_order: u32) -> Self {
        Self {
            id: None,
            text,
            uthmani_script: String::new(),
            indopak_script: String::new(),
            simple_clean: String::new(),
            buckwalter: String::new(),
            revelation_order,
            mappings: None,
            digitized_by: "System".to_string(),
            signature: "unsigned".to_string(),
        }
    }
}

impl Attributable for Verse {
    fn source(&self) -> String {
        "Divine Revelation (Quran)".to_string()
    }
}

impl DigitalProvenance for Verse {
    fn digitized_by(&self) -> String {
        self.digitized_by.clone()
    }
    fn signature(&self) -> String {
        self.signature.clone()
    }
}

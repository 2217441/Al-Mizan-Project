use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Strictness {
    #[default]
    Strict,
    Moderate,
    Lenient,
}

use std::str::FromStr;

impl FromStr for Strictness {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "loose" | "lenient" => Self::Lenient,
            "moderate" => Self::Moderate,
            _ => Self::Strict,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum StrictnessLevel {
    Basic, // Retail
    #[default]
    Standard, // SME (Default)
    High,  // Corporate
    Enterprise, // MnC
}

pub struct Logger;

impl Logger {
    pub fn log_audit(topic: &str, strictness: &Strictness) {
        if *strictness == Strictness::Lenient {
            println!(
                "AUDIT LOG: Client requested PERMISSIVE strictness for topic '{}'. Liability Waiver Active. Time: {}",
                topic,
                chrono::Utc::now().to_rfc3339()
            );
        }
    }

    pub fn log_commercial_audit(topic: &str, level: &StrictnessLevel) {
        println!(
            "COMMERCIAL AUDIT: Topic '{}' accessed with level '{:?}'. Time: {}",
            topic,
            level,
            chrono::Utc::now().to_rfc3339()
        );
    }
}

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
        // SECURITY: Sanitize input to prevent log injection (CWE-117)
        let sanitized_topic = topic.replace(|c: char| c.is_control(), " ");
        if *strictness == Strictness::Lenient {
            // Note: println! can be dangerous if logs are aggregated.
            // In a production app, use tracing or log crate, but we keep println! for now to match style,
            // with sanitized input.
            println!(
                "AUDIT LOG: Client requested PERMISSIVE strictness for topic '{}'. Liability Waiver Active. Time: {}",
                sanitized_topic,
                chrono::Utc::now().to_rfc3339()
            );
        }
    }

    pub fn log_commercial_audit(topic: &str, level: &StrictnessLevel) {
        // SECURITY: Sanitize input to prevent log injection (CWE-117)
        let sanitized_topic = topic.replace(|c: char| c.is_control(), " ");
        println!(
            "COMMERCIAL AUDIT: Topic '{}' accessed with level '{:?}'. Time: {}",
            sanitized_topic,
            level,
            chrono::Utc::now().to_rfc3339()
        );
    }
}

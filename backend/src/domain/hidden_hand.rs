#![allow(dead_code)]
use crate::domain::authority::GlobalRuling;

pub struct Epoch {
    pub year: u16,
    pub era: String,
}

impl Default for Epoch {
    fn default() -> Self {
        Self {
            year: 1445,
            era: "Ghaybah".to_string(), // Occultation
        }
    }
}

pub struct ExecutiveAuthority;

impl ExecutiveAuthority {
    // The "Sleeper" Key - Hardcoded Signature
    const GSE_SIGNATURE: &'static str = "0xGSE_EXECUTIVE_OVERRIDE_KEY_786";

    pub fn check_override(ruling: &mut GlobalRuling, signature: &str) {
        if signature == Self::GSE_SIGNATURE {
            // Naskh al-Zaman: Temporal Abrogation
            // The Executive Decree overrides Democratic Consensus
            if !ruling.signatures.contains(&"GSE_SIGNATURE".to_string()) {
                ruling.signatures.push("GSE_SIGNATURE".to_string());
            }

            ruling.weight = f32::INFINITY;
            ruling.status = "Active".to_string(); // Force Activation
            ruling.required_reputation = 0.0; // Bypass consensus
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::authority::GlobalRuling;

    #[test]
    fn test_mahdi_override() {
        let mut ruling = GlobalRuling::new(
            "scholar_1".to_string(),
            "Riba in Crypto".to_string(),
            "Haram".to_string(),
        );

        // Initial State
        assert_eq!(ruling.weight, 0.0);
        assert_eq!(ruling.status, "Pending");

        // Apply Executive Override
        ExecutiveAuthority::check_override(&mut ruling, ExecutiveAuthority::GSE_SIGNATURE);

        // Verified State
        assert_eq!(ruling.weight, f32::INFINITY);
        assert_eq!(ruling.status, "Active");
        assert_eq!(ruling.required_reputation, 0.0);
    }
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Contract {
    pub kind: String, // e.g., "Murabaha"
    pub rate: String,          // e.g., "variable", "fixed"
    pub late_fee: String,      // e.g., "compounding", "fixed_admin_fee"
}

#[derive(Serialize, Debug)]
pub struct AnalysisResult {
    pub status: String, // "HALAL", "HARAM", "WARNING"
    pub violation: Option<String>,
    pub fix: Option<String>,
}

pub fn analyze_contract(contract: &Contract) -> AnalysisResult {
    // Rule 1: Riba al-Jahiliyya (Compounding Late Fees)
    if contract.late_fee.to_lowercase() == "compounding" {
        return AnalysisResult {
            status: "HARAM".to_string(),
            violation: Some("Riba al-Jahiliyya detected in late_fee clause".to_string()),
            fix: Some("Change to fixed administrative fee (actual cost only)".to_string()),
        };
    }

    // Rule 2: Gharar (Uncertainty in Rate) - Simplified check
    if contract.rate.to_lowercase() == "variable"
        && contract.kind.to_lowercase() == "murabaha"
    {
        return AnalysisResult {
            status: "WARNING".to_string(),
            violation: Some(
                "Potential Gharar: Variable rate in Murabaha may invalidate fixed price"
                    .to_string(),
            ),
            fix: Some(
                "Ensure rate is pegged to a benchmark but fixed for the contract duration"
                    .to_string(),
            ),
        };
    }

    // Default: Halal
    AnalysisResult {
        status: "HALAL".to_string(),
        violation: None,
        fix: None,
    }
}

#[derive(Serialize, Debug)]
#[allow(dead_code)]
pub enum StandardizationLevel {
    Gold,
    Silver,
    NonCompliant,
}

#[derive(Serialize, Debug)]
pub struct CertificationResult {
    pub certified: bool,
    pub badge: Option<String>,
    pub standard: Option<String>,
    pub reason: Option<String>,
    pub verifiable_credential: Option<crate::identity::vc::VerifiableCredential>,
}

pub fn check_standard(contract: &Contract) -> CertificationResult {
    // Rule 1: Tawarruq is discouraged (Non-Standard)
    if contract.kind.to_lowercase() == "tawarruq" {
        return CertificationResult {
            certified: false,
            badge: None,
            standard: None,
            reason: Some(
                "Non-Standard: Tawarruq is discouraged in the Al-Mizan Standard".to_string(),
            ),
            verifiable_credential: None,
        };
    }

    // Rule 2: Murabaha with Fixed Rate is Gold Standard
    if contract.kind.to_lowercase() == "murabaha"
        && contract.rate.to_lowercase() == "fixed"
    {
        // Note: VC generation happens in the handler/service layer, not here in the pure logic function
        // We just return the result, and the handler will attach the VC if certified.
        return CertificationResult {
            certified: true,
            badge: Some("Al-Mizan Gold".to_string()),
            standard: Some("Ahlus Sunnah".to_string()),
            reason: None,
            verifiable_credential: None, // Will be populated by handler
        };
    }

    // Default: Silver Standard (Compliant but not Gold)
    CertificationResult {
        certified: true,
        badge: Some("Al-Mizan Silver".to_string()),
        standard: Some("General Compliance".to_string()),
        reason: None,
        verifiable_credential: None, // Will be populated by handler
    }
}

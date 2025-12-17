use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GlobalRuling {
    pub authority_id: String, // Proposer
    pub topic: String,
    pub verdict: String,
    pub effective_date: String,
    pub signatures: Vec<String>,
    pub status: String,           // "Pending", "Probationary", "Active", "Slashed"
    pub required_reputation: f32, // Changed from required_signatures
    pub current_reputation: f32,
    pub weight: f32,
    pub probation_end: Option<String>,
}

impl GlobalRuling {
    pub fn new(authority_id: String, topic: String, verdict: String) -> Self {
        Self {
            authority_id,
            topic,
            verdict,
            effective_date: "2025-12-03T21:00:00Z".to_string(),
            signatures: Vec::new(),
            status: "Pending".to_string(),
            required_reputation: 20.0, // e.g., 40 scholars * 0.5 avg reputation
            current_reputation: 0.0,
            weight: 0.0,
            probation_end: None,
        }
    }

    // In a real system, we'd look up the signer's reputation.
    // Here we accept it as an argument for the domain logic simulation.
    pub fn sign(&mut self, signature: String, reputation: f32) {
        if !self.signatures.contains(&signature) {
            self.signatures.push(signature);
            self.current_reputation += reputation;
        }
        self.check_status();
    }

    fn check_status(&mut self) {
        if self.current_reputation >= self.required_reputation {
            // Stake & Slash: Enter Probation first
            self.status = "Probationary".to_string();
            // Set probation end to 7 days from now (Mocked for simplicity)
            self.probation_end = Some("2025-12-10T21:00:00Z".to_string());
            self.weight = 0.5; // Partial weight during probation
        }
    }

    #[allow(dead_code)]
    pub fn finalize_probation(&mut self) {
        if self.status == "Probationary" {
            // In real logic, check if now > probation_end
            self.status = "Active".to_string();
            self.weight = 1.0;
        }
    }

    pub fn slash_authority(&mut self) {
        // The "Slash": Burn reputation
        self.status = "Slashed".to_string();
        self.weight = -1.0; // Negative weight indicates treachery
        self.signatures.clear(); // Void all support
        self.current_reputation = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reputation_weighted_consensus() {
        let mut ruling = GlobalRuling::new(
            "auth".to_string(),
            "topic".to_string(),
            "verdict".to_string(),
        );

        // 1. Low Reputation Signers (0.1) - Need 200 of them
        for i in 0..10 {
            ruling.sign(format!("low_sig_{}", i), 0.1);
        }
        assert_eq!(ruling.status, "Pending");
        // Floating point comparison fix
        println!("Current Reputation: {}", ruling.current_reputation);
        assert!((ruling.current_reputation - 1.0).abs() < 0.0001);

        // 2. High Reputation Signer (Grand Mufti - 10.0)
        ruling.sign("grand_mufti".to_string(), 10.0);
        assert_eq!(ruling.status, "Pending");
        assert_eq!(ruling.current_reputation, 11.0);

        // 3. Another High Reputation Signer (10.0) -> Crosses 20.0 threshold
        ruling.sign("grand_ayatollah".to_string(), 10.0);
        assert_eq!(ruling.status, "Probationary");
        assert_eq!(ruling.current_reputation, 21.0);
    }

    #[test]
    fn test_slash_mechanism() {
        let mut ruling = GlobalRuling::new(
            "traitor".to_string(),
            "poison".to_string(),
            "halal".to_string(),
        );
        ruling.status = "Active".to_string();
        ruling.weight = 1.0;

        ruling.slash_authority();

        assert_eq!(ruling.status, "Slashed");
        assert_eq!(ruling.weight, -1.0);
        assert!(ruling.signatures.is_empty());
    }
}

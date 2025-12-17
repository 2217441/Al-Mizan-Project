use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifiableCredential {
    pub context: Vec<String>,
    pub id: String,
    pub r#type: Vec<String>,
    pub issuer: String,
    pub issuance_date: String,
    pub credential_subject: CredentialSubject,
    pub proof: Proof,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CredentialSubject {
    pub id: String,
    pub al_mizan_certification: CertificationClaims,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CertificationClaims {
    pub standard: String,
    pub badge: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Proof {
    pub r#type: String,
    pub created: String,
    pub verification_method: String,
    pub proof_purpose: String,
    pub jws: String,
}

pub fn issue_credential(subject_did: &str, claims: CertificationClaims) -> VerifiableCredential {
    // Mock Issuer DID (The Waqf)
    let issuer_did = "did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp";

    VerifiableCredential {
        context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
        id: format!("urn:uuid:{}", uuid::Uuid::new_v4()),
        r#type: vec![
            "VerifiableCredential".to_string(),
            "AlMizanCertification".to_string(),
        ],
        issuer: issuer_did.to_string(),
        issuance_date: chrono::Utc::now().to_rfc3339(),
        credential_subject: CredentialSubject {
            id: subject_did.to_string(),
            al_mizan_certification: claims,
        },
        proof: Proof {
            r#type: "Ed25519Signature2020".to_string(),
            created: chrono::Utc::now().to_rfc3339(),
            verification_method: format!(
                "{}#z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp",
                issuer_did
            ),
            proof_purpose: "assertionMethod".to_string(),
            jws: "eyJhbGciOiJFZERTQSIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19..mock_signature"
                .to_string(),
        },
    }
}

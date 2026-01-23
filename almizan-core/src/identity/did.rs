use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DidDocument {
    pub id: String,
    pub verification_method: Vec<VerificationMethod>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerificationMethod {
    pub id: String,
    pub r#type: String,
    pub controller: String,
    pub public_key_multibase: String,
}

pub fn generate_did_key() -> DidDocument {
    // Mock DID Key generation
    // In reality, this would generate a key pair and encode the public key in multibase
    let did = "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK";
    DidDocument {
        id: did.to_string(),
        verification_method: vec![VerificationMethod {
            id: format!("{did}#z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"),
            r#type: "Ed25519VerificationKey2020".to_string(),
            controller: did.to_string(),
            public_key_multibase: "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
        }],
    }
}

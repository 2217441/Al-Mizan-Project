use crate::identity::{did, vc};
use axum::{extract::Path, response::Json};

pub async fn resolve_did(Path(_did): Path<String>) -> Json<did::DidDocument> {
    // In a real system, we would look up the DID.
    // For now, we return a mock generated DID Document.
    let doc = did::generate_did_key();
    Json(doc)
}

pub async fn verify_vc(Json(vc): Json<vc::VerifiableCredential>) -> Json<bool> {
    // Mock Verification Logic
    // In reality, we would check the signature against the issuer's public key from their DID Document
    let is_valid = vc.issuer.starts_with("did:key:");
    Json(is_valid)
}

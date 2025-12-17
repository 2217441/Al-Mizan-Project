use crate::domain::authority::GlobalRuling;
use axum::{extract::Json, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProposeRulingRequest {
    pub topic: String,
    pub verdict: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct SignRulingRequest {
    #[allow(dead_code)]
    pub ruling_id: String, // In mock, we'll just simulate signing the last one
    pub signature: String,
}

// Mock storage for the session
use lazy_static::lazy_static;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    static ref MOCK_RULING: Arc<Mutex<Option<GlobalRuling>>> = Arc::new(Mutex::new(None));
}

pub async fn propose_ruling(Json(payload): Json<ProposeRulingRequest>) -> impl IntoResponse {
    let ruling = GlobalRuling::new(
        "scholar:proposer".to_string(),
        payload.topic,
        payload.verdict,
    );

    let mut store = MOCK_RULING.lock().unwrap();
    *store = Some(ruling.clone());

    Json(ruling)
}

pub async fn sign_ruling(Json(payload): Json<SignRulingRequest>) -> impl IntoResponse {
    let mut store = MOCK_RULING.lock().unwrap();

    if let Some(ref mut ruling) = *store {
        // In a real system, we'd fetch the scholar's reputation from the DB.
        // For now, we assume a default reputation of 0.5 for API signers.
        ruling.sign(payload.signature, 0.5);
        return Json(Some(ruling.clone()));
    }

    Json(None)
}

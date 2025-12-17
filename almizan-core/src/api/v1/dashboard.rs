use axum::{extract::Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DashboardRequest {
    pub auth_token: String, // Mock Auth Token (Role)
}

#[derive(Serialize)]
pub struct DashboardResponse {
    pub dashboard_title: String,
    pub modules: Vec<String>,
}

pub async fn get_dashboard(Json(payload): Json<DashboardRequest>) -> impl IntoResponse {
    // Role-Based Semantic Camouflage
    // If Role == MUJTAHID (via token), show Theological Labels.
    // Else, show Corporate/Secular Labels.

    let is_mujtahid = payload.auth_token == "MUJTAHID_KEY_786";

    let (title, modules) = if is_mujtahid {
        (
            "Eschatology Command Center (The Eye)".to_string(),
            vec![
                "Prophecy Tracker (Euphrates)".to_string(),
                "Army of Khorasan Monitor".to_string(),
                "Dajjalic System Analysis".to_string(),
            ],
        )
    } else {
        (
            "Strategic Foresight Dashboard".to_string(),
            vec![
                "Predictive Geopolitical Indicators".to_string(),
                "Regional Stability Monitor".to_string(),
                "Global Systemic Risk Analysis".to_string(),
            ],
        )
    };

    let response = DashboardResponse {
        dashboard_title: title,
        modules,
    };

    Json(response)
}

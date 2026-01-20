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

    // SECURITY: ADMIN_DASHBOARD_TOKEN must be set in production
    let admin_token = std::env::var("ADMIN_DASHBOARD_TOKEN").unwrap_or_else(|_| {
        if std::env::var("RUST_ENV").unwrap_or_default() == "production" {
            panic!("ADMIN_DASHBOARD_TOKEN must be set in production environment");
        }
        tracing::warn!("Using insecure dev admin token - DO NOT USE IN PRODUCTION");
        "MUJTAHID_KEY_786".to_string()
    });

    let is_mujtahid = constant_time_eq(&payload.auth_token, &admin_token);

    // Enterprise Integration: Calculate Trust Metrics
    // In a real scenario, this would aggregate across the network.
    let metric = crate::enterprise::analytics::calculate_trust_metrics("scholar_sys_001");
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let system_trust = format!("{}%", (metric.reliability_score * 100.0) as u32);

    let (title, modules) = if is_mujtahid {
        (
            format!("Theological Depth Dashboard (Admin) - Trust: {system_trust}"),
            vec![
                "Narrative Trend Analysis".to_string(),
                "Source Reliability Index".to_string(),
                "Systemic Bias Analysis".to_string(),
            ],
        )
    } else {
        (
            format!("Public Research Dashboard - Trust: {system_trust}"),
            vec![
                "Dataset Statistics".to_string(),
                "Ontology Coverage Map".to_string(),
                "Recent Ingestions".to_string(),
            ],
        )
    };

    let response = DashboardResponse {
        dashboard_title: title,
        modules,
    };

    Json(response)
}

/// Constant-time string comparison to prevent timing attacks.
/// Returns true if strings are equal, false otherwise.
/// Note: This implementation may leak length differences, which is generally acceptable for this use case.
fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut result = 0;

    for (x, y) in a_bytes.iter().zip(b_bytes.iter()) {
        result |= x ^ y;
    }

    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq("secret", "secret"));
        assert!(!constant_time_eq("secret", "wrong"));
        assert!(!constant_time_eq("secret", "secre"));
        assert!(!constant_time_eq("secret", "secret_extra"));
        assert!(!constant_time_eq("", "secret"));
        assert!(constant_time_eq("", ""));
    }
}

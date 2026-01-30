use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct DashboardRequest {
    #[validate(length(max = 1024))]
    pub auth_token: String, // Mock Auth Token (Role)
}

#[derive(Serialize)]
pub struct DashboardResponse {
    pub dashboard_title: String,
    pub modules: Vec<String>,
}

pub async fn get_dashboard(Json(payload): Json<DashboardRequest>) -> Result<impl IntoResponse, StatusCode> {
    if payload.validate().is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Role-Based Semantic Camouflage
    // If Role == MUJTAHID (via token), show Theological Labels.
    // Else, show Corporate/Secular Labels.

    // SECURITY: ADMIN_DASHBOARD_TOKEN must be set in production
    let admin_token = std::env::var("ADMIN_DASHBOARD_TOKEN")
        .expect("ADMIN_DASHBOARD_TOKEN environment variable must be set");

    // SECURITY: Iterate over the secret (admin_token) to prevent timing attacks based on input length
    let is_mujtahid = constant_time_eq(&admin_token, &payload.auth_token);

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

    Ok(Json(response))
}

/// Constant-time string comparison to prevent timing attacks.
///
/// `secret` must be the first argument to ensure the loop runs for a constant time (relative to secret length).
fn constant_time_eq(secret: &str, input: &str) -> bool {
    let mut result = 0;

    // Check if lengths differ, but do NOT return early.
    if secret.len() != input.len() {
        result = 1;
    }

    let secret_bytes = secret.as_bytes();
    let input_bytes = input.as_bytes();

    // Iterate over the secret to ensure constant time relative to the secret
    for (i, &s_byte) in secret_bytes.iter().enumerate() {
        // Safe indexing: if input is shorter, compare against a varied byte (s_byte ^ 1)
        // This ensures the loop runs secret.len() times regardless of input.len()
        let i_byte = *input_bytes.get(i).unwrap_or(&(s_byte ^ 1));
        result |= s_byte ^ i_byte;
    }

    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_time_eq() {
        // Equal
        assert!(constant_time_eq("secret", "secret"));

        // Content mismatch
        assert!(!constant_time_eq("secret", "wrong1"));

        // Length mismatch
        assert!(!constant_time_eq("secret", "secrett")); // Longer
        assert!(!constant_time_eq("secret", "secre"));   // Shorter
        assert!(!constant_time_eq("secret", "longsecretstring")); // Much longer
        assert!(!constant_time_eq("secret", "s")); // Very short

        // Edge cases
        assert!(!constant_time_eq("", "secret"));
        assert!(!constant_time_eq("secret", ""));
        assert!(constant_time_eq("", ""));

        // Partial matches that shouldn't pass
        assert!(!constant_time_eq("secret", "secreX"));
    }
}

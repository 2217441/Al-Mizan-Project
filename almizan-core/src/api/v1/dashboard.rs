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
    let admin_token = std::env::var("ADMIN_DASHBOARD_TOKEN")
        .expect("ADMIN_DASHBOARD_TOKEN environment variable must be set");

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

    Json(response)
}

/// Constant-time string comparison to prevent timing attacks.
///
/// `a` must be the secret (constant length), and `b` the user input.
fn constant_time_eq(a: &str, b: &str) -> bool {
    let mut result = 0;

    // Check if lengths differ, but do NOT return early.
    if a.len() != b.len() {
        result = 1;
    }

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    // Iterate over the secret (a) to ensure constant time relative to the secret
    for (i, &a_byte) in a_bytes.iter().enumerate() {
        // Safe indexing: if b is shorter, compare against a varied byte (a_byte ^ 1)
        // This ensures the loop runs a.len() times regardless of b.len()
        let b_byte = *b_bytes.get(i).unwrap_or(&(a_byte ^ 1));
        result |= a_byte ^ b_byte;
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

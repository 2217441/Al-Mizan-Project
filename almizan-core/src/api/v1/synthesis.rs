use crate::domain::compliance::{Logger, Strictness, StrictnessLevel};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct SynthesisRequest {
    #[validate(length(min = 1, max = 200))]
    pub topic: String,
    #[validate(length(max = 20))]
    pub strictness: Option<String>,       // "Strict", "Permissive"
    #[validate(length(max = 20))]
    pub strictness_level: Option<String>, // "Basic", "Standard", "High", "Extreme"
}

#[derive(Serialize)]
pub struct SynthesisResponse<'a> {
    #[serde(rename = "@context")]
    pub context: Cow<'a, str>,
    #[serde(rename = "@type")]
    pub type_: Cow<'a, str>,
    pub status: Cow<'a, str>,
    pub ruling_status: Cow<'a, str>,
    pub consensus_score: f32,
    pub summary: Cow<'a, str>,
    pub primary_scholar: Cow<'a, str>,
    pub scholar_avatar: Cow<'a, str>,
}

pub async fn synthesize_topic(Json(payload): Json<SynthesisRequest>) -> impl IntoResponse {
    // 0. Validate Input
    if payload.validate().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Invalid input"})),
        )
            .into_response();
    }

    // Domain Logic: Parse Strictness
    let strictness_mode = match payload.strictness.as_deref() {
        Some("loose") => Strictness::Lenient,
        _ => Strictness::Strict,
    };

    // Domain Logic: Parse StrictnessLevel (Commercial)
    let commercial_level = match payload.strictness_level.as_deref() {
        Some("basic") => StrictnessLevel::Basic,
        Some("high") => StrictnessLevel::High,
        Some("extreme") => StrictnessLevel::Enterprise,
        _ => StrictnessLevel::Standard,
    };

    // AUDIT LOG: Critical for Liability
    Logger::log_audit(&payload.topic, &strictness_mode);
    Logger::log_commercial_audit(&payload.topic, &commercial_level);

    let (status, score, summary, scholar, avatar, ruling_status) = match payload
        .topic
        .to_lowercase()
        .as_str()
    {
        "bitcoin" => {
            if strictness_mode == Strictness::Lenient {
                (
                    Cow::Borrowed("Green"),
                    0.6,
                    Cow::Borrowed("Permissible (Minority/Loose). Some scholars view it as a digital asset. CAUTION: This is a minority opinion."),
                    Cow::Borrowed("Sheikh Joe Crypto (Modernist)"),
                    Cow::Borrowed("https://api.dicebear.com/7.x/shapes/svg?seed=Joe"),
                    Cow::Borrowed("http://schema.org/Approved"),
                )
            } else {
                (
                    Cow::Borrowed("Yellow"),
                    0.4, // Below 0.5 for strict
                    Cow::Borrowed("Disputed (Strict Default). Significant scholarly disagreement regarding Gharar and lack of intrinsic value. Proceed with caution."),
                    Cow::Borrowed("Imam Al-Ghazali (Derived)"),
                    Cow::Borrowed("https://api.dicebear.com/7.x/shapes/svg?seed=Ghazali"),
                    Cow::Borrowed("http://schema.org/Pending"),
                )
            }
        }
        "riba" => (
            Cow::Borrowed("Red"),
            0.0,
            Cow::Borrowed("Major Prohibition (Consensus). Riba is universally prohibited in all its forms."),
            Cow::Borrowed("The Four Imams (Consensus)"),
            Cow::Borrowed("https://api.dicebear.com/7.x/shapes/svg?seed=Consensus"),
            Cow::Borrowed("http://schema.org/Rejected"),
        ),
        "gold" => (
            Cow::Borrowed("Green"),
            1.0,
            Cow::Borrowed("Permissible (Majority). Gold is the standard of value in Islamic Finance."),
            Cow::Borrowed("Imam Malik"),
            Cow::Borrowed("https://api.dicebear.com/7.x/shapes/svg?seed=Malik"),
            Cow::Borrowed("http://schema.org/Approved"),
        ),
        _ => (
            Cow::Borrowed("Yellow"),
            0.5,
            Cow::Borrowed("Topic analysis in progress. Consensus not yet reached."),
            Cow::Borrowed("Al-Mizan Synthesis Engine"),
            Cow::Borrowed("https://api.dicebear.com/7.x/shapes/svg?seed=Mizan"),
            Cow::Borrowed("http://schema.org/Pending"),
        ),
    };

    let response = SynthesisResponse {
        context: Cow::Borrowed("http://schema.org"),
        type_: Cow::Borrowed("FinancialProduct"),
        status,
        ruling_status,
        consensus_score: score,
        summary,
        primary_scholar: scholar,
        scholar_avatar: avatar,
    };

    // The "Jurisdiction" Disclaimer - Legally Critical
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        "X-Disclaimer",
        "Advisory only. Consult local state Mufti for binding rulings."
            .parse()
            .unwrap(),
    );

    (headers, Json(response)).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synthesis_payload_validation() {
        // Valid case
        let valid = SynthesisRequest {
            topic: "Bitcoin".to_string(),
            strictness: Some("Strict".to_string()),
            strictness_level: Some("Standard".to_string()),
        };
        assert!(valid.validate().is_ok());

        // Invalid: Topic too long
        let long_topic = SynthesisRequest {
            topic: "a".repeat(201), // 201 > 200
            strictness: None,
            strictness_level: None,
        };
        assert!(long_topic.validate().is_err());

        // Invalid: Topic empty
        let empty_topic = SynthesisRequest {
            topic: "".to_string(),
            strictness: None,
            strictness_level: None,
        };
        assert!(empty_topic.validate().is_err());

        // Invalid: Strictness too long
        let long_strictness = SynthesisRequest {
            topic: "Bitcoin".to_string(),
            strictness: Some("a".repeat(21)),
            strictness_level: None,
        };
        assert!(long_strictness.validate().is_err());
    }
}

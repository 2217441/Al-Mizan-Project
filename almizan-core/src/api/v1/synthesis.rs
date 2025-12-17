use crate::domain::compliance::{Logger, Strictness, StrictnessLevel};
use axum::{extract::Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SynthesisRequest {
    pub topic: String,
    pub strictness: Option<String>,       // "Strict", "Permissive"
    pub strictness_level: Option<String>, // "Basic", "Standard", "High", "Extreme"
}

#[derive(Serialize)]
pub struct SynthesisResponse {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub type_: String,
    pub status: String,        // "Red", "Yellow", "Green"
    pub ruling_status: String, // "http://schema.org/Approved", "http://schema.org/Rejected"
    pub consensus_score: f32,
    pub summary: String,
    pub primary_scholar: String,
    pub scholar_avatar: String,
}

pub async fn synthesize_topic(Json(payload): Json<SynthesisRequest>) -> impl IntoResponse {
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
                    "Green".to_string(),
                    0.6,
                    "Permissible (Minority/Loose). Some scholars view it as a digital asset. CAUTION: This is a minority opinion.".to_string(),
                    "Sheikh Joe Crypto (Modernist)".to_string(),
                    "https://api.dicebear.com/7.x/avataaars/svg?seed=Joe".to_string(),
                    "http://schema.org/Approved".to_string(),
                )
            } else {
                (
                    "Yellow".to_string(),
                    0.4, // Below 0.5 for strict
                    "Disputed (Strict Default). Significant scholarly disagreement regarding Gharar and lack of intrinsic value. Proceed with caution.".to_string(),
                    "Imam Al-Ghazali (Derived)".to_string(),
                    "https://api.dicebear.com/7.x/avataaars/svg?seed=Ghazali".to_string(),
                    "http://schema.org/Pending".to_string(),
                )
            }
        }
        "riba" => (
            "Red".to_string(),
            0.0,
            "Major Prohibition (Consensus). Riba is universally prohibited in all its forms."
                .to_string(),
            "The Four Imams (Consensus)".to_string(),
            "https://api.dicebear.com/7.x/avataaars/svg?seed=Consensus".to_string(),
            "http://schema.org/Rejected".to_string(),
        ),
        "gold" => (
            "Green".to_string(),
            1.0,
            "Permissible (Majority). Gold is the standard of value in Islamic Finance.".to_string(),
            "Imam Malik".to_string(),
            "https://api.dicebear.com/7.x/avataaars/svg?seed=Malik".to_string(),
            "http://schema.org/Approved".to_string(),
        ),
        _ => (
            "Yellow".to_string(),
            0.5,
            "Topic analysis in progress. Consensus not yet reached.".to_string(),
            "Al-Mizan Synthesis Engine".to_string(),
            "https://api.dicebear.com/7.x/bottts/svg?seed=Mizan".to_string(),
            "http://schema.org/Pending".to_string(),
        ),
    };

    let response = SynthesisResponse {
        context: "http://schema.org".to_string(),
        type_: "FinancialProduct".to_string(),
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

    (headers, Json(response))
}

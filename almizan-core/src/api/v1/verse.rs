use super::utils::serialize_thing_id;
use crate::repository::db::Database;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct VerseResponse {
    #[serde(serialize_with = "serialize_thing_id")]
    id: surrealdb::sql::Thing,
    surah: i32,
    ayah: i32,
    text_uthmani: String,
    juz: i32,
    place: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    roots: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
struct DbVerse {
    id: surrealdb::sql::Thing,
    surah_number: i32,
    ayah_number: i32,
    text_uthmani: String,
    juz_number: Option<i32>,
    revelation_place: Option<String>,
    roots: Option<Vec<String>>,
}

impl Default for DbVerse {
    fn default() -> Self {
        Self {
            id: surrealdb::sql::Thing::from(("quran_verse", "0_0")),
            surah_number: 0,
            ayah_number: 0,
            text_uthmani: String::new(),
            juz_number: None,
            revelation_place: None,
            roots: None,
        }
    }
}

#[derive(Deserialize)]
pub struct VerseQuery {
    #[serde(default)]
    include_roots: bool,
}

/// GET /api/v1/verse/{surah}/{ayah}
/// Get a specific verse by surah and ayah number
pub async fn get_verse(
    State(db): State<Database>,
    Path((surah, ayah)): Path<(i32, i32)>,
    Query(params): Query<VerseQuery>,
) -> impl IntoResponse {
    // Optimization: Combined verse and roots fetch into a single query using conditional projection
    // This eliminates the N+1 query pattern when roots are requested
    let sql = "
        SELECT
            id,
            surah_number,
            ayah_number,
            text_uthmani,
            juz_number,
            revelation_place,
            (IF $include_roots THEN ->has_root->root_word.root_ar ELSE NONE END) AS roots
        FROM quran_verse
        WHERE surah_number = $surah AND ayah_number = $ayah
    ";

    let result: Result<Vec<DbVerse>, _> = db
        .client
        .query(sql)
        .bind(("surah", surah))
        .bind(("ayah", ayah))
        .bind(("include_roots", params.include_roots))
        .await
        .and_then(|mut r| r.take(0));

    match result {
        Ok(verses) if !verses.is_empty() => {
            let v = &verses[0];

            Json(VerseResponse {
                id: v.id.clone(),
                surah: v.surah_number,
                ayah: v.ayah_number,
                text_uthmani: v.text_uthmani.clone(),
                juz: v.juz_number.unwrap_or(0),
                place: v.revelation_place.clone().unwrap_or_default(),
                roots: v.roots.clone(),
            })
            .into_response()
        }
        Ok(_) => (
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Verse not found"})),
        )
            .into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// GET /api/v1/verse/{surah}
/// Get all verses in a surah
pub async fn get_surah(State(db): State<Database>, Path(surah): Path<i32>) -> impl IntoResponse {
    // Query all verse IDs for this surah in a single batch
    // Replaced N+1 query loop with single efficient SELECT
    // Optimization: Use explicit column selection to prevent over-fetching and reduce payload size
    let sql = "SELECT id, surah_number, ayah_number, text_uthmani, juz_number, revelation_place FROM quran_verse WHERE surah_number = $surah ORDER BY ayah_number ASC";
    let mut response = match db.client.query(sql).bind(("surah", surah)).await {
        Ok(r) => r,
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
                .into_response()
        }
    };

    let verses: Vec<DbVerse> = match response.take(0) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("Failed to deserialize verses: {}", e);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to process verses"})),
            )
                .into_response();
        }
    };

    let response: Vec<VerseResponse> = verses
        .into_iter()
        .map(|v| VerseResponse {
            id: v.id,
            surah: v.surah_number,
            ayah: v.ayah_number,
            text_uthmani: v.text_uthmani,
            juz: v.juz_number.unwrap_or(0),
            place: v.revelation_place.unwrap_or_default(),
            roots: None,
        })
        .collect();

    Json(serde_json::json!({
        "surah": surah,
        "count": response.len(),
        "verses": response
    }))
    .into_response()
}

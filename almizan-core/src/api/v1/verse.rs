use crate::repository::db::Database;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct VerseResponse {
    id: String,
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
    // Use parameterized query to prevent SQL injection
    let sql = "SELECT id, surah_number, ayah_number, text_uthmani, juz_number, revelation_place FROM quran_verse WHERE surah_number = $surah AND ayah_number = $ayah";

    let result: Result<Vec<DbVerse>, _> = db
        .client
        .query(sql)
        .bind(("surah", surah))
        .bind(("ayah", ayah))
        .await
        .and_then(|mut r| r.take(0));

    match result {
        Ok(verses) if !verses.is_empty() => {
            let v = &verses[0];

            // Optionally get roots
            let roots = if params.include_roots {
                // Vulnerability Fix: Use parameterized queries for roots lookup as well
                let roots_sql = "SELECT ->has_root->root_word.root_ar AS roots FROM type::thing($verse_id)";
                let verse_id = format!("quran_verse:{}_{}", surah, ayah);

                let roots_result: Vec<serde_json::Value> = db
                    .client
                    .query(roots_sql)
                    .bind(("verse_id", verse_id))
                    .await
                    .and_then(|mut r| r.take(0))
                    .unwrap_or_default();

                // Extract roots array from result
                roots_result
                    .first()
                    .and_then(|r| r.get("roots"))
                    .and_then(|r| r.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
            } else {
                None
            };

            Json(VerseResponse {
                id: v.id.to_string(),
                surah: v.surah_number,
                ayah: v.ayah_number,
                text_uthmani: v.text_uthmani.clone(),
                juz: v.juz_number.unwrap_or(0),
                place: v.revelation_place.clone().unwrap_or_default(),
                roots,
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
    let sql = "SELECT * FROM quran_verse WHERE surah_number = $surah ORDER BY ayah_number ASC";
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

    let verses: Vec<DbVerse> = response.take(0).unwrap_or_default();

    let response: Vec<VerseResponse> = verses
        .into_iter()
        .map(|v| VerseResponse {
            id: v.id.to_string(),
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

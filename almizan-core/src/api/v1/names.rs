use crate::repository::db::Database;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Id;

#[derive(Serialize)]
pub struct DivineNameResponse {
    id: i32,
    arabic: String,
    transliteration: String,
    meaning: String,
}

#[derive(Deserialize, Debug)]
struct DbDivineName {
    id: surrealdb::sql::Thing,
    arabic: String,
    transliteration: String,
    meaning_en: String,
}

/// GET /api/v1/names
/// Get all 99 names of Allah
pub async fn get_all_names(State(db): State<Database>) -> impl IntoResponse {
    let sql = "SELECT id, arabic, transliteration, meaning_en FROM divine_name ORDER BY id";

    let result: Result<Vec<DbDivineName>, _> =
        db.client.query(sql).await.and_then(|mut r| r.take(0));

    match result {
        Ok(names) => {
            let response: Vec<DivineNameResponse> = names
                .into_iter()
                .map(|n| {
                    // Extract numeric ID from Thing without string allocation
                    // Optimization: Avoids .to_string() allocation and parsing for numeric IDs
                    let num_id: i32 = match n.id.id {
                        Id::Number(num) => num.try_into().unwrap_or(0),
                        Id::String(s) => s.parse().unwrap_or(0),
                        _ => 0,
                    };
                    DivineNameResponse {
                        id: num_id,
                        arabic: n.arabic,
                        transliteration: n.transliteration,
                        meaning: n.meaning_en,
                    }
                })
                .collect();

            Json(serde_json::json!({
                "count": response.len(),
                "names": response
            }))
            .into_response()
        }
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// GET /api/v1/names/{id}
/// Get a specific divine name by ID
pub async fn get_name(State(db): State<Database>, Path(id): Path<i32>) -> impl IntoResponse {
    let result: Result<Option<DbDivineName>, _> =
        db.client.select(("divine_name", id.to_string())).await;

    match result {
        Ok(Some(n)) => Json(DivineNameResponse {
            id,
            arabic: n.arabic,
            transliteration: n.transliteration,
            meaning: n.meaning_en,
        })
        .into_response(),
        Ok(None) => (
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Name not found"})),
        )
            .into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

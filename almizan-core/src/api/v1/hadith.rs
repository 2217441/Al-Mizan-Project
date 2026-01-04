use crate::repository::db::Database;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HadithResponse {
    id: String,
    collection: String,
    book_number: Option<i32>,
    hadith_number: f64,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    grade: Option<String>,
}

#[derive(Deserialize, Debug)]
struct DbHadith {
    id: surrealdb::sql::Thing,
    collection: String,
    book_number: Option<i32>,
    hadith_number: f64,
    matn_en: Option<String>,
    grade: Option<String>,
}

/// GET /api/v1/hadith/{collection}/{number}
/// Get a specific hadith by collection and number
pub async fn get_hadith(
    State(db): State<Database>,
    Path((collection, number)): Path<(String, f64)>,
) -> impl IntoResponse {
    // Try direct ID lookup first (e.g., hadith:bukhari_1_1)
    let sql = format!(
        "SELECT id, collection, book_number, hadith_number, matn_en, grade FROM hadith WHERE collection = '{}' AND hadith_number = {} LIMIT 1",
        collection, number
    );

    let result: Result<Vec<DbHadith>, _> = db.client.query(&sql).await.and_then(|mut r| r.take(0));

    match result {
        Ok(hadiths) if !hadiths.is_empty() => {
            let h = &hadiths[0];
            Json(HadithResponse {
                id: h.id.to_string(),
                collection: h.collection.clone(),
                book_number: h.book_number,
                hadith_number: h.hadith_number,
                text: h.matn_en.clone().unwrap_or_default(),
                grade: h.grade.clone(),
            })
            .into_response()
        }
        Ok(_) => (
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Hadith not found"})),
        )
            .into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// GET /api/v1/hadith/{collection}
/// List hadiths from a collection (paginated)
pub async fn list_collection(
    State(db): State<Database>,
    Path(collection): Path<String>,
) -> impl IntoResponse {
    let sql = format!(
        "SELECT id, collection, book_number, hadith_number, matn_en, grade FROM hadith WHERE collection = '{}' ORDER BY hadith_number LIMIT 50",
        collection
    );

    let result: Result<Vec<DbHadith>, _> = db.client.query(&sql).await.and_then(|mut r| r.take(0));

    match result {
        Ok(hadiths) => {
            let response: Vec<HadithResponse> = hadiths
                .into_iter()
                .map(|h| HadithResponse {
                    id: h.id.to_string(),
                    collection: h.collection,
                    book_number: h.book_number,
                    hadith_number: h.hadith_number,
                    text: h.matn_en.unwrap_or_default(),
                    grade: h.grade,
                })
                .collect();

            Json(serde_json::json!({
                "collection": collection,
                "count": response.len(),
                "hadiths": response
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

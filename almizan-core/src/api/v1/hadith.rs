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
    text: Option<String>,
    grade: Option<String>,
}

/// GET /api/v1/hadith/{collection}/{number}
/// Get a specific hadith by collection and number
pub async fn get_hadith(
    State(db): State<Database>,
    Path((collection, number)): Path<(String, f64)>,
) -> impl IntoResponse {
    // Use parameterized query to prevent SQL injection
    // Optimization: Select only the necessary text field using conditional logic in the query
    // This reduces payload size and avoids allocating strings for unused text
    let sql = "SELECT id, collection, book_number, hadith_number, IF matn_en != NONE AND matn_en != NULL AND matn_en != '' THEN matn_en ELSE matn_ar END AS text, grade FROM hadith WHERE collection = $collection AND hadith_number = $number LIMIT 1";

    let result: Result<Vec<DbHadith>, _> = db
        .client
        .query(sql)
        .bind(("collection", collection.clone()))
        .bind(("number", number))
        .await
        .and_then(|mut r| r.take(0));

    match result {
        Ok(hadiths) if !hadiths.is_empty() => {
            let h = &hadiths[0];
            Json(HadithResponse {
                id: h.id.to_string(),
                collection: h.collection.clone(),
                book_number: h.book_number,
                hadith_number: h.hadith_number,
                text: h.text.clone().unwrap_or_default(),
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
    // Use parameterized query to prevent SQL injection
    // Optimization: Select only the necessary text field using conditional logic in the query
    let sql = "SELECT id, collection, book_number, hadith_number, IF matn_en != NONE AND matn_en != NULL AND matn_en != '' THEN matn_en ELSE matn_ar END AS text, grade FROM hadith WHERE collection = $collection ORDER BY hadith_number LIMIT 50";

    let result: Result<Vec<DbHadith>, _> = db
        .client
        .query(sql)
        .bind(("collection", collection.clone()))
        .await
        .and_then(|mut r| r.take(0));

    match result {
        Ok(hadiths) => {
            let response: Vec<HadithResponse> = hadiths
                .into_iter()
                .map(|h| HadithResponse {
                    id: h.id.to_string(),
                    collection: h.collection,
                    book_number: h.book_number,
                    hadith_number: h.hadith_number,
                    text: h.text.unwrap_or_default(),
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

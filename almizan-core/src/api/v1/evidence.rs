use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
// use std::sync::Arc;
use crate::repository::db::Database;
// use crate::domain::graph::GraphEngine; // Future integration

pub async fn get_evidence(
    State(db): State<Database>,
    Path(ruling_id): Path<String>,
) -> (StatusCode, Json<Value>) {
    // 1. Construct the Graph Query
    // "Select the nodes that this ruling is derived from"
    // Vulnerability Fix: Use parameterized queries to prevent SQL injection
    let sql = "SELECT * FROM ->DERIVED_FROM->(quran_verse, hadith) WHERE in = $ruling_id";

    // 2. Execute via DB Client
    // Ideally we usage execute_query or similar helper
    let mut response = match db.client.query(sql).bind(("ruling_id", ruling_id.clone())).await {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        }
    };

    // 3. Parse Result
    let result: Vec<Value> = match response.take(0) {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        }
    };

    (
        StatusCode::OK,
        Json(json!({ "ruling_id": ruling_id, "evidence": result })),
    )
}

use askama::Template;
use axum::{http::StatusCode, response::Html, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Template)]
#[template(path = "network.html")]
pub struct NetworkDashboardTemplate;

#[derive(Template)]
#[template(path = "playground.html")]
pub struct PlaygroundTemplate;

pub async fn playground() -> impl IntoResponse {
    let template = PlaygroundTemplate;
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Template render error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Serialize)]
pub struct SnapshotMetadata {
    pub id: String,
    pub timestamp: String,
    pub node_count: u32,
    pub signature: String,
}

pub async fn dashboard() -> impl IntoResponse {
    let template = NetworkDashboardTemplate;
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Template render error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn export_snapshot() -> Json<SnapshotMetadata> {
    // Mock Export Logic
    // In a real implementation, this would query SurrealDB and dump the graph to JSON.
    let metadata = SnapshotMetadata {
        id: "snapshot_v1_mock".to_string(),
        timestamp: "2025-12-03T21:00:00Z".to_string(),
        node_count: 150,
        signature: "sha256:mock_snapshot_signature".to_string(),
    };
    Json(metadata)
}

pub async fn ingest_snapshot() -> &'static str {
    // Mock Ingest Logic
    // In a real implementation, this would parse the uploaded JSON and merge it into SurrealDB.
    "Snapshot Ingested Successfully (Mock)"
}

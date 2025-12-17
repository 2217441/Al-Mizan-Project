use crate::repository::db::Database;
use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct GraphData {
    nodes: Vec<CytoscapeNode>,
    edges: Vec<CytoscapeEdge>,
}

#[derive(Serialize)]
struct CytoscapeNode {
    data: NodeData,
}

#[derive(Serialize)]
struct NodeData {
    id: String,
    label: String,
    r#type: String,
}

#[derive(Serialize)]
struct CytoscapeEdge {
    data: EdgeData,
}

#[derive(Serialize)]
struct EdgeData {
    id: String,
    source: String,
    target: String,
    label: String,
    authority: String,
}

#[derive(Deserialize)]
struct DbNode {
    id: surrealdb::sql::Thing,
    label: String,
    r#type: String,
}

#[derive(Deserialize)]
struct DbEdge {
    id: surrealdb::sql::Thing,
    #[serde(rename = "in")]
    source: surrealdb::sql::Thing,
    #[serde(rename = "out")]
    target: surrealdb::sql::Thing,
    label: String,
    authority: Option<String>,
}

pub async fn get_graph(State(db): State<Database>) -> impl IntoResponse {
    let nodes_result: Vec<DbNode> = db.client.select("node").await.unwrap_or_default();
    let edges_result: Vec<DbEdge> = db.client.select("edge").await.unwrap_or_default();

    let nodes = nodes_result
        .into_iter()
        .map(|n| CytoscapeNode {
            data: NodeData {
                id: n.id.id.to_string(),
                label: n.label,
                r#type: n.r#type,
            },
        })
        .collect();

    let edges = edges_result
        .into_iter()
        .map(|e| CytoscapeEdge {
            data: EdgeData {
                id: e.id.id.to_string(),
                source: e.source.id.to_string(),
                target: e.target.id.to_string(),
                label: e.label,
                authority: e.authority.unwrap_or_else(|| "Unknown".to_string()),
            },
        })
        .collect();

    Json(GraphData { nodes, edges })
}

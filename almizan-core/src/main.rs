use askama::Template;
use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod api;
pub mod domain;
mod enterprise;
mod identity;
mod repository;

use api::auth;
use repository::db::Database;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Initializing Al-Mizan Core...");

    // Initialize Database
    let db = match Database::init().await {
        Ok(db) => {
            tracing::info!("Connected to SurrealDB successfully");
            db
        }
        Err(e) => {
            tracing::error!("Failed to connect to SurrealDB: {}", e);
            std::process::exit(1);
        }
    };

    // Build our application
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/auth/signup", post(auth::signup))
        .route("/auth/signin", post(auth::signin))
        .route("/api/v1/evidence/:id", get(api::v1::evidence::get_evidence))
        // .route(
        //     "/api/v1/synthesis",
        //     post(api::v1::synthesis::synthesize_topic),
        // )
        .route("/api/v1/dashboard", post(api::v1::dashboard::get_dashboard))
        // .route("/api/v1/graph", get(api::v1::graph::get_graph))
        // .route(
        //     "/api/v1/syntax/translate",
        //     post(api::v1::syntax::translate_query),
        // )
        .route("/citadel", get(api::v1::citadel::dashboard))
        .route(
            "/api/v1/citadel/export",
            get(api::v1::citadel::export_snapshot),
        )
        .route(
            "/api/v1/citadel/ingest",
            post(api::v1::citadel::ingest_snapshot),
        )
        // .route(
        //     "/api/v1/authority/propose",
        //     post(api::v1::authority::propose_ruling),
        // )
        // .route(
        //     "/api/v1/authority/sign",
        //     post(api::v1::authority::sign_ruling),
        // )
        // .route(
        //     "/api/v1/enterprise/metrics",
        //     get(api::v1::enterprise::get_metrics),
        // )
        // .route(
        //     "/api/v1/enterprise/audit",
        //     post(api::v1::enterprise::audit_document),
        // )
        // .route(
        //     "/api/v1/enterprise/analyze_contract",
        //     post(api::v1::enterprise::analyze_contract_handler),
        // )
        // .route(
        //     "/api/v1/enterprise/certify",
        //     post(api::v1::enterprise::certify_contract_handler),
        // )
        // .route(
        //     "/api/v1/identity/resolve/:did",
        //     get(api::v1::identity::resolve_did),
        // )
        // .route(
        //     "/api/v1/identity/verify",
        //     post(api::v1::identity::verify_vc),
        // )
        // .route("/strategy", get(api::v1::event::get_events))
        .route("/graph", get(graph_handler))
        .route("/presentation", get(presentation_handler))
        .nest_service("/static", ServeDir::new("static"))
        .route(
            "/favicon.ico",
            get(|| async { axum::http::StatusCode::NO_CONTENT }),
        )
        .with_state(db);

    // Run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index_handler() -> impl axum::response::IntoResponse {
    let template = IndexTemplate;
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "graph.html")]
// Graph Handler
struct GraphTemplate;

async fn graph_handler() -> Html<String> {
    let template = GraphTemplate;
    Html(template.render().unwrap())
}

// Presentation Handler (Placeholder)
async fn presentation_handler() -> Html<String> {
    Html("<h1>Coming Soon: Presentation Mode</h1>".to_string())
}

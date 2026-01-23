#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
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
#[template(path = "certainty-engine.html")]
struct CertaintyEngineTemplate;

#[derive(Template)]
#[template(path = "landing.html")]
struct LandingTemplate;

#[derive(Template)]
#[template(path = "presentation.html")]
struct PresentationTemplate;

#[derive(Template)]
#[template(path = "governance.html")]
struct GovernanceTemplate;

#[derive(Template)]
#[template(path = "components.html")]
struct ComponentsTemplate;

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
            tracing::error!("Failed to connect to SurrealDB: {e}");
            std::process::exit(1);
        }
    };

    // Build and run the application
    let app = create_router(db);
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| {
            tracing::error!("Failed to bind to address {addr}: {e}");
            std::process::exit(1);
        });

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn create_router(db: Database) -> Router<()> {
    Router::new()
        .route("/", get(landing_handler))
        .route("/certainty-engine", get(certainty_engine_handler))
        .route("/auth/signup", post(auth::signup))
        .route("/auth/signin", post(auth::signin))
        .route(
            "/api/v1/evidence/{id}",
            get(api::v1::evidence::get_evidence),
        )
        .route(
            "/api/v1/synthesis",
            post(api::v1::synthesis::synthesize_topic),
        )
        .route("/api/v1/dashboard", post(api::v1::dashboard::get_dashboard))
        .route("/api/v1/graph", get(api::v1::graph::get_graph))
        .route(
            "/api/v1/verse/{surah}/{ayah}",
            get(api::v1::verse::get_verse),
        )
        .route("/api/v1/verse/{surah}", get(api::v1::verse::get_surah))
        .route(
            "/api/v1/hadith/{collection}/{number}",
            get(api::v1::hadith::get_hadith),
        )
        .route(
            "/api/v1/hadith/{collection}",
            get(api::v1::hadith::list_collection),
        )
        .route("/api/v1/names", get(api::v1::names::get_all_names))
        .route("/api/v1/names/{id}", get(api::v1::names::get_name))
        .route("/network", get(api::v1::network::dashboard))
        .route("/playground", get(api::v1::network::playground))
        .route(
            "/api/v1/network/export",
            get(api::v1::network::export_snapshot),
        )
        .route(
            "/api/v1/network/ingest",
            post(api::v1::network::ingest_snapshot),
        )
        .route(
            "/api/v1/enterprise/metrics",
            get(api::v1::enterprise::get_metrics),
        )
        .route(
            "/api/v1/enterprise/audit",
            post(api::v1::enterprise::audit_document),
        )
        .route(
            "/api/v1/enterprise/analyze_contract",
            post(api::v1::enterprise::analyze_contract_handler),
        )
        .route(
            "/api/v1/enterprise/certify",
            post(api::v1::enterprise::certify_contract_handler),
        )
        .route(
            "/api/v1/identity/resolve/{did}",
            get(api::v1::identity::resolve_did),
        )
        .route(
            "/api/v1/identity/verify",
            post(api::v1::identity::verify_vc),
        )
        .route("/graph", get(graph_handler))
        .route("/landing", get(landing_handler))
        .route("/presentation", get(presentation_handler))
        .route("/governance", get(governance_handler))
        .route("/components", get(components_handler))
        .nest_service("/static", ServeDir::new("static"))
        .route(
            "/favicon.ico",
            get(|| async { axum::http::StatusCode::NO_CONTENT }),
        )
        .with_state(db)
}

async fn certainty_engine_handler() -> impl IntoResponse {
    let template = CertaintyEngineTemplate;
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Template render error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Template)]
#[template(path = "graph.html")]
struct GraphTemplate;

async fn graph_handler() -> impl IntoResponse {
    let template = GraphTemplate;
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Template render error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn landing_handler() -> impl IntoResponse {
    let template = LandingTemplate;
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Template render error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn presentation_handler() -> impl IntoResponse {
    let template = PresentationTemplate;
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Template render error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn governance_handler() -> impl IntoResponse {
    let template = GovernanceTemplate;
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Template render error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn components_handler() -> impl IntoResponse {
    let template = ComponentsTemplate;
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Template render error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}


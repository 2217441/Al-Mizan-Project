use crate::domain::event::Event;
use askama::Template;
use axum::{extract::Query, http::StatusCode, response::Html, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EventFilter {
    pub status: Option<String>,
}

#[derive(Template)]
#[template(path = "strategy.html")]
struct StrategyDashboardTemplate {
    events: Vec<Event>,
}

pub async fn get_events(Query(filter): Query<EventFilter>) -> impl IntoResponse {
    // Mock Data
    let all_events = vec![
        Event::new(
            "Drying of Euphrates".to_string(),
            "Pending".to_string(),
            "hadith:bukhari_7119".to_string(),
            "Euphrates River".to_string(),
            "The Hour will not be established until the Euphrates uncovers a mountain of gold."
                .to_string(),
        ),
        Event::new(
            "Conquest of Constantinople".to_string(),
            "Fulfilled".to_string(),
            "hadith:muslim_2920".to_string(),
            "Istanbul".to_string(),
            "Verily you shall conquer Constantinople.".to_string(),
        ),
        Event::new(
            "The Malahim (Great War)".to_string(),
            "Pending".to_string(),
            "hadith:abudawud_4291".to_string(),
            "Dabiq/Amaq".to_string(),
            "The Hour will not be established until the Romans land at A'maq or Dabiq.".to_string(),
        ),
    ];

    let filtered_events = if let Some(status) = filter.status {
        all_events
            .into_iter()
            .filter(|e| e.status.to_lowercase() == status.to_lowercase())
            .collect()
    } else {
        all_events
    };

    let template = StrategyDashboardTemplate {
        events: filtered_events,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Template render error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

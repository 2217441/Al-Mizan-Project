use crate::enterprise::{analytics, compliance};
use axum::{extract::Query, response::Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MetricsRequest {
    pub scholar_id: String,
}

#[derive(Deserialize)]
pub struct AuditRequest {
    pub document_id: String,
}

pub async fn get_metrics(Query(payload): Query<MetricsRequest>) -> Json<analytics::TrustMetric> {
    let metrics = analytics::calculate_trust_metrics(&payload.scholar_id);
    Json(metrics)
}

pub async fn audit_document(
    Json(payload): Json<AuditRequest>,
) -> Json<compliance::ComplianceReport> {
    let report = compliance::generate_compliance_report(&payload.document_id);
    Json(report)
}

pub async fn analyze_contract_handler(
    Json(payload): Json<crate::enterprise::shariah::Contract>,
) -> Json<crate::enterprise::shariah::AnalysisResult> {
    let result = crate::enterprise::shariah::analyze_contract(&payload);
    Json(result)
}

pub async fn certify_contract_handler(
    Json(payload): Json<crate::enterprise::shariah::Contract>,
) -> Json<crate::enterprise::shariah::CertificationResult> {
    let mut result = crate::enterprise::shariah::check_standard(&payload);

    if result.certified {
        let claims = crate::identity::vc::CertificationClaims {
            standard: result.standard.clone().unwrap_or_default(),
            badge: result.badge.clone().unwrap_or_default(),
            status: "Active".to_string(),
        };
        // Mock Subject DID (The Client)
        let subject_did = "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK";
        let vc = crate::identity::vc::issue_credential(subject_did, claims);
        result.verifiable_credential = Some(vc);
    }

    Json(result)
}

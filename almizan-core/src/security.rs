use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};

/// Middleware to add security headers to every response.
pub async fn add_security_headers(req: Request<Body>, next: Next) -> Response<Body> {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    // Prevent MIME type sniffing
    headers.insert(
        "X-Content-Type-Options",
        "nosniff".parse().unwrap(),
    );

    // Clickjacking protection
    headers.insert(
        "X-Frame-Options",
        "SAMEORIGIN".parse().unwrap(),
    );

    // XSS Protection (Legacy but still useful for older browsers)
    headers.insert(
        "X-XSS-Protection",
        "1; mode=block".parse().unwrap(),
    );

    // Control information sent to other sites
    headers.insert(
        "Referrer-Policy",
        "strict-origin-when-cross-origin".parse().unwrap(),
    );

    // Content Security Policy (CSP)
    // Starting with a permissive policy that allows inline scripts/styles as the app currently relies on them.
    headers.insert(
        "Content-Security-Policy",
        "default-src 'self' https: data:; script-src 'self' 'unsafe-inline' https:; style-src 'self' 'unsafe-inline' https:; img-src 'self' data: https:; font-src 'self' data: https:; connect-src 'self' https: ws: wss:;".parse().unwrap(),
    );

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::get, Router};
    use axum::http::StatusCode;
    use tower::util::ServiceExt; // Changed to tower::util::ServiceExt

    #[tokio::test]
    async fn test_security_headers_middleware() {
        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .layer(axum::middleware::from_fn(add_security_headers));

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::from("")).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let headers = response.headers();

        assert_eq!(headers.get("X-Content-Type-Options").unwrap(), "nosniff");
        assert_eq!(headers.get("X-Frame-Options").unwrap(), "SAMEORIGIN");
        assert_eq!(headers.get("X-XSS-Protection").unwrap(), "1; mode=block");
        assert_eq!(headers.get("Referrer-Policy").unwrap(), "strict-origin-when-cross-origin");
        assert!(headers.get("Content-Security-Policy").unwrap().to_str().unwrap().contains("default-src 'self'"));
    }
}

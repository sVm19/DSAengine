use axum::{body::Body, http::Request};
use serde_json::{json, Value};
use tower::ServiceExt;

/// Executes an algorithm tool via an in-process Axum oneshot dispatch.
///
/// This is zero-cost: no TCP socket, no HTTP server, no API key required.
/// It builds the same feature-gated router that the web server uses,
/// then dispatches the request in-memory using `tower::ServiceExt::oneshot`.
pub async fn execute_algorithm_local(name: &str, arguments: Value) -> Result<Value, String> {
    let (category, skill) = name
        .split_once('.')
        .ok_or_else(|| format!("Invalid tool name '{}'. Expected '<category>.<skill>'.", name))?;

    // Build a bare router (no auth middleware — that lives only in web_server.rs)
    let router = crate::skill_routes::register(axum::Router::new());

    // In skill_routes.rs, routes are registered directly without /api/v1/ prefix
    let path = format!("/{}/{}", category, skill);
    let body_bytes = serde_json::to_vec(&arguments).map_err(|e| e.to_string())?;

    let request = Request::builder()
        .method("POST")
        .uri(&path)
        .header("content-type", "application/json")
        .body(Body::from(body_bytes))
        .map_err(|e| e.to_string())?;

    let response = router
        .oneshot(request)
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status().as_u16();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .map_err(|e| e.to_string())?;

    let body: Value = serde_json::from_slice(&bytes).unwrap_or_else(|_| {
        json!({ "raw": String::from_utf8_lossy(&bytes).to_string() })
    });

    Ok(json!({ "status_code": status, "response": body }))
}

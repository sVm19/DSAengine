use axum::extract::Path;
use axum::routing::{get, post};
use axum::{
    http::StatusCode,
    http::{HeaderName, HeaderValue},
    middleware, Json, Router,
};
use dsaengine::mcp_common;
use dsaengine::skill_routes::{self, McpTool};
use dsaengine::utils::api_docs;
use serde_json::{json, Value};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
#[derive(OpenApi)]
#[openapi(
    paths(
        template_skill_handler,
        explore_handler
    ),
    components(
        schemas(
            api_docs::ArrayRequest,
            api_docs::GraphRequest,
            api_docs::TreeRequest,
            api_docs::Complexity,
            api_docs::StandardResponse,
            api_docs::SkillManifest
        )
    ),
    tags(
        (name = "fundamentals"),
        (name = "arrays"),
        (name = "linked_lists"),
        (name = "stacks"),
        (name = "trees"),
        (name = "trees_adv"),
        (name = "graphs"),
        (name = "dp"),
        (name = "greedy"),
        (name = "backtracking"),
        (name = "sorting"),
        (name = "advanced")
    )
)]
struct ApiDoc;

//  The Global Guard (Security Layer)
async fn auth_middleware(
    headers: axum::http::HeaderMap,
    request: axum::extract::Request,
    next: middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let key = headers.get("X-API-KEY").and_then(|k| k.to_str().ok());
    let expected = std::env::var("MASTER_API_2026").ok();

    if let Some(expected_key) = expected.as_deref() {
        if key == Some(expected_key) {
            Ok(next.run(request).await)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        // Fail closed when the API key is not configured.
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/{category}/{skill}",
    params(
        ("category" = String, Path, description = "Skill category", example = "graphs"),
        ("skill" = String, Path, description = "Skill name", example = "dijkstra")
    ),
    request_body(content = api_docs::GraphRequest, content_type = "application/json"),
    responses(
        (status = 200, description = "Generic DSA operation response", body = api_docs::StandardResponse),
        (status = 404, description = "Skill not found")
    ),
    tag = "auto"
)]
pub async fn template_skill_handler(
    Path((category, skill)): Path<(String, String)>,
    Json(payload): Json<serde_json::Value>,
) -> Json<api_docs::StandardResponse> {
    Json(api_docs::StandardResponse {
        status: "error".to_string(),
        engine: "dsaengine".to_string(),
        complexity: None,
        result: Some(serde_json::json!({
            "message": format!("Use a specific endpoint, e.g. /api/v1/{}/{}", category, skill),
            "received": payload
        })),
        description: Some("Received unmatched skill path; use specific skill endpoint.".to_string()),
        before_vs_after: Some("Generic proxy does not run algorithm; invoke specific skill for performance comparison.".to_string()),
        correction_suggestion: None,
    })
}

#[utoipa::path(
    get,
    path = "/api/v1/explore",
    responses(
        (status = 200, description = "List all skill metadata")
    ),
    tag = "admin"
)]
pub async fn explore_handler() -> Json<serde_json::Value> {
    let tools = skill_routes::all_tools();
    let skills: Vec<serde_json::Value> = tools
        .iter()
        .map(|t| {
            json!({
                "category": t.category,
                "name": t.skill,
                "route": t.route,
                "description": t.description,
            })
        })
        .collect();

    Json(json!({
        "status": "success",
        "engine": "dsaengine",
        "skill_count": skills.len(),
        "skills": skills,
        "description": "Auto-discovered skill registry powered by #[mcp_tool] metadata."
    }))
}

async fn mcp_list_tools() -> axum::Json<Vec<McpTool>> {
    axum::Json(skill_routes::all_tools())
}

fn mcp_ok(id: Value, result: Value) -> Json<Value> {
    Json(json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result
    }))
}

fn mcp_err(id: Value, code: i64, message: &str) -> Json<Value> {
    Json(json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message
        }
    }))
}

async fn mcp_rpc_handler(Json(payload): Json<Value>) -> Json<Value> {
    let id = payload.get("id").cloned().unwrap_or(Value::Null);
    let method = payload.get("method").and_then(Value::as_str).unwrap_or("");

    let result = match method {
        "initialize" => mcp_common::handle_initialize(),
        "notifications/initialized" => json!({}),
        "tools/list" => mcp_common::handle_tools_list(),
        "tools/call" => {
            let params = payload.get("params").cloned().unwrap_or_else(|| json!({}));
            let name = params.get("name").and_then(Value::as_str).unwrap_or("");
            let arguments = params
                .get("arguments")
                .cloned()
                .unwrap_or_else(|| json!({}));
            if name.is_empty() {
                return mcp_err(id, -32602, "Missing tool name in params.");
            }
            mcp_common::handle_tools_call(name, arguments).await
        }
        _ => return mcp_err(id, -32601, "Method not found"),
    };
    mcp_ok(id, result)
}

pub async fn run_server() {
    // ═══════════════════════════════════════════════════════════════
    //  All 142 skill routes are auto-registered via skill_routes.rs
    //  generated by the #[mcp_tool] proc macro metadata.
    //  To add a new skill: create the .rs file, add to mod.rs,
    //  then run: python scratch/phase3_routes.py
    // ═══════════════════════════════════════════════════════════════
    let skill_router = Router::new().route("/explore", get(explore_handler));

    // Register all 142 skill routes in one call
    let api_routes = skill_routes::register(skill_router)
        .route("/:category/:skill", post(template_skill_handler))
        .layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
        .route("/", get(|| async { Json(json!({"message": "Welcome to dsaengine MCP Server", "status": "running", "health": "/health", "api": "/api/v1"})) }))
        .route("/health", get(|| async { Json(json!({"status": "up", "engine": "dsaengine"})) }))
        .route("/mcp", post(mcp_rpc_handler))
        .route("/mcp/list_tools", get(mcp_list_tools))
        .nest("/api/v1", api_routes)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::overriding(HeaderName::from_static("content-type"), |_: &_| Some(HeaderValue::from_static("application/json"))));

    let port = std::env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    let bind_addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Failed to bind configured address");
    println!(
        " dsaengine is live at http://{bind_addr} — {} tools registered",
        skill_routes::all_tools().len()
    );
    axum::serve(listener, app).await.unwrap();
}

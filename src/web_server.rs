use axum::{Router, middleware, http::StatusCode, Json, http::{HeaderName, HeaderValue}};
use axum::routing::{get, post};
use axum::extract::Path;
use tower_http::trace::TraceLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use serde_json::json;
use dsaengine::skills::{
    advanced_topics, arrays_strings, graphs, sorting_searching, stacks_queues, trees_advanced,
};
use dsaengine::utils::api_docs;

#[derive(OpenApi)]
#[openapi(
    paths(
        fundamentals_skill_handler,
        arrays_skill_handler,
        linked_lists_skill_handler,
        stacks_skill_handler,
        trees_skill_handler,
        trees_adv_skill_handler,
        graphs_skill_handler,
        dp_skill_handler,
        greedy_skill_handler,
        backtracking_skill_handler,
        sorting_skill_handler,
        advanced_skill_handler,
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
    let mut skills = Vec::new();
    if let Ok(toplevel) = std::fs::read_dir("src/skills") {
        for cat_entry in toplevel.flatten() {
            if let Ok(cat_type) = cat_entry.file_type() {
                if cat_type.is_dir() {
                    let category = cat_entry.file_name().into_string().unwrap_or_default();
                    if let Ok(skill_entries) = std::fs::read_dir(cat_entry.path()) {
                        for skill_entry in skill_entries.flatten() {
                            let p = skill_entry.path();
                            if p.extension().and_then(|s| s.to_str()) == Some("rs") && p.file_name().and_then(|s| s.to_str()) != Some("mod.rs") {
                                let name = p.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown").to_string();
                                let file_contents = std::fs::read_to_string(&p).unwrap_or_default();
                                let mut input_contract = Vec::new();
                                for key in ["nums", "numbers", "target", "edges", "adj", "source", "num_nodes", "nodes", "values"].iter() {
                                    if file_contents.contains(key) {
                                        input_contract.push((*key).to_string());
                                    }
                                }

                                skills.push(serde_json::json!({
                                    "category": category,
                                    "name": name,
                                    "input_contract": input_contract,
                                    "description": "Auto-discovered skill metadata"
                                }));
                            }
                        }
                    }
                }
            }
        }
    }

    Json(serde_json::json!({
        "status": "success",
        "engine": "dsaengine",
        "skills": skills,
        "description": "List of registered skills with input contract keys"
    }))
}

macro_rules! define_category_handler {
    ($fn_name:ident, $category:expr) => {
        #[utoipa::path(
            post,
            path = concat!("/api/v1/", $category, "/{skill}"),
            params(
                ("skill" = String, Path, description = "Skill name", example = "dijkstra")
            ),
            request_body(content = api_docs::GraphRequest, content_type = "application/json"),
            responses(
                (status = 200, description = "Generic DSA operation response", body = api_docs::StandardResponse),
                (status = 404, description = "Skill not found")
            ),
            tag = $category
        )]
        pub async fn $fn_name(
            Path(skill): Path<String>,
            Json(payload): Json<serde_json::Value>,
        ) -> Json<api_docs::StandardResponse> {
            template_skill_handler(Path(($category.to_string(), skill)), Json(payload)).await
        }
    };
}

define_category_handler!(fundamentals_skill_handler, "fundamentals");
define_category_handler!(arrays_skill_handler, "arrays");
define_category_handler!(linked_lists_skill_handler, "linked_lists");
define_category_handler!(stacks_skill_handler, "stacks");
define_category_handler!(trees_skill_handler, "trees");
define_category_handler!(trees_adv_skill_handler, "trees_adv");
define_category_handler!(graphs_skill_handler, "graphs");
define_category_handler!(dp_skill_handler, "dp");
define_category_handler!(greedy_skill_handler, "greedy");
define_category_handler!(backtracking_skill_handler, "backtracking");
define_category_handler!(sorting_skill_handler, "sorting");
define_category_handler!(advanced_skill_handler, "advanced");


#[derive(serde::Serialize)]
pub struct McpTool {
    pub name: String,
    pub category: String,
    pub route: String,
    pub description: String,
}

async fn mcp_list_tools() -> axum::Json<Vec<McpTool>> {
    let mut tools = Vec::new();
    if let Ok(entries) = std::fs::read_dir("src/skills") {
        for cat in entries.flatten() {
            if let Ok(meta) = cat.metadata() {
                if meta.is_dir() {
                    let cat_name = cat.file_name().to_string_lossy().to_string();
                    if let Ok(files) = std::fs::read_dir(cat.path()) {
                        for skill in files.flatten() {
                            if skill.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                                if let Some(name) = skill.path().file_stem().and_then(|s| s.to_str()) {
                                    tools.push(McpTool {
                                        name: format!("{}/{}", cat_name, name),
                                        category: cat_name.clone(),
                                        route: format!("/api/v1/{}/{}", cat_name, name),
                                        description: format!("Executes {}/{} algorithm tool.", cat_name, name),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    axum::Json(tools)
}

pub async fn run_server() {
    // 2. Generate all 100+ routes in one line
    let api_routes = Router::new()
        .route("/explore", get(explore_handler))
        .route("/graphs/dijkstra", post(graphs::dijkstra::post))
        .route("/advanced/bloom_filter", post(advanced_topics::bloom_filter::post))
        .route("/advanced/lfu_cache", post(advanced_topics::lfu_cache::post))
        .route("/advanced/lru_cache", post(advanced_topics::lru_cache::post))
        .route("/advanced/suffix_array", post(advanced_topics::suffix_array::post))
        .route("/advanced_topics/bloom_filter", post(advanced_topics::bloom_filter::post))
        .route("/advanced_topics/lfu_cache", post(advanced_topics::lfu_cache::post))
        .route("/advanced_topics/lru_cache", post(advanced_topics::lru_cache::post))
        .route("/advanced_topics/suffix_array", post(advanced_topics::suffix_array::post))
        .route("/stacks/stack_via_queues", post(stacks_queues::stack_via_queues::post))
        .route("/stacks/queue_via_stacks", post(stacks_queues::queue_via_stacks::post))
        .route("/stacks/min_stack", post(stacks_queues::min_stack::post))
        .route("/stacks_queues/stack_via_queues", post(stacks_queues::stack_via_queues::post))
        .route("/stacks_queues/queue_via_stacks", post(stacks_queues::queue_via_stacks::post))
        .route("/stacks_queues/min_stack", post(stacks_queues::min_stack::post))
        .route("/trees_adv/top_k_elements", post(trees_advanced::top_k_elements::post))
        .route("/trees_adv/segment_tree_query", post(trees_advanced::segment_tree_query::post))
        .route("/trees_adv/segment_tree_builder", post(trees_advanced::segment_tree_builder::post))
        .route("/trees_advanced/top_k_elements", post(trees_advanced::top_k_elements::post))
        .route("/trees_advanced/segment_tree_query", post(trees_advanced::segment_tree_query::post))
        .route("/trees_advanced/segment_tree_builder", post(trees_advanced::segment_tree_builder::post))
        .route("/sorting/visualizer", post(sorting_searching::visualizer::post))
        .route("/sorting/rotated_search", post(sorting_searching::rotated_search::post))
        .route("/sorting_searching/visualizer", post(sorting_searching::visualizer::post))
        .route("/sorting_searching/rotated_search", post(sorting_searching::rotated_search::post))
        .route("/arrays/anagram_detector", post(arrays_strings::anagram_detector::post))
        .route("/arrays/array_rotation", post(arrays_strings::array_rotation::post))
        .route("/arrays/compression", post(arrays_strings::compression::post))
        .route("/arrays/boyer_moore_voting", post(arrays_strings::boyer_moore_voting::post))
        .route("/arrays/container_water", post(arrays_strings::container_water::post))
        .route("/arrays/difference_array", post(arrays_strings::difference_array::post))
        .route("/arrays/dutch_national_flag", post(arrays_strings::dutch_national_flag::post))
        .route("/arrays/kadanes_algorithm", post(arrays_strings::kadanes_algorithm::post))
        .route("/arrays/kmp_search", post(arrays_strings::kmp_search::post))
        .route("/arrays/longest_substring", post(arrays_strings::longest_substring::post))
        .route("/arrays/manachers_algorithm", post(arrays_strings::manachers_algorithm::post))
        .route("/arrays/next_permutation", post(arrays_strings::next_permutation::post))
        .route("/arrays/palindrome_matcher", post(arrays_strings::palindrome_matcher::post))
        .route("/arrays/rabin_karp", post(arrays_strings::rabin_karp::post))
        .route("/arrays/rainwater_trapping", post(arrays_strings::rainwater_trapping::post))
        .route("/arrays/subarray_sum", post(arrays_strings::subarray_sum::post))
        .route("/arrays/three_sum_solver", post(arrays_strings::three_sum_solver::post))
        .route("/arrays/two_sum_matcher", post(arrays_strings::two_sum_matcher::post))
        .route("/arrays/z_algorithm", post(arrays_strings::z_algorithm::post))
        .route("/arrays_strings/anagram_detector", post(arrays_strings::anagram_detector::post))
        .route("/arrays_strings/array_rotation", post(arrays_strings::array_rotation::post))
        .route("/arrays_strings/compression", post(arrays_strings::compression::post))
        .route("/arrays_strings/boyer_moore_voting", post(arrays_strings::boyer_moore_voting::post))
        .route("/arrays_strings/container_water", post(arrays_strings::container_water::post))
        .route("/arrays_strings/difference_array", post(arrays_strings::difference_array::post))
        .route("/arrays_strings/dutch_national_flag", post(arrays_strings::dutch_national_flag::post))
        .route("/arrays_strings/kadanes_algorithm", post(arrays_strings::kadanes_algorithm::post))
        .route("/arrays_strings/kmp_search", post(arrays_strings::kmp_search::post))
        .route("/arrays_strings/longest_substring", post(arrays_strings::longest_substring::post))
        .route("/arrays_strings/manachers_algorithm", post(arrays_strings::manachers_algorithm::post))
        .route("/arrays_strings/next_permutation", post(arrays_strings::next_permutation::post))
        .route("/arrays_strings/palindrome_matcher", post(arrays_strings::palindrome_matcher::post))
        .route("/arrays_strings/rabin_karp", post(arrays_strings::rabin_karp::post))
        .route("/arrays_strings/rainwater_trapping", post(arrays_strings::rainwater_trapping::post))
        .route("/arrays_strings/subarray_sum", post(arrays_strings::subarray_sum::post))
        .route("/arrays_strings/three_sum_solver", post(arrays_strings::three_sum_solver::post))
        .route("/arrays_strings/two_sum_matcher", post(arrays_strings::two_sum_matcher::post))
        .route("/arrays_strings/z_algorithm", post(arrays_strings::z_algorithm::post))
        .route("/:category/:skill", post(template_skill_handler))
        .layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
        .route("/", get(|| async { Json(json!({"message": "Welcome to dsaengine API", "status": "running", "health": "/health", "api": "/api/v1"})) }))
        .route("/health", get(|| async { Json(json!({"status": "up", "engine": "dsaengine"})) }))
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
    println!(" dsaengine is live at http://{bind_addr}");
    axum::serve(listener, app).await.unwrap();
}

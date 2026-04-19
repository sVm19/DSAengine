//! MCP Stdio Server
//!
//! Implements the Model Context Protocol over stdin/stdout (JSON-RPC 2.0).
//! Coding agents (Cursor, Windsurf, Claude Code, etc.) spawn this binary with
//! `--mcp` and communicate via newline-delimited JSON — zero HTTP, zero hosting,
//! zero API cost.
//!
//! Protocol flow:
//!   agent → stdin  → `initialize`   → server replies with capabilities
//!   agent → stdin  → `tools/list`   → server lists all 142+ tools + dsa_classify
//!   agent → stdin  → `tools/call`   → server dispatches to algorithm handler
//!   agent → stdin  → any other      → server returns method-not-found error

use dsaengine::skill_routes;
use dsaengine::utils::classifier;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

/// Entry point for `--mcp` mode. Reads newline-delimited JSON-RPC from stdin,
/// writes responses to stdout. Runs forever until stdin is closed.
pub async fn run() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    eprintln!("[dsaengine-mcp] stdio server ready");

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) if l.trim().is_empty() => continue,
            Ok(l) => l,
            Err(_) => break,
        };

        let response = handle_message(&line).await;

        let mut out = stdout.lock();
        let _ = writeln!(out, "{response}");
        let _ = out.flush();
    }

    eprintln!("[dsaengine-mcp] stdin closed, exiting");
}

/// Parses a single JSON-RPC message and returns the serialised response.
async fn handle_message(raw: &str) -> String {
    let msg: Value = match serde_json::from_str(raw) {
        Ok(v) => v,
        Err(e) => {
            return json!({
                "jsonrpc": "2.0",
                "id": null,
                "error": { "code": -32700, "message": format!("Parse error: {e}") }
            })
            .to_string();
        }
    };

    let id = msg.get("id").cloned().unwrap_or(Value::Null);
    let method = msg.get("method").and_then(Value::as_str).unwrap_or("");

    let result = match method {
        "initialize" => handle_initialize(),
        "notifications/initialized" => json!({}),
        "tools/list" => handle_tools_list(),
        "tools/call" => handle_tools_call(&msg).await,
        _ => {
            return json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": { "code": -32601, "message": format!("Method not found: {method}") }
            })
            .to_string();
        }
    };

    json!({ "jsonrpc": "2.0", "id": id, "result": result }).to_string()
}

// ── Handlers ─────────────────────────────────────────────────────────────────

fn handle_initialize() -> Value {
    json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {
            "tools": { "listChanged": false }
        },
        "serverInfo": {
            "name": "DSAEngine",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "142+ optimal DSA algorithm implementations. Zero-cost, local, deterministic."
        }
    })
}

fn handle_tools_list() -> Value {
    let mut tools: Vec<Value> = Vec::new();

    // ── Meta-tool: dsa_classify ──────────────────────────────────────────────
    tools.push(json!({
        "name": "dsa_classify",
        "description": "CALL THIS FIRST. Classifies a problem description into optimal DSA algorithm \
                        recommendations using deterministic keyword matching. Returns ranked algorithms \
                        with complexity analysis and anti-patterns to avoid. Input: natural language \
                        description of what you need to build.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "description": {
                    "type": "string",
                    "description": "Natural language description of the problem, e.g. 'find shortest path in a weighted city graph' or 'cache user sessions with eviction'"
                }
            },
            "required": ["description"]
        }
    }));

    // ── All 142 algorithm tools from skill_routes ────────────────────────────
    for tool in skill_routes::all_tools() {
        tools.push(json!({
            "name": tool.name,
            "description": tool.description,
            "inputSchema": {
                "type": "object",
                "additionalProperties": true,
                "description": format!("Arguments for {}. See /api/v1/{}/{} for full schema.", tool.name, tool.category, tool.skill)
            }
        }));
    }

    json!({ "tools": tools })
}

async fn handle_tools_call(msg: &Value) -> Value {
    let params = msg.get("params").cloned().unwrap_or_else(|| json!({}));
    let name = params.get("name").and_then(Value::as_str).unwrap_or("");
    let arguments = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));

    if name.is_empty() {
        return error_result(-32602, "Missing tool name in params");
    }

    // ── Route to dsa_classify ────────────────────────────────────────────────
    if name == "dsa_classify" {
        return handle_classify(arguments);
    }

    // ── Route to algorithm tools via internal HTTP call ──────────────────────
    match execute_algorithm(name, arguments).await {
        Ok(result) => json!({
            "content": [{
                "type": "text",
                "text": format!("Executed '{}' successfully.", name)
            }],
            "structuredContent": result
        }),
        Err(e) => error_result(-32000, &e),
    }
}

/// Handles `dsa_classify` — pure Rust, zero network, zero AI cost.
fn handle_classify(arguments: Value) -> Value {
    let description = arguments
        .get("description")
        .and_then(Value::as_str)
        .unwrap_or("");

    if description.is_empty() {
        return error_result(-32602, "dsa_classify requires a 'description' string");
    }

    let recommendations = classifier::classify(description);

    if recommendations.is_empty() {
        return json!({
            "content": [{
                "type": "text",
                "text": "No strong algorithm match found. Consider describing the data structure, \
                         operation type (search/insert/delete), or problem pattern (e.g. 'sliding window', \
                         'shortest path', 'top-k elements')."
            }],
            "structuredContent": {
                "status": "no_match",
                "suggestions": [
                    "Try adding keywords like: shortest path, cache, subarray, prefix, connected components, sorted, pattern search",
                    "Or browse all tools with tools/list"
                ]
            }
        });
    }

    let top = &recommendations[0];
    let summary = format!(
        "Recommended: {} ({}). Call tool: `{}`",
        top.algorithm, top.time_complexity, top.tool_name
    );

    json!({
        "content": [{
            "type": "text",
            "text": summary
        }],
        "structuredContent": {
            "status": "ok",
            "recommendations": recommendations,
            "next_step": format!("Call `{}` with your data to get the optimal implementation.", top.tool_name)
        }
    })
}

/// Calls the algorithm tool by routing through the existing HTTP web server
/// running on localhost. The web server is already running (same process boot),
/// or we forward using the internal base URL.
async fn execute_algorithm(name: &str, arguments: Value) -> Result<Value, String> {
    let (category, skill) = name
        .split_once('.')
        .ok_or_else(|| format!("Invalid tool name '{name}'. Expected '<category>.<skill>'."))?;

    let master_key = std::env::var("MASTER_API_2026")
        .map_err(|_| "MASTER_API_2026 env var not set. Run `dsaengine` (HTTP mode) in background or set the key.".to_string())?;

    let port = std::env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    let base =
        std::env::var("INTERNAL_BASE_URL").unwrap_or_else(|_| format!("http://127.0.0.1:{port}"));
    let url = format!("{base}/api/v1/{category}/{skill}");

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("X-API-KEY", master_key)
        .json(&arguments)
        .send()
        .await
        .map_err(|e| {
            format!("HTTP call to {url} failed: {e}. Is `dsaengine` HTTP server running?")
        })?;

    let status = response.status().as_u16();
    let body: Value = response
        .json()
        .await
        .map_err(|e| format!("JSON parse error from {url}: {e}"))?;

    Ok(json!({ "status_code": status, "response": body }))
}

fn error_result(code: i64, message: &str) -> Value {
    json!({
        "content": [{ "type": "text", "text": message }],
        "isError": true,
        "error": { "code": code, "message": message }
    })
}

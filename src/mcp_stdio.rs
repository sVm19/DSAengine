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
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

/// Entry point for `--mcp` mode. Reads newline-delimited JSON-RPC from stdin,
/// writes responses to stdout. Runs forever until stdin is closed.
pub async fn run() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    eprintln!("DSA Engine MCP mode started. Waiting for tool calls...");

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

    // Delegate entirely to mcp_common — single source of truth, zero HTTP.
    dsaengine::mcp_common::handle_tools_call(name, arguments).await
}

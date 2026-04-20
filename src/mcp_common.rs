// src/mcp_common.rs

use crate::skill_routes;
use crate::utils::classifier;
use serde_json::{json, Value};

/// Handles `initialize` method, returns server capabilities.
pub fn handle_initialize() -> Value {
    json!({
        "protocolVersion": "2024-11-05",
        "capabilities": { "tools": { "listChanged": false } },
        "serverInfo": {
            "name": "DSAEngine",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "142+ optimal DSA algorithm implementations. Zero-cost, local, deterministic."
        }
    })
}

/// Returns the list of tools, including the meta-tool `dsa_classify`.
pub fn handle_tools_list() -> Value {
    let mut tools: Vec<Value> = Vec::new();
    // meta-tool
    tools.push(json!({
        "name": "dsa_classify",
        "description": "CALL THIS FIRST. Classifies a problem description into optimal DSA algorithm recommendations using deterministic keyword matching.",
        "inputSchema": {
            "type": "object",
            "properties": { "description": { "type": "string", "description": "Natural language description of the problem" }, "mode": { "type": "string", "enum": ["result","code","pseudocode","full"], "description": "Output mode" } },
            "required": ["description"]
        }
    }));
    // all algorithm tools
    for tool in skill_routes::all_tools() {
        tools.push(json!({
            "name": tool.name,
            "description": tool.description,
            "inputSchema": {
                "type": "object",
                "additionalProperties": true,
                "description": format!("Arguments for {}", tool.name)
            }
        }));
    }
    json!({ "tools": tools })
}

/// Handles a tool call. Routes `dsa_classify` to the classifier, otherwise forwards to the HTTP backend.
pub async fn handle_tools_call(name: &str, arguments: Value) -> Value {
    if name.is_empty() {
        return error_result(-32602, "Missing tool name in params");
    }
    if name == "dsa_classify" {
        return handle_classify(arguments);
    }
    // forward to local in-memory executor
    match crate::utils::executor::execute_algorithm_local(name, arguments).await {
        Ok(result) => json!({
            "content": [{ "type": "text", "text": format!("Executed '{}' successfully.", name) }],
            "structuredContent": result
        }),
        Err(e) => error_result(-32000, &e),
    }
}

/// Handles `dsa_classify` with optional `mode` parameter.
fn handle_classify(arguments: Value) -> Value {
    let description = arguments
        .get("description")
        .and_then(Value::as_str)
        .unwrap_or("");
    let mode = arguments
        .get("mode")
        .and_then(Value::as_str)
        .unwrap_or("result");
    if description.is_empty() {
        return error_result(-32602, "dsa_classify requires a 'description' string");
    }
    let recommendations = classifier::classify(description);
    if recommendations.is_empty() {
        return json!({
            "content": [{ "type": "text", "text": "No strong algorithm match found." }],
            "structuredContent": { "status": "no_match" }
        });
    }
    let top = &recommendations[0];
    let summary = format!(
        "Recommended: {} ({}). Call tool: `{}`",
        top.algorithm, top.time_complexity, top.tool_name
    );
    match mode {
        "result" => json!({
            "content": [{ "type": "text", "text": summary }],
            "structuredContent": { "status": "ok", "recommendations": recommendations, "next_step": format!("Call `{}` with your data.", top.tool_name) }
        }),
        "code" => json!({
            "content": [{ "type": "text", "text": summary }],
            "structuredContent": { "code_path": top.tool_name }
        }),
        "pseudocode" => json!({
            "content": [{ "type": "text", "text": summary }],
            "structuredContent": { "pseudocode": format!("// Pseudocode for {}", top.algorithm) }
        }),
        "full" => json!({
            "content": [{ "type": "text", "text": summary }],
            "structuredContent": { "status": "ok", "recommendations": recommendations, "code_path": top.tool_name, "pseudocode": format!("// Pseudocode for {}", top.algorithm) }
        }),
        _ => json!({
            "content": [{ "type": "text", "text": summary }],
            "structuredContent": { "status": "ok", "recommendations": recommendations }
        }),
    }
}



fn error_result(code: i64, message: &str) -> Value {
    json!({
        "content": [{ "type": "text", "text": message }],
        "isError": true,
        "error": { "code": code, "message": message }
    })
}

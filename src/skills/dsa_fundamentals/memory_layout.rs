use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Memory Layout Visualizer
/// CATEGORY: dsa-fundamentals
/// DESCRIPTION: Simulates how data structures (Arrays vs Linked Lists) are mapped to memory addresses.
pub struct MemoryLayout;

impl Complexity for MemoryLayout {
    fn name(&self) -> &'static str {
        "Memory Layout Visualizer"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n)"
    }

    fn space_complexity(&self) -> &'static str {
        "O(1)"
    }

    fn description(&self) -> &'static str {
        "Helps the AI Agent visualize Cache Locality and the difference between Contiguous and Pointer-based storage."
    }
}

impl MemoryLayout {
    /// Visualizes a contiguous block of memory (e.g., an Array or Vector).
    pub fn visualize_contiguous<T: std::fmt::Debug>(data: &[T]) {
        println!("\n🧠 [MEMORY LAYOUT: CONTIGUOUS]");
        for (i, item) in data.iter().enumerate() {
            let addr = format!("{:p}", item);
            println!("  Address: {} | Index: [{}] | Value: {:?}", addr, i, item);
        }
        AgentLogger::log(
            AgentFeedback::Info,
            "Cache Locality: HIGH (Sequential access is fast)",
        );
    }

    /// Visualizes a heap-allocated, pointer-based structure (e.g., Linked List nodes).
    pub fn visualize_pointer_node<T: std::fmt::Debug>(
        value: T,
        address: String,
        next_address: String,
    ) {
        println!(
            "  [Node({:?})] @ {} ---> Next: {}",
            value, address, next_address
        );

        if next_address == "null" || next_address == "0x0" {
            AgentLogger::log(AgentFeedback::Step, "Tail of the structure reached.");
        }
    }

    /// Explains the impact of the layout on performance.
    pub fn explain_cache_impact(is_contiguous: bool) {
        if is_contiguous {
            println!("⚡ Pro: CPU can pre-fetch data due to spatial locality.");
        } else {
            println!("🐢 Con: Frequent cache misses due to pointer hopping (Random access).");
        }
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "dsa_fundamentals.memory_layout",
    description = "Use this for solving memory layout problems. Trigger Keywords: memory_layout, memory layout, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_memory_layout(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct MemoryLayoutRequest {
    pub values: Vec<Value>,
    pub layout: Option<String>,
}

async fn handle_memory_layout(payload: Value) -> DsaResult<ResultBox> {
    let req: MemoryLayoutRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid MemoryLayoutRequest: {e}"),
            hint: "Provide 'values' and optional 'layout' ('contiguous' or 'linked').".to_string(),
        })?;

    let layout = req.layout.unwrap_or_else(|| "contiguous".to_string());
    let cells: Vec<_> = req
        .values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            json!({
                "index": index,
                "synthetic_address": format!("0x{:08x}", index * std::mem::size_of::<Value>()),
                "value": value,
                "next": if layout == "linked" && index + 1 < req.values.len() {
                    json!(format!("0x{:08x}", (index + 1) * std::mem::size_of::<Value>()))
                } else {
                    Value::Null
                }
            })
        })
        .collect();

    let solver = MemoryLayout;
    Ok(ResultBox::success(json!({
        "layout": layout,
        "element_size_bytes": std::mem::size_of::<Value>(),
        "cells": cells,
        "cache_locality": if layout == "contiguous" { "high" } else { "low" }
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Memory-layout visualization generated."))
}

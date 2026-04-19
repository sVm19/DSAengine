use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::mem;

/// SKILL: Space Calculator
/// CATEGORY: dsa-fundamentals
/// DESCRIPTION: Calculates the static memory footprint of data types and estimates heap growth.
pub struct SpaceCalculator;

impl Complexity for SpaceCalculator {
    fn name(&self) -> &'static str {
        "Space Complexity Calculator"
    }

    fn time_complexity(&self) -> &'static str {
        "O(1)"
    }

    fn space_complexity(&self) -> &'static str {
        "O(1)"
    }

    fn description(&self) -> &'static str {
        "Analyzes the byte size of Rust types and predicts memory scaling for large inputs."
    }
}

impl SpaceCalculator {
    /// Returns the exact size of a type in bytes (Stack memory).
    /// AI agents use this to prevent Stack Overflow in recursion.
    pub fn get_stack_size<T>() -> usize {
        mem::size_of::<T>()
    }

    /// Estimates the heap memory for a collection of a given size.
    /// Formula: (Size of Type * Number of Elements)
    pub fn estimate_collection_memory<T>(count: usize) -> usize {
        let element_size = mem::size_of::<T>();
        let total = element_size * count;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Estimating memory for {} elements: {} bytes", count, total),
        );

        total
    }

    /// Provides a warning if the memory usage exceeds a specific threshold.
    pub fn check_memory_safety(bytes: usize) {
        let mb = bytes as f64 / 1_048_576.0;
        if mb > 512.0 {
            AgentLogger::log(
                AgentFeedback::Warning,
                format!("High Memory Usage Detected: {:.2} MB", mb),
            );
        } else {
            AgentLogger::log(
                AgentFeedback::Success,
                format!("Memory within safe limits: {:.2} MB", mb),
            );
        }
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "dsa_fundamentals.space_calculator",
    description = "Use this for solving space calculator problems. Trigger Keywords: space_calculator, space calculator, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_space_calculator(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SpaceCalculatorRequest {
    pub count: usize,
    pub element_size_bytes: Option<usize>,
    pub type_name: Option<String>,
}

async fn handle_space_calculator(payload: Value) -> DsaResult<ResultBox> {
    let req: SpaceCalculatorRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid SpaceCalculatorRequest: {e}"),
            hint: "Provide 'count' and optional 'element_size_bytes' or 'type_name'.".to_string(),
        })?;

    let type_name = req.type_name.unwrap_or_else(|| "u8".to_string());
    let element_size = req
        .element_size_bytes
        .unwrap_or_else(|| match type_name.as_str() {
            "u16" | "i16" => SpaceCalculator::get_stack_size::<u16>(),
            "u32" | "i32" | "f32" => SpaceCalculator::get_stack_size::<u32>(),
            "u64" | "i64" | "f64" | "usize" | "isize" => SpaceCalculator::get_stack_size::<u64>(),
            "bool" => SpaceCalculator::get_stack_size::<bool>(),
            _ => SpaceCalculator::get_stack_size::<u8>(),
        });
    let total_bytes = element_size.saturating_mul(req.count);
    SpaceCalculator::check_memory_safety(total_bytes);

    let solver = SpaceCalculator;
    Ok(ResultBox::success(json!({
        "type_name": type_name,
        "count": req.count,
        "element_size_bytes": element_size,
        "total_bytes": total_bytes,
        "total_mebibytes": total_bytes as f64 / 1_048_576.0,
        "within_512_mib_soft_limit": total_bytes <= 512 * 1_048_576
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Space usage estimate completed."))
}

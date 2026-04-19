use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Bit Manipulation
/// CATEGORY: advanced-topics
/// DESCRIPTION: Solves the classic single-number problem with XOR and exposes a few constant-time bit helpers.
pub struct BitManipulation;

impl Complexity for BitManipulation {
    fn name(&self) -> &'static str {
        "Bit Manipulation"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) for batch XOR reduction, O(1) for the primitive bit helpers."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - Uses only register-sized accumulators."
    }

    fn description(&self) -> &'static str {
        "Folds a slice with XOR to isolate the unique value while keeping memory usage constant."
    }
}

impl BitManipulation {
    pub fn solve(values: &[u32]) -> u32 {
        Self::single_number(values)
    }

    pub fn single_number(values: &[u32]) -> u32 {
        let mut accumulator = 0u32;

        for (index, &value) in values.iter().enumerate() {
            let previous = accumulator;
            accumulator ^= value;
            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "XOR folded index {}: {} ^ {} -> {}.",
                    index, previous, value, accumulator
                ),
            );
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Unique value resolved to {}.", accumulator),
        );
        accumulator
    }

    pub fn count_set_bits(mut value: u64) -> u32 {
        let mut count = 0u32;
        while value != 0 {
            value &= value - 1;
            count += 1;
        }
        count
    }

    pub fn is_power_of_two(value: u64) -> bool {
        value != 0 && (value & (value - 1)) == 0
    }

    pub fn next_power_of_two(mut value: u64) -> u64 {
        if value <= 1 {
            return 1;
        }

        value -= 1;
        value |= value >> 1;
        value |= value >> 2;
        value |= value >> 4;
        value |= value >> 8;
        value |= value >> 16;
        value |= value >> 32;
        value + 1
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "bit_manipulation",
    description = "Use this for solving bit manipulation problems. Trigger Keywords: bit_manipulation, bit manipulation, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_bit_manipulation(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_bit_manipulation(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        values: Option<Vec<u32>>,
        value: Option<u64>,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'values' for single_number, or 'value' for bit ops. Optional 'mode': 'single_number' | 'count_bits' | 'power_check' | 'next_power'.".to_string(),
    })?;

    let result = match req.mode.as_deref().unwrap_or("") {
        "count_bits" => {
            let v = req.value.unwrap_or(0);
            let bits = BitManipulation::count_set_bits(v);
            json!({ "mode": "count_bits", "value": v, "set_bits": bits })
        }
        "power_check" => {
            let v = req.value.unwrap_or(0);
            let is_pow = BitManipulation::is_power_of_two(v);
            json!({ "mode": "power_check", "value": v, "is_power_of_two": is_pow })
        }
        "next_power" => {
            let v = req.value.unwrap_or(0);
            let next = BitManipulation::next_power_of_two(v);
            json!({ "mode": "next_power", "value": v, "next_power_of_two": next })
        }
        _ => {
            let vals = req.values.unwrap_or_default();
            let single = BitManipulation::single_number(&vals);
            json!({ "mode": "single_number", "single_number": single })
        }
    };

    let solver = BitManipulation;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["single_number", "count_bits", "power_check", "next_power"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Bit manipulation completed."))
}

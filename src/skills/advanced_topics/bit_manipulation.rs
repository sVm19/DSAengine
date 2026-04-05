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
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "bit_manipulation", description = "Use this for solving bit manipulation problems. Trigger Keywords: bit_manipulation, bit manipulation, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Difference Array
/// CATEGORY: arrays-strings
/// DESCRIPTION: Applies batched inclusive range increments using a difference buffer and one prefix pass.
pub struct DifferenceArray;

impl Complexity for DifferenceArray {
    fn name(&self) -> &'static str {
        "Difference Array"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n + q) - q range updates plus one prefix reconstruction over n cells."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) - Stores a difference buffer and the rebuilt output array."
    }

    fn description(&self) -> &'static str {
        "Transforms many range writes into constant-time edge markers, then reconstructs with prefix sums."
    }
}

impl DifferenceArray {
    pub fn solve(base: &[i32], updates: &[(usize, usize, i32)]) -> Vec<i32> {
        if base.is_empty() {
            return Vec::new();
        }

        let mut diff = vec![0; base.len() + 1];
        for &(start, end, delta) in updates {
            if start >= base.len() {
                AgentLogger::log(
                    AgentFeedback::Warning,
                    format!(
                        "Skipping update [{}, {}] because start is out of bounds.",
                        start, end
                    ),
                );
                continue;
            }

            let right = end.min(base.len() - 1);
            if start > right {
                continue;
            }

            diff[start] += delta;
            diff[right + 1] -= delta;
            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Queued delta {} for inclusive range [{}, {}].",
                    delta, start, right
                ),
            );
        }

        let mut carry = 0;
        let mut rebuilt = Vec::with_capacity(base.len());
        for (index, &value) in base.iter().enumerate() {
            carry += diff[index];
            rebuilt.push(value + carry);
        }

        AgentLogger::log(
            AgentFeedback::Success,
            "Collapsed the difference buffer back into the updated array.",
        );
        rebuilt
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "difference_array", description = "Use this for solving difference array problems. Trigger Keywords: difference_array, difference array, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

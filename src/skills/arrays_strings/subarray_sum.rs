use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::HashMap;

/// SKILL: Subarray Sum
/// CATEGORY: arrays-strings
/// DESCRIPTION: Counts subarrays with sum k using prefix sums and a frequency hashmap.
pub struct SubarraySum;

impl Complexity for SubarraySum {
    fn name(&self) -> &'static str {
        "Subarray Sum"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Each value updates one prefix sum and one hashmap lookup."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) - Stores the frequency of previously seen prefix sums."
    }

    fn description(&self) -> &'static str {
        "Uses prefix_sum - k lookups to count every valid subarray ending at the current index."
    }
}

impl SubarraySum {
    pub fn solve(nums: &[i32], target: i32) -> usize {
        let target = target as i64;
        let mut prefix_sum = 0i64;
        let mut total = 0usize;
        let mut frequency = HashMap::new();
        frequency.insert(0i64, 1usize);

        for (index, &value) in nums.iter().enumerate() {
            prefix_sum += value as i64;
            if let Some(&matches_here) = frequency.get(&(prefix_sum - target)) {
                total += matches_here;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Prefix sum {} closed {} subarray(s) ending at index {}.",
                        prefix_sum, matches_here, index
                    ),
                );
            }

            *frequency.entry(prefix_sum).or_insert(0) += 1;
            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Updated prefix-sum hashmap with running total {}.",
                    prefix_sum
                ),
            );
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Counted {} subarray(s) that sum to {}.", total, target),
        );
        total
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "arrays_strings.subarray_sum", description = "Use this for solving subarray sum problems. Trigger Keywords: subarray_sum, subarray sum, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_subarray_sum(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

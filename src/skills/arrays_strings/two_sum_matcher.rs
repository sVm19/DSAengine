use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::HashMap;

/// SKILL: Two Sum Matcher
/// CATEGORY: arrays-strings
/// DESCRIPTION: Finds a target-summing pair with a one-pass hashmap lookup.
pub struct TwoSumMatcher;

impl Complexity for TwoSumMatcher {
    fn name(&self) -> &'static str {
        "Two Sum Matcher"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Each value is inserted into and queried from the hashmap once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) - Stores previously seen values and their indices."
    }

    fn description(&self) -> &'static str {
        "Matches each value against the complement it needs to reach the target."
    }
}

impl TwoSumMatcher {
    pub fn solve(nums: &[i32], target: i32) -> Option<(usize, usize)> {
        let mut seen = HashMap::with_capacity(nums.len());
        for (index, &value) in nums.iter().enumerate() {
            let complement = target - value;
            if let Some(&left_index) = seen.get(&complement) {
                AgentLogger::log(
                    AgentFeedback::Success,
                    format!(
                        "Matched indices ({}, {}) because {} + {} = {}.",
                        left_index, index, complement, value, target
                    ),
                );
                return Some((left_index, index));
            }

            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Recording value {} at index {} while waiting for complement {}.",
                    value, index, complement
                ),
            );
            seen.insert(value, index);
        }

        AgentLogger::log(
            AgentFeedback::Warning,
            format!("No pair sums to target {}.", target),
        );
        None
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "two_sum_matcher", description = "Use this for solving two sum matcher problems. Trigger Keywords: two_sum_matcher, two sum matcher, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

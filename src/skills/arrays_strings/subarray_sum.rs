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
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct SubarraySumRequest {
    pub nums: Vec<i32>,
    pub target: i32,
}

#[macros::mcp_tool(
    name = "arrays_strings.subarray_sum",
    description = "Use this for solving subarray sum problems. Trigger Keywords: subarray_sum, subarray sum, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_subarray_sum(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_subarray_sum(payload: Value) -> DsaResult<ResultBox> {
    let req: SubarraySumRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid SubarraySumRequest: {e}"),
            hint: "Provide 'nums' and 'target'.".to_string(),
        })?;

    let count = SubarraySum::solve(&req.nums, req.target);
    let solver = SubarraySum;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });
    Ok(ResultBox::success(json!({
        "count": count
    }))
    .with_complexity(complexity)
    .with_description("Subarray-sum counting completed."))
}

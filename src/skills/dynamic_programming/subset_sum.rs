use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Subset Sum
/// CATEGORY: dynamic-programming
/// DESCRIPTION: Determines whether any subset of a given integer slice sums to a target value,
///              using a space-optimised 1-D boolean DP array (bitset knapsack style).
pub struct SubsetSum;

impl Complexity for SubsetSum {
    fn name(&self) -> &'static str {
        "Subset Sum (1-D Bitset DP)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n * target) — n elements × target capacity; inner loop traversed right-to-left."
    }

    fn space_complexity(&self) -> &'static str {
        "O(target) — A single boolean array of length target+1; no 2-D table."
    }

    fn description(&self) -> &'static str {
        "Iterates items over a right-to-left capacity sweep; dp[c] = true once any subset summing to c is found."
    }
}

impl SubsetSum {
    /// Returns `true` if any subset of `nums` sums exactly to `target`.
    pub fn solve(nums: &[u32], target: u32) -> bool {
        if target == 0 {
            return true;
        }

        let cap = target as usize;
        let mut dp = vec![false; cap + 1];
        dp[0] = true; // Empty subset has sum 0.

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Subset-sum DP: {} item(s), target={target}.", nums.len()),
        );

        for (idx, &num) in nums.iter().enumerate() {
            let num = num as usize;
            if num > cap {
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Skipped nums[{idx}]={num}: exceeds target capacity."),
                );
                continue;
            }

            // Right-to-left sweep prevents reusing an item in the same iteration.
            for c in (num..=cap).rev() {
                if dp[c - num] && !dp[c] {
                    dp[c] = true;
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!(
                            "nums[{idx}]={num}: marking dp[{c}] = true (dp[{}] was true).",
                            c - num
                        ),
                    );
                }
            }
        }

        let result = dp[cap];
        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Subset-sum({target}): {}.",
                if result {
                    "achievable"
                } else {
                    "not achievable"
                }
            ),
        );
        result
    }

    /// Returns all achievable subset sums up to `target`.
    pub fn all_sums(nums: &[u32], target: u32) -> Vec<u32> {
        let cap = target as usize;
        let mut dp = vec![false; cap + 1];
        dp[0] = true;

        for &num in nums {
            let num = num as usize;
            if num > cap {
                continue;
            }
            for c in (num..=cap).rev() {
                if dp[c - num] {
                    dp[c] = true;
                }
            }
        }

        let sums: Vec<u32> = (0..=cap).filter(|&c| dp[c]).map(|c| c as u32).collect();
        AgentLogger::log(
            AgentFeedback::Success,
            format!("Found {} achievable sum(s) up to {target}.", sums.len()),
        );
        sums
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "subset_sum",
    description = "Use this for solving subset sum problems. Trigger Keywords: subset_sum, subset sum, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_subset_sum(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_subset_sum(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        nums: Vec<u32>,
        target: u32,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'nums' (array of u32) and 'target' (u32). Optional 'mode': 'exists' | 'find_subset'.".to_string(),
    })?;

    let result = match req.mode.as_deref().unwrap_or("") {
        "find_subset" => {
            let subset = SubsetSum::all_sums(&req.nums, req.target);
            json!({ "mode": "find_subset", "subset": subset, "found": !subset.is_empty() })
        }
        _ => {
            let exists = SubsetSum::solve(&req.nums, req.target);
            json!({ "mode": "exists", "exists": exists })
        }
    };

    let solver = SubsetSum;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["exists", "find_subset"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Subset sum completed."))
}

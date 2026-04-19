use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::HashMap;

/// SKILL: Next Greater Element
/// CATEGORY: stacks-queues
/// DESCRIPTION: Finds the next strictly greater element for each element in an array
///              using a monotonically decreasing stack.
pub struct NextGreater;

impl Complexity for NextGreater {
    fn name(&self) -> &'static str {
        "Next Greater Element (Monotonic Stack)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Each element is pushed and popped exactly once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Sized by the stack mapping unresolved elements."
    }

    fn description(&self) -> &'static str {
        "Iterates over the sequence. If the current element is greater than the stack's top, it is the 'next greater' element for the stack's top. Pops and maps them, then pushes the current element."
    }
}

impl NextGreater {
    /// Returns the next greater element for each number in `nums1`, found within `nums2`.
    /// Elements not found resolve to -1.
    pub fn solve(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut stack = Vec::with_capacity(nums2.len());

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Next Greater Element mapping for {} array against subset of size {}.",
                nums2.len(),
                nums1.len()
            ),
        );

        for &num in nums2 {
            while let Some(&top) = stack.last() {
                if num > top {
                    stack.pop();
                    map.insert(top, num);
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Resolved next greater for {top} -> {num}."),
                    );
                } else {
                    break;
                }
            }
            stack.push(num);
        }

        let mut ans = Vec::with_capacity(nums1.len());
        for &num in nums1 {
            ans.push(*map.get(&num).unwrap_or(&-1));
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Mapped {} elements.", ans.len()),
        );

        ans
    }

    /// Solves the next greater element in a circular array.
    pub fn solve_circular(nums: &[i32]) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![-1; n];
        let mut stack: Vec<usize> = Vec::with_capacity(n); // stores indices

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Circular Next Greater Element for array of size {}.", n),
        );

        // Loop twice to simulate circular traversal
        for i in 0..(n * 2) {
            let idx = i % n;
            let val = nums[idx];

            while let Some(&top_idx) = stack.last() {
                if val > nums[top_idx] {
                    stack.pop();
                    ans[top_idx] = val;
                } else {
                    break;
                }
            }

            // Only push indices on the first pass
            if i < n {
                stack.push(idx);
            }
        }

        ans
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "next_greater",
    description = "Use this for solving next greater problems. Trigger Keywords: next_greater, next greater, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_next_greater(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_next_greater(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        nums: Vec<i32>,
        nums2: Option<Vec<i32>>,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'nums'. For next_greater_I: also provide 'nums2'. Optional 'mode': 'standard' | 'circular'.".to_string(),
    })?;

    let result = match req.mode.as_deref().unwrap_or("") {
        "circular" => {
            let result = NextGreater::solve_circular(&req.nums);
            json!({ "mode": "circular", "next_greater": result })
        }
        _ => {
            let nums2 = req.nums2.as_deref().unwrap_or(&req.nums);
            let result = NextGreater::solve(&req.nums, nums2);
            json!({ "mode": "standard", "next_greater": result })
        }
    };

    let solver = NextGreater;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["standard", "circular"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Next greater element computed."))
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::VecDeque;

/// SKILL: Sliding Window Maximum
/// CATEGORY: stacks-queues
/// DESCRIPTION: Finds the maximum value in every sliding window of size `k`
///              using a monotonically decreasing Double-Ended Queue (Deque).
pub struct WindowMaximum;

impl Complexity for WindowMaximum {
    fn name(&self) -> &'static str {
        "Sliding Window Maximum (Monotonic Deque)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Every index is enqueued and dequeued at most once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(k) — The deque never exceeds the size of the sliding window."
    }

    fn description(&self) -> &'static str {
        "Maintains a monotonic decreasing deque of indices. Front of deque is always the index of the max element. Out-of-window indices are popped from the front, and smaller elements are popped from the back."
    }
}

impl WindowMaximum {
    /// Returns a vector containing the maximum elements mapping to each `k`-sized window.
    pub fn solve(nums: &[i32], k: usize) -> Vec<i32> {
        let n = nums.len();
        if n == 0 || k == 0 {
            return Vec::new();
        }

        let mut ans = Vec::with_capacity(n - k + 1);
        let mut deque: VecDeque<usize> = VecDeque::new(); // Stores indices

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Sliding window maximum (size {k}) over {n} elements."),
        );

        for (i, &num) in nums.iter().enumerate() {
            // Remove indices that are out of the current window `i - k`
            if let Some(&front) = deque.front() {
                if front + k <= i {
                    deque.pop_front();
                }
            }

            // Remove elements from the back that are smaller than the current element
            while let Some(&back) = deque.back() {
                if nums[back] <= num {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back(i);

            // The window is fully formed at index k-1 onwards
            if i >= k - 1 {
                let max_in_window = nums[*deque.front().unwrap()];
                ans.push(max_in_window);

                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Window ending at {i} max is {max_in_window}."),
                );
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Extracted {} window maxima.", ans.len()),
        );

        ans
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "window_maximum",
    description = "Use this for solving window maximum problems. Trigger Keywords: window_maximum, window maximum, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_window_maximum(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_window_maximum(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        nums: Vec<i32>,
        k: usize,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'nums' (array of i32) and 'k' (window size).".to_string(),
    })?;

    let result = {
        let maxes = WindowMaximum::solve(&req.nums, req.k);
        json!({ "window_size": req.k, "maximums": maxes })
    };

    let solver = WindowMaximum;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["compute"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Sliding window maximum computed."))
}

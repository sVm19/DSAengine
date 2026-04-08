use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// SKILL: Top K Elements
/// CATEGORY: trees-advanced
/// DESCRIPTION: Locates the K largest elements natively within an array by
///              simulating an inverted min-heap pushing O(1) space limits inside O(log K).
pub struct TopKElements;

impl Complexity for TopKElements {
    fn name(&self) -> &'static str {
        "Top K Elements (K-Bounded Min-Heap)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N log K) — Scans N elements once. Only evaluates elements logically capable of displacing K-minimums."
    }

    fn space_complexity(&self) -> &'static str {
        "O(K) — Maximum heap container limit."
    }

    fn description(&self) -> &'static str {
        "Maintains a strict K-sized Min-Heap (via Reverse priority). If adding a new element breaches K size, the smallest is popped, ensuring only the largest K elements survive the complete scan iteration."
    }
}

impl TopKElements {
    pub fn solve(arr: &[i32], k: usize) -> Vec<i32> {
        let mut min_heap = BinaryHeap::with_capacity(k + 1);

        AgentLogger::log(AgentFeedback::Info, format!("Filtering Top {k} elements from {}-element stream.", arr.len()));

        for &val in arr {
            min_heap.push(Reverse(val));
            if min_heap.len() > k {
                let Reverse(dropped) = min_heap.pop().unwrap();
                AgentLogger::log(AgentFeedback::Step, format!("Added {val}. Heap exceeded {k}. Evicting global minimum {dropped}."));
            }
        }

        let result: Vec<i32> = min_heap.into_iter().map(|Reverse(x)| x).collect();
        AgentLogger::log(AgentFeedback::Success, "Top K elements dynamically extracted.");
        
        result
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct TopKElementsRequest {
    pub nums: Vec<i32>,
    pub k: usize,
}

#[macros::mcp_tool(name = "trees_advanced.top_k_elements", description = "Use this for solving top k elements problems. Trigger Keywords: top_k_elements, top k elements, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_top_k_elements(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_top_k_elements(payload: Value) -> DsaResult<ResultBox> {
    let req: TopKElementsRequest = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid TopKElementsRequest: {e}"),
        hint: "Provide 'nums' and positive 'k'.".to_string(),
    })?;

    if req.k == 0 {
        return Err(DsaError::InvalidInput {
            message: "k must be at least 1.".to_string(),
            hint: "Use a value like k=3 to fetch top 3 elements.".to_string(),
        });
    }

    let mut top = TopKElements::solve(&req.nums, req.k.min(req.nums.len()));
    top.sort_unstable_by(|a, b| b.cmp(a));

    let solver = TopKElements;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });

    Ok(ResultBox::success(json!({
        "top_k": top
    }))
    .with_complexity(complexity)
    .with_description("Top-K extraction completed."))
}

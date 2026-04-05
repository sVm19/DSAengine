use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Counting Sort
/// CATEGORY: sorting-searching
/// DESCRIPTION: Non-comparison integer sort algorithm that operates in O(N + K) time
///              where K is the range of non-negative values in the array.
pub struct CountingSort;

impl Complexity for CountingSort {
    fn name(&self) -> &'static str {
        "Counting Sort (Linear Integer Sort)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N + K) — One pass to count frequencies, one pass over K buckets, and one pass to reconstruct the array."
    }

    fn space_complexity(&self) -> &'static str {
        "O(K) — Memory proportional to the range (K) is required to maintain the frequency bucket array."
    }

    fn description(&self) -> &'static str {
        "Extracts min/max to establish range (K). Allocates a K-sized bucket array tracking occurrence frequency of each integer. Sequentially overwrites the input array bounded by frequencies."
    }
}

impl CountingSort {
    /// Sorts the array in ascending order in-place using counting sort.
    /// Values must be within a reasonably small range to avoid massive allocations.
    pub fn solve(arr: &mut [i32]) {
        if arr.len() <= 1 {
            return;
        }

        let min_val = *arr.iter().min().unwrap();
        let max_val = *arr.iter().max().unwrap();
        let range = (max_val - min_val) as usize + 1;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Counting sort on {} elements. Value range: {} to {} (size {}).", arr.len(), min_val, max_val, range),
        );

        if range > 100_000_000 {
            AgentLogger::log(
                AgentFeedback::Warning,
                format!("Range size {range} is dangerously large for counting sort memory constraints."),
            );
            // In a real robust system, might fallback to quick_sort here. But we will proceed for the skill demo.
        }

        let mut counts = vec![0usize; range];
        
        // Pass 1: Count frequency
        for &val in arr.iter() {
            let idx = (val - min_val) as usize;
            counts[idx] += 1;
        }

        AgentLogger::log(AgentFeedback::Step, "Frequency counting complete.");

        // Pass 2: Reconstruct array
        let mut target_idx = 0;
        for (i, &count) in counts.iter().enumerate() {
            let val = min_val + i as i32;
            for _ in 0..count {
                arr[target_idx] = val;
                target_idx += 1;
            }
        }

        AgentLogger::log(AgentFeedback::Success, "Reconstruction complete. Array sorted.");
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "sorting_searching.counting_sort", description = "Use this for solving counting sort problems. Trigger Keywords: sorting, searching, counting_sort. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_counting_sort(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Bottom-Up Merge Sort
/// CATEGORY: sorting-searching
/// DESCRIPTION: Sorts an array using an iterative bottom-up merge approach,
///              guaranteeing O(N log N) time while absolutely preventing stack overflow.
pub struct MergeSort;

impl Complexity for MergeSort {
    fn name(&self) -> &'static str {
        "Merge Sort (Iterative Bottom-Up)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N log N) — Strict O(log N) passes. In each pass, we merge all adjacent subarrays in O(N) time."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Requires a temporary buffer slice to merge elements back safely."
    }

    fn description(&self) -> &'static str {
        "Iteratively merges adjacent pairs of size 1 into size 2, then size 2 into size 4, etc. Requires zero recursion. Uses a persistent auxiliary buffer."
    }
}

impl MergeSort {
    /// Sorts `arr` in place using an iterative O(N log N) merge algorithm.
    pub fn solve<T: Ord + Clone>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }

        let n = arr.len();
        // Scratch buffer for merges
        let mut buffer = arr.to_vec();

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Iterative Bottom-Up Merge Sort on {} elements.", n),
        );

        let mut width = 1;
        while width < n {
            AgentLogger::log(AgentFeedback::Step, format!("Merging subarrays of width {width}."));
            
            for i in (0..n).step_by(2 * width) {
                let left = i;
                let mid = (i + width).min(n);
                let right = (i + 2 * width).min(n);
                
                // Merge operation: [left..mid] and [mid..right]
                Self::merge(&mut buffer, arr, left, mid, right);
            }
            
            // `buffer` now holds the stable sort state for this width pass.
            // Copy it completely back to `arr` for the next pass
            arr.clone_from_slice(&buffer);

            width *= 2;
        }

        AgentLogger::log(AgentFeedback::Success, "Merge sort completed.");
    }

    /// Internal merge helper. Merges `source[left..mid]` and `source[mid..right]` into `dest[left..right]`.
    fn merge<T: Ord + Clone>(dest: &mut [T], source: &[T], left: usize, mid: usize, right: usize) {
        let mut i = left;
        let mut j = mid;
        let mut k = left;

        while i < mid && j < right {
            if source[i] <= source[j] { // Stable comparison
                dest[k] = source[i].clone();
                i += 1;
            } else {
                dest[k] = source[j].clone();
                j += 1;
            }
            k += 1;
        }

        while i < mid {
            dest[k] = source[i].clone();
            i += 1;
            k += 1;
        }

        while j < right {
            dest[k] = source[j].clone();
            j += 1;
            k += 1;
        }
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "sorting_searching.merge_sort", description = "Use this for solving merge sort problems. Trigger Keywords: sorting, searching, merge_sort. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_merge_sort(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

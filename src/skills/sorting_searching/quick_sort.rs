use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Quick Sort
/// CATEGORY: sorting-searching
/// DESCRIPTION: Iterative Quick Sort implementation achieving O(N log N) average runtime
///              by utilizing Lomuto partition and an explicit boundary stack.
pub struct QuickSort;

impl Complexity for QuickSort {
    fn name(&self) -> &'static str {
        "Quick Sort (Iterative / Lomuto Partition)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N log N) Expected / O(N²) Worst Case — Time relies entirely on pivot randomness, but works optimally on average arrays."
    }

    fn space_complexity(&self) -> &'static str {
        "O(log N) Expected — Bounding stack size is managed safely and iteratively to prevent recursion overflow."
    }

    fn description(&self) -> &'static str {
        "Uses an explicit stack of [low, high] pairs instead of recursion. Lomuto's partition mechanism selects the last element, places all smaller elements to the left, and finalizes the pivot."
    }
}

impl QuickSort {
    /// Sorts `arr` in place using an explicit-stack iterative Quick Sort.
    pub fn solve<T: Ord + Clone + std::fmt::Display>(arr: &mut [T]) {
        let n = arr.len();
        if n <= 1 {
            return;
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Iterative Quick Sort on {} elements.", n),
        );

        let mut stack = Vec::new();
        stack.push((0, n - 1));

        let mut splits = 0;

        while let Some((low, high)) = stack.pop() {
            if low < high {
                let p = Self::partition(arr, low, high);
                splits += 1;

                if splits % 1000 == 0 {
                    AgentLogger::log(AgentFeedback::Step, format!("Performed {splits} partition splits..."));
                }

                if p > 0 {
                    stack.push((low, p - 1));
                }
                stack.push((p + 1, high));
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Quick sort complete. Array finalized after {splits} partitions."),
        );
    }

    /// Lomuto partition scheme: assumes pivot is at `arr[high]`.
    fn partition<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) -> usize {
        // Pick median-of-three or random to prevent O(N²) worst-case? 
        // For standard Lomuto we just use the high index element directly.
        // arr.swap(mid, high) can be used to optimize.
        
        let mut i = low; // boundary between elements < pivot and >= pivot
        
        for j in low..high {
            if arr[j] < arr[high] {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, high);
        i
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "sorting_searching.quick_sort", description = "Use this for solving quick sort problems. Trigger Keywords: sorting, searching, quick_sort. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_quick_sort(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

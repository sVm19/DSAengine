use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Peak Finder
/// CATEGORY: sorting-searching
/// DESCRIPTION: Finds a peak element in an array (an element strictly greater than
///              its neighbours) using binary search, achieving O(log N) runtime.
pub struct PeakFinder;

impl Complexity for PeakFinder {
    fn name(&self) -> &'static str {
        "Peak Element Finder (Gradient-Descent Binary Search)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(log N) — Half the search space is eliminated on each iteration by following the upward slope."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Bounding iterators consume constant space."
    }

    fn description(&self) -> &'static str {
        "Checks `arr[mid] < arr[mid+1]`. If true, the slope goes up to the right, meaning a peak must exist on the right. If false, a peak must exist on the left (including mid)."
    }
}

impl PeakFinder {
    /// Returns the index of any local peak in `nums`.
    /// 
    /// A peak is defined as `nums[i] > nums[i-1]` and `nums[i] > nums[i+1]`.
    /// Edges are treated as `-inf`.
    pub fn solve(nums: &[i32]) -> usize {
        if nums.is_empty() {
            return usize::MAX; // Or panic; returning MAX for valid safe handling if empty
        }

        let mut l = 0usize;
        let mut r = nums.len() - 1;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Peak finder initiated on array of length {}.", nums.len()),
        );

        while l < r {
            let mid = l + (r - l) / 2;
            AgentLogger::log(AgentFeedback::Step, format!("Checking gradient at mid={mid} (val= {} vs {}).", nums[mid], nums[mid+1]));

            if nums[mid] < nums[mid + 1] {
                // Slope is strictly increasing to the right
                l = mid + 1;
            } else {
                // Slope is decreasing or flat; a peak exists on the left (or is mid itself)
                r = mid;
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Peak element found at index {l} (value={}).", nums[l]),
        );

        l
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "sorting_searching.peak_finder", description = "Use this for solving peak finder problems. Trigger Keywords: sorting, searching, peak_finder. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_peak_finder(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Binary Search Template
/// CATEGORY: sorting-searching
/// DESCRIPTION: A robust `[l, r)` boundary binary search template that finds
///              the exact value, lower bound, or upper bound in a sorted array.
pub struct BinarySearchTemplate;

impl Complexity for BinarySearchTemplate {
    fn name(&self) -> &'static str {
        "Binary Search Template (Half-Open Interval)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(log N) — Halves the search space on each iteration."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Search boundaries maintained purely through two index registers."
    }

    fn description(&self) -> &'static str {
        "Uses [l, r) loop invariant (`while l < r`), preventing infinite loops seamlessly while supporting exact matching and insertion-point queries."
    }
}

impl BinarySearchTemplate {
    /// Returns the index of `target` in `arr`, or `None` if not found.
    pub fn exact_search<T: Ord + std::fmt::Display>(arr: &[T], target: &T) -> Option<usize> {
        let mut l = 0usize;
        let mut r = arr.len();

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Exact binary search for target '{target}' in array of length {r}."),
        );

        while l < r {
            let mid = l + (r - l) / 2;
            AgentLogger::log(AgentFeedback::Step, format!("Checking mid={mid} (value={}).", arr[mid]));
            
            match arr[mid].cmp(target) {
                std::cmp::Ordering::Equal => {
                    AgentLogger::log(AgentFeedback::Success, format!("Found '{target}' at index {mid}."));
                    return Some(mid);
                }
                std::cmp::Ordering::Less => l = mid + 1,
                std::cmp::Ordering::Greater => r = mid,
            }
        }

        AgentLogger::log(AgentFeedback::Warning, format!("Target '{target}' not found."));
        None
    }

    /// Returns the index of the first element `≥ target` (lower bound/insertion point).
    pub fn lower_bound<T: Ord + std::fmt::Display>(arr: &[T], target: &T) -> usize {
        let mut l = 0usize;
        let mut r = arr.len();

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Lower bound search for target '{target}' in array of length {r}."),
        );

        while l < r {
            let mid = l + (r - l) / 2;
            if arr[mid] < *target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        AgentLogger::log(AgentFeedback::Success, format!("Lower bound for '{target}' is at index {l}."));
        l
    }

    /// Returns the index of the first element `> target` (upper bound).
    pub fn upper_bound<T: Ord + std::fmt::Display>(arr: &[T], target: &T) -> usize {
        let mut l = 0usize;
        let mut r = arr.len();

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Upper bound search for target '{target}' in array of length {r}."),
        );

        while l < r {
            let mid = l + (r - l) / 2;
            if arr[mid] <= *target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        AgentLogger::log(AgentFeedback::Success, format!("Upper bound for '{target}' is at index {l}."));
        l
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "sorting_searching.binary_search_template", description = "Use this for solving binary search template problems. Trigger Keywords: sorting, searching, binary_search_template. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_binary_search_template(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

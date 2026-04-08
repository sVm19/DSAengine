use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Search in Rotated Sorted Array
/// CATEGORY: sorting-searching
/// DESCRIPTION: Finds a target value in a sorted array that has been rotated
///              at an unknown pivot, achieving O(log N) time via binary search.
pub struct RotatedSearch;

impl Complexity for RotatedSearch {
    fn name(&self) -> &'static str {
        "Search in Rotated Array (Segment-Aware Binary Search)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(log N) — Determines the sorted half and narrows the interval logarithmically."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Constant number of boundary pointers used."
    }

    fn description(&self) -> &'static str {
        "During binary search, at least one half of the array [l, mid] or [mid, r] must be strictly sorted. Identifies the sorted half, checks if the target falls within its range, and routes the search space accordingly."
    }
}

impl RotatedSearch {
    /// Returns the index of `target` in the rotated sorted array `nums`, or `None` if absent.
    pub fn solve(nums: &[i32], target: i32) -> Option<usize> {
        if nums.is_empty() {
            return None;
        }

        let mut l = 0usize;
        let mut r = nums.len() - 1;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Rotated array search for target {target} in array of length {}.", nums.len()),
        );

        while l <= r {
            let mid = l + (r - l) / 2;
            AgentLogger::log(AgentFeedback::Step, format!("Checking interval [{l}, {r}]; mid={mid} (val={}).", nums[mid]));

            if nums[mid] == target {
                AgentLogger::log(AgentFeedback::Success, format!("Target {target} found at index {mid}."));
                return Some(mid);
            }

            // Determine which half is properly sorted
            if nums[l] <= nums[mid] {
                // Left half is sorted
                if target >= nums[l] && target < nums[mid] {
                    // Target is strictly bounded within the sorted left half
                    if mid == 0 { break; }
                    r = mid - 1;
                } else {
                    // Target must be in the unsorted right half
                    l = mid + 1;
                }
            } else {
                // Right half is sorted
                if target > nums[mid] && target <= nums[r] {
                    // Target is strictly bounded within the sorted right half
                    l = mid + 1;
                } else {
                    // Target must be in the unsorted left half
                    if mid == 0 { break; }
                    r = mid - 1;
                }
            }
        }

        AgentLogger::log(AgentFeedback::Warning, format!("Target {target} not found in rotated array."));
        None
    }

    /// Finds the index of the minimum element (the pivot point).
    pub fn find_min(nums: &[i32]) -> usize {
        let mut l = 0usize;
        let mut r = nums.len() - 1;

        while l < r {
            let mid = l + (r - l) / 2;
            if nums[mid] > nums[r] {
                // Minimum must be to the right
                l = mid + 1;
            } else {
                // Minimum is at mid or to the left
                r = mid;
            }
        }
        
        AgentLogger::log(AgentFeedback::Success, format!("Rotated array minimum found at index {l}."));
        l
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct RotatedSearchRequest {
    pub nums: Vec<i32>,
    pub target: i32,
    pub include_pivot: Option<bool>,
}

#[macros::mcp_tool(name = "sorting_searching.rotated_search", description = "Use this for solving rotated search problems. Trigger Keywords: sorting, searching, rotated_search. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_rotated_search(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_rotated_search(payload: Value) -> DsaResult<ResultBox> {
    let req: RotatedSearchRequest = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid RotatedSearchRequest: {e}"),
        hint: "Provide 'nums' and 'target'.".to_string(),
    })?;

    if req.nums.is_empty() {
        return Err(DsaError::InvalidInput {
            message: "nums cannot be empty.".to_string(),
            hint: "Provide at least one value in 'nums'.".to_string(),
        });
    }

    let index = RotatedSearch::solve(&req.nums, req.target);
    let include_pivot = req.include_pivot.unwrap_or(false);
    let pivot = if include_pivot {
        Some(RotatedSearch::find_min(&req.nums))
    } else {
        None
    };

    let solver = RotatedSearch;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });

    Ok(ResultBox::success(json!({
        "index": index,
        "pivot_index": pivot
    }))
    .with_complexity(complexity)
    .with_description("Rotated-array search completed."))
}

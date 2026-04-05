use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Three Sum Solver
/// CATEGORY: arrays-strings
/// DESCRIPTION: Finds unique zero-sum triplets after an in-place sort.
pub struct ThreeSumSolver;

impl Complexity for ThreeSumSolver {
    fn name(&self) -> &'static str {
        "Three Sum Solver"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n^2) - Sorting is O(n log n), then each anchor uses one linear two-pointer sweep."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) auxiliary - Uses in-place sorting; output storage is excluded."
    }

    fn description(&self) -> &'static str {
        "Sorts once, fixes an anchor, and narrows a two-pointer pair while skipping duplicates."
    }
}

impl ThreeSumSolver {
    pub fn solve(nums: &mut [i32]) -> Vec<[i32; 3]> {
        if nums.len() < 3 {
            return Vec::new();
        }

        nums.sort_unstable();
        let mut triplets = Vec::new();

        for anchor in 0..nums.len() - 2 {
            if anchor > 0 && nums[anchor] == nums[anchor - 1] {
                continue;
            }
            if nums[anchor] > 0 {
                break;
            }

            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Anchoring triplet search at index {} with value {}.",
                    anchor, nums[anchor]
                ),
            );

            let mut left = anchor + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[anchor] + nums[left] + nums[right];
                if sum == 0 {
                    triplets.push([nums[anchor], nums[left], nums[right]]);
                    AgentLogger::log(
                        AgentFeedback::Success,
                        format!(
                            "Captured zero-sum triplet [{}, {}, {}].",
                            nums[anchor], nums[left], nums[right]
                        ),
                    );
                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        triplets
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "arrays_strings.three_sum_solver", description = "Use this for solving three sum solver problems. Trigger Keywords: three_sum_solver, three sum solver, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_three_sum_solver(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

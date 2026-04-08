use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Dutch National Flag Algorithm
/// CATEGORY: arrays-strings
/// DESCRIPTION: Sorts an array of 0s, 1s, and 2s in linear time using three pointers.
pub struct DutchNationalFlag;

impl Complexity for DutchNationalFlag {
    fn name(&self) -> &'static str {
        "Dutch National Flag (3-Way Partition)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Single pass through the array."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - Sorting is done in-place."
    }

    fn description(&self) -> &'static str {
        "Efficiently partitions an array into three sections. Essential for QuickSort optimization."
    }
}

impl DutchNationalFlag {
    pub fn solve(nums: &mut [i32]) {
        if nums.is_empty() {
            return;
        }

        let mut low = 0usize;
        let mut mid = 0usize;
        let mut high = nums.len();

        AgentLogger::log(
            AgentFeedback::Info,
            "Starting three-way partition for values {0, 1, 2}.",
        );

        while mid < high {
            match nums[mid] {
                0 => {
                    nums.swap(low, mid);
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Swapped 0 from index {} into low partition {}.", mid, low),
                    );
                    low += 1;
                    mid += 1;
                }
                1 => {
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Left 1 at middle index {} and advanced the scanner.", mid),
                    );
                    mid += 1;
                }
                2 => {
                    high -= 1;
                    nums.swap(mid, high);
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Swapped 2 from index {} into high partition {}.", mid, high),
                    );
                }
                value => {
                    AgentLogger::log(
                        AgentFeedback::Warning,
                        format!("Encountered unexpected value {} at index {}.", value, mid),
                    );
                    high -= 1;
                    nums.swap(mid, high);
                }
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            "Three-way partition completed successfully.",
        );
    }

    /// Sorts the array in-place while tracing the pointer movements.
    pub fn sort(nums: &mut [i32]) {
        Self::solve(nums);
    }

    /// Explains the pointer logic for the AI Agent.
    pub fn explain_pointers() {
        println!("📍 [POINTER ROLES]:");
        println!("  - LOW: Boundary for 0s (everything before is 0).");
        println!("  - MID: Current element being scanned.");
        println!("  - HIGH: Boundary for 2s (everything after is 2).");
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct DutchNationalFlagRequest {
    pub nums: Vec<i32>,
}

#[macros::mcp_tool(name = "arrays_strings.dutch_national_flag", description = "Use this for solving dutch national flag problems. Trigger Keywords: dutch_national_flag, dutch national flag, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_dutch_national_flag(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_dutch_national_flag(payload: Value) -> DsaResult<ResultBox> {
    let req: DutchNationalFlagRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid DutchNationalFlagRequest: {e}"),
            hint: "Provide 'nums' containing 0, 1, and 2 values.".to_string(),
        })?;

    let mut nums = req.nums;
    DutchNationalFlag::solve(&mut nums);
    let solver = DutchNationalFlag;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });

    Ok(ResultBox::success(json!({
        "sorted": nums
    }))
    .with_complexity(complexity)
    .with_description("Dutch National Flag partition completed."))
}

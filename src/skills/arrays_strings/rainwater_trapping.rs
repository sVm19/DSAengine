use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Trapping Rain Water
/// CATEGORY: arrays-strings
/// DESCRIPTION: Calculates how much water can be trapped between bars of different heights.
pub struct RainwaterTrapping;

impl Complexity for RainwaterTrapping {
    fn name(&self) -> &'static str {
        "Trapping Rain Water (Two-Pointer)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Single pass from both ends."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - Constant space for pointers and max trackers."
    }

    fn description(&self) -> &'static str {
        "Uses two pointers to find the water level based on the shorter of the two bounding walls."
    }
}

impl RainwaterTrapping {
    pub fn solve(heights: &[i32]) -> i32 {
        if heights.is_empty() {
            return 0;
        }

        let mut left = 0;
        let mut right = heights.len() - 1;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut total_water = 0;

        AgentLogger::log(
            AgentFeedback::Info,
            "Starting Rainwater Trapping analysis...",
        );

        while left < right {
            if heights[left] < heights[right] {
                if heights[left] >= left_max {
                    left_max = heights[left];
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Raised left_max to {} at index {}.", left_max, left),
                    );
                } else {
                    total_water += left_max - heights[left];
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!(
                            "Index {}: Trapped {} units of water",
                            left,
                            left_max - heights[left]
                        ),
                    );
                }
                left += 1;
            } else {
                if heights[right] >= right_max {
                    right_max = heights[right];
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Raised right_max to {} at index {}.", right_max, right),
                    );
                } else {
                    total_water += right_max - heights[right];
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!(
                            "Index {}: Trapped {} units of water",
                            right,
                            right_max - heights[right]
                        ),
                    );
                }
                right -= 1;
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Total Water Trapped: {}", total_water),
        );
        total_water
    }

    /// Calculates total water trapped and logs the height decisions.
    pub fn calculate(heights: &[i32]) -> i32 {
        Self::solve(heights)
    }

    /// Explains the core logic for the AI Agent.
    pub fn explain_logic() {
        println!("💧 [LOGIC]: The amount of water at any point is determined by:");
        println!("   min(max_left_height, max_right_height) - current_height.");
        println!("   By moving the pointer with the smaller height, we guarantee the 'min' part.");
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "arrays_strings.rainwater_trapping", description = "Use this for solving rainwater trapping problems. Trigger Keywords: rainwater_trapping, rainwater trapping, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_rainwater_trapping(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

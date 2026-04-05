use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Container Water
/// CATEGORY: arrays-strings
/// DESCRIPTION: Computes the maximum water area using the classic two-pointer sweep.
pub struct ContainerWater;

impl Complexity for ContainerWater {
    fn name(&self) -> &'static str {
        "Container Water"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Each boundary pointer moves inward at most once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - Tracks only two pointers and the best area."
    }

    fn description(&self) -> &'static str {
        "Maximizes width x min(height[left], height[right]) by always discarding the shorter wall."
    }
}

impl ContainerWater {
    pub fn solve(heights: &[i32]) -> i32 {
        if heights.len() < 2 {
            return 0;
        }

        let mut left = 0usize;
        let mut right = heights.len() - 1;
        let mut best = 0;

        while left < right {
            let width = (right - left) as i32;
            let limiting_height = heights[left].min(heights[right]);
            let area = width * limiting_height;
            best = best.max(area);

            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Evaluated walls ({}, {}) -> width={}, limiting_height={}, area={}.",
                    left, right, width, limiting_height, area
                ),
            );

            if heights[left] <= heights[right] {
                left += 1;
                AgentLogger::log(
                    AgentFeedback::Step,
                    "Moved the left wall inward because it was the limiting height.",
                );
            } else {
                right -= 1;
                AgentLogger::log(
                    AgentFeedback::Step,
                    "Moved the right wall inward because it was the limiting height.",
                );
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Maximum container area found: {}.", best),
        );
        best
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "arrays_strings.container_water", description = "Use this for solving container water problems. Trigger Keywords: container_water, container water, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_container_water(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

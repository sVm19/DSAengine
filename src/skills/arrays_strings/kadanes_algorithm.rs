use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Kadane's Algorithm (Visualizer)
/// CATEGORY: arrays-strings
/// DESCRIPTION: Finds the maximum subarray sum and visualizes the local vs global decision.
pub struct KadanesAlgorithm;

impl Complexity for KadanesAlgorithm {
    fn name(&self) -> &'static str {
        "Kadane's Subarray Visualizer"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n)"
    }

    fn space_complexity(&self) -> &'static str {
        "O(1)"
    }

    fn description(&self) -> &'static str {
        "Computes the maximum sum of a contiguous subarray using a single pass."
    }
}

impl KadanesAlgorithm {
    pub fn solve(nums: &[i32]) -> Option<(i32, usize, usize)> {
        if nums.is_empty() {
            AgentLogger::log(AgentFeedback::Warning, "Empty array provided.");
            return None;
        }

        let mut current_max = nums[0];
        let mut global_max = nums[0];
        let mut best_start = 0;
        let mut best_end = 0;
        let mut temp_start = 0;

        AgentLogger::log(
            AgentFeedback::Info,
            "Scanning contiguous sums with Kadane's recurrence.",
        );

        for i in 1..nums.len() {
            let val = nums[i];

            if val > current_max + val {
                current_max = val;
                temp_start = i;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Restarted the candidate window at index {} with value {}.",
                        i, val
                    ),
                );
            } else {
                current_max += val;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Extended the current window through index {}. Running sum={}.",
                        i, current_max
                    ),
                );
            }

            if current_max > global_max {
                global_max = current_max;
                best_start = temp_start;
                best_end = i;
                AgentLogger::log(
                    AgentFeedback::Success,
                    format!(
                        "Promoted a new best subarray sum {} over range [{}..={}].",
                        global_max, best_start, best_end
                    ),
                );
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Finished Kadane scan with best sum {}.", global_max),
        );
        Some((global_max, best_start, best_end))
    }

    /// Solves the problem while providing a step-by-step visual trace.
    pub fn solve_with_visual(nums: &[i32]) -> i32 {
        Self::solve(nums)
            .map(|(sum, start, end)| {
                AgentLogger::log(
                    AgentFeedback::Info,
                    format!("Best subarray resolved to range [{}..={}].", start, end),
                );
                sum
            })
            .unwrap_or(0)
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct KadanesAlgorithmRequest {
    pub nums: Vec<i32>,
}

#[macros::mcp_tool(name = "arrays_strings.kadanes_algorithm", description = "Use this for solving kadanes algorithm problems. Trigger Keywords: kadanes_algorithm, kadanes algorithm, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_kadanes_algorithm(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_kadanes_algorithm(payload: Value) -> DsaResult<ResultBox> {
    let req: KadanesAlgorithmRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid KadanesAlgorithmRequest: {e}"),
            hint: "Provide 'nums' array.".to_string(),
        })?;

    let best = KadanesAlgorithm::solve(&req.nums);
    let solver = KadanesAlgorithm;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });

    Ok(ResultBox::success(json!({
        "best_subarray": best.map(|(sum, start, end)| json!({"sum": sum, "start": start, "end": end}))
    }))
    .with_complexity(complexity)
    .with_description("Kadane maximum-subarray scan completed."))
}

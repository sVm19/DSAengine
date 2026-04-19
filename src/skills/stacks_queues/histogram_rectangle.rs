use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Largest Rectangle in Histogram
/// CATEGORY: stacks-queues
/// DESCRIPTION: Finds the largest rectangular area possible in a histogram
///              using an ascending monotonic stack to establish boundaries rapidly.
pub struct HistogramRectangle;

impl Complexity for HistogramRectangle {
    fn name(&self) -> &'static str {
        "Largest Rectangle in Histogram (Monotonic Stack)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Single pass over heights; each bar is pushed and popped exactly once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Stack size memory limit in cases of perfectly ascending histograms."
    }

    fn description(&self) -> &'static str {
        "Maintains a stack of indices representing an ascending wave of heights. Upon seeing a drop in height, previously stacked bars are finalized because their right boundary has been discovered. The left boundary is the element below them on the stack."
    }
}

impl HistogramRectangle {
    /// Returns the maximum area of a rectangle in the histogram.
    pub fn solve(heights: &[i32]) -> i32 {
        let n = heights.len();
        let mut max_area = 0;
        let mut stack: Vec<usize> = Vec::with_capacity(n); // Stores indices

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Calculating max rectangle for histogram of {} bars.", n),
        );

        for i in 0..=n {
            // Virtual right boundary value is 0 to flush remaining stack elements at the end
            let current_h = if i == n { 0 } else { heights[i] };

            while !stack.is_empty() && current_h < heights[*stack.last().unwrap()] {
                let h_idx = stack.pop().unwrap();
                let height = heights[h_idx];

                // Right boundary is exclusive current index `i`.
                // Left boundary is exclusive stack top AFTER popping, or -1 if empty.
                let width = if stack.is_empty() {
                    i as i32
                } else {
                    (i - *stack.last().unwrap() - 1) as i32
                };

                let area = height * width;
                if area > max_area {
                    max_area = area;
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("New max area {max_area} found: height={height}, width={width}."),
                    );
                }
            }

            stack.push(i);
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Max histogram rectangle area is {max_area}."),
        );

        max_area
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "histogram_rectangle",
    description = "Use this for solving histogram rectangle problems. Trigger Keywords: histogram_rectangle, histogram rectangle, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_histogram_rectangle(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_histogram_rectangle(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        heights: Vec<i32>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'heights' (array of i32).".to_string(),
    })?;

    let result = {
        let area = HistogramRectangle::solve(&req.heights);
        json!({ "max_area": area })
    };

    let solver = HistogramRectangle;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["max_area"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Histogram rectangle computed."))
}

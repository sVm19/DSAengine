use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Stacks Visualizer
/// CATEGORY: stacks-queues
/// DESCRIPTION: Outputs string representations of stacks and deques during algorithmic execution
///              to verify structural monotonicity and logic state.
pub struct Visualizer;

impl Complexity for Visualizer {
    fn name(&self) -> &'static str {
        "Stack/Deque Visualizer"
    }

    fn time_complexity(&self) -> &'static str {
        "O(S) — Iterates through stack/deque items to serialize output."
    }

    fn space_complexity(&self) -> &'static str {
        "O(S) — Allocates string memory proportional to the size of the structure."
    }

    fn description(&self) -> &'static str {
        "Formats arbitrary slices acting as stacks into clear console streams. Assists in auditing monotonic integrity."
    }
}

impl Visualizer {
    /// Visualizes a stack. Assumes `stack.last()` is the top.
    pub fn view_stack<T: std::fmt::Display>(stack: &[T], name: &str) -> String {
        let mut out = format!("{name}: [BOTTOM] ");
        for item in stack {
            out.push_str(&format!("{item} "));
        }
        out.push_str("<- [TOP]");

        AgentLogger::log(
            AgentFeedback::Step,
            format!("Stack visualized: length={}.", stack.len()),
        );

        out
    }

    /// Visualizes a deque interacting with a slice (like in Sliding Window Maximum).
    /// Resolves indices to visual values if provided.
    pub fn view_deque_indices<T: std::fmt::Display>(
        deque: &std::collections::VecDeque<usize>,
        arr: &[T],
        name: &str,
    ) -> String {
        let mut out = format!("{name}: [FRONT max/oldest] ");
        for &idx in deque {
            if idx < arr.len() {
                out.push_str(&format!("{}@{} ", arr[idx], idx));
            } else {
                out.push_str(&format!("ERR@{} ", idx));
            }
        }
        out.push_str("<- [BACK min/newest]");

        AgentLogger::log(
            AgentFeedback::Step,
            format!(
                "Deque indices mapped and visualized (size={}).",
                deque.len()
            ),
        );

        out
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "visualizer",
    description = "Use this for solving visualizer problems. Trigger Keywords: visualizer, visualizer, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_sq_visualizer(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_sq_visualizer(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        stack: Option<Vec<i32>>,
        name: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'stack' (array) and optional 'name'.".to_string(),
    })?;

    let result = {
        let stack = req.stack.unwrap_or_default();
        let name = req.name.as_deref().unwrap_or("Stack");
        let viz = Visualizer::view_stack(&stack, name);
        json!({ "visualization": viz })
    };

    let solver = Visualizer;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["visualize"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Visualizer completed."))
}

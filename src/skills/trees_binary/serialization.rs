use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::VecDeque;

/// SKILL: Tree Serialization
/// CATEGORY: trees-binary
/// DESCRIPTION: Serializes and deserializes a binary tree into a flat string sequence
///              utilizing a level-order (BFS) queuing methodology.
pub struct Serialization;

impl Complexity for Serialization {
    fn name(&self) -> &'static str {
        "Tree Serialization (Level-Order Stringification)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Enqueues and evaluates all live nodes explicitly."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Both the BFS queue bound and the string extraction layer occupy linear footprints naturally."
    }

    fn description(&self) -> &'static str {
        "Nodes are enqueued left-to-right. Null boundaries are converted to explicitly coded strings (e.g. 'N') rendering exactly reconstructable arrays."
    }
}

impl Serialization {
    /// Serializes a tree to a comma-separated format.
    pub fn solve_serialize(nodes: &[(usize, usize, i32)], root: usize) -> String {
        let null = usize::MAX;
        if root == null {
            return String::new();
        }

        let mut out = String::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);

        AgentLogger::log(
            AgentFeedback::Info,
            "Serializing binary tree level-by-level.",
        );

        while let Some(curr) = queue.pop_front() {
            if curr == null {
                out.push_str("N,");
            } else {
                out.push_str(&format!("{},", nodes[curr].2));
                queue.push_back(nodes[curr].0);
                queue.push_back(nodes[curr].1);
            }
        }

        AgentLogger::log(AgentFeedback::Success, "Tree serialization complete.");
        out
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "serialization",
    description = "Use this for solving serialization problems. Trigger Keywords: serialization, serialization, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_serialization(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_serialization(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        nodes: Vec<(usize, usize, i32)>,
        root: usize,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'nodes' and 'root'.".to_string(),
    })?;

    let result = {
        let s = Serialization::solve_serialize(&req.nodes, req.root);
        json!({ "serialized": s })
    };

    let solver = Serialization;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["serialize"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Serialization completed."))
}

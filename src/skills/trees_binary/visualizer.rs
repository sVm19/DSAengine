use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::VecDeque;

/// SKILL: Binary Tree Visualizer
/// CATEGORY: trees-binary
/// DESCRIPTION: Outputs tree layouts as readable ASCII streams using BFS queueing.
pub struct Visualizer;

impl Complexity for Visualizer {
    fn name(&self) -> &'static str {
        "Tree ASCII Visualizer (Level-Order Generation)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Every node generates string metadata definitively once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Bounded tightly to level capacities and explicit output blocks."
    }

    fn description(&self) -> &'static str {
        "Traverses rows extracting strings, grouping elements logically across horizontal lines corresponding directly to actual tree levels."
    }
}

impl Visualizer {
    pub fn to_string(nodes: &[(usize, usize, i32)], root: usize) -> String {
        let null = usize::MAX;
        if root == null {
            return String::from("[Empty Tree]");
        }

        let mut out = String::from("\n[Tree Levels]\n");
        let mut queue = VecDeque::new();
        queue.push_back(root);

        let mut level = 0;

        AgentLogger::log(
            AgentFeedback::Info,
            "Rendering discrete tree topological array limits.",
        );

        while !queue.is_empty() {
            let limit = queue.len();
            out.push_str(&format!("L{level}: "));

            for _ in 0..limit {
                let curr = queue.pop_front().unwrap();
                out.push_str(&format!("{} ", nodes[curr].2));

                if nodes[curr].0 != null {
                    queue.push_back(nodes[curr].0);
                }
                if nodes[curr].1 != null {
                    queue.push_back(nodes[curr].1);
                }
            }
            out.push('\n');
            level += 1;
        }

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
    match handle_tree_visualizer(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_tree_visualizer(payload: Value) -> DsaResult<ResultBox> {
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
        let viz = Visualizer::to_string(&req.nodes, req.root);
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

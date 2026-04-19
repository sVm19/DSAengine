use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::VecDeque;

/// SKILL: Height / Depth Calculator
/// CATEGORY: trees-binary
/// DESCRIPTION: Calculates the maximum depth of a binary tree iteratively using a
///              level-order queue (BFS) to prevent recursion-depth limits.
pub struct HeightCalc;

impl Complexity for HeightCalc {
    fn name(&self) -> &'static str {
        "Tree Max Depth (Iterative Level-Order)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Visits each node exactly once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(W) — Sized to the maximum width of the tree (O(N) in worst case)."
    }

    fn description(&self) -> &'static str {
        "Uses a queue to track level depths. Pops all nodes of the current level while pushing all their children, incrementing the depth counter per level."
    }
}

impl HeightCalc {
    /// Returns the maximum depth of the binary tree.
    /// `nodes[i] = (left_idx, right_idx, value)`.
    pub fn solve(nodes: &[(usize, usize, i32)], root: usize) -> u32 {
        let null = usize::MAX;
        if root == null {
            return 0;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut depth = 0;

        AgentLogger::log(AgentFeedback::Info, "Calculating max depth iteratively.");

        while !queue.is_empty() {
            let level_size = queue.len();
            depth += 1;

            for _ in 0..level_size {
                let curr = queue.pop_front().unwrap();
                let left = nodes[curr].0;
                let right = nodes[curr].1;

                if left != null {
                    queue.push_back(left);
                }
                if right != null {
                    queue.push_back(right);
                }
            }
        }

        AgentLogger::log(AgentFeedback::Success, format!("Tree depth is {depth}."));
        depth
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "height_calc",
    description = "Use this for solving height calc problems. Trigger Keywords: height_calc, height calc, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_height_calc(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_height_calc(payload: Value) -> DsaResult<ResultBox> {
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
        let height = HeightCalc::solve(&req.nodes, req.root);
        json!({ "height": height })
    };

    let solver = HeightCalc;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["height"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("HeightCalc completed."))
}

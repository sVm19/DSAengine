use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Path Sum Evaluator
/// CATEGORY: trees-binary
/// DESCRIPTION: Identifies if any root-to-leaf path sums to a target value.
pub struct PathSum;

impl Complexity for PathSum {
    fn name(&self) -> &'static str {
        "Path Sum Evaluator (Iterative DFS Stack)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Every node is explored along its path from the root."
    }

    fn space_complexity(&self) -> &'static str {
        "O(H) — Height of the tree bounds the maximum stack depth."
    }

    fn description(&self) -> &'static str {
        "Pushes (node, running_sum) to a custom iteration stack. Validates leaf nodes safely, dropping recursive overhead."
    }
}

impl PathSum {
    pub fn solve(nodes: &[(usize, usize, i32)], root: usize, target: i32) -> bool {
        let null = usize::MAX;
        if root == null {
            return false;
        }

        let mut stack = Vec::new();
        stack.push((root, nodes[root].2));

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Searching for root-to-leaf path sum of {target}."),
        );

        while let Some((node_idx, curr_sum)) = stack.pop() {
            let left = nodes[node_idx].0;
            let right = nodes[node_idx].1;

            if left == null && right == null && curr_sum == target {
                AgentLogger::log(
                    AgentFeedback::Success,
                    format!("Path sum found culminating at leaf index {node_idx}."),
                );
                return true;
            }

            if left != null {
                stack.push((left, curr_sum + nodes[left].2));
            }
            if right != null {
                stack.push((right, curr_sum + nodes[right].2));
            }
        }

        AgentLogger::log(
            AgentFeedback::Warning,
            "No matching root-to-leaf path sum found.",
        );
        false
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "path_sum",
    description = "Use this for solving path sum problems. Trigger Keywords: path_sum, path sum, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_path_sum(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_path_sum(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        nodes: Vec<(usize, usize, i32)>,
        root: usize,
        target: i32,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'nodes', 'root', 'target'.".to_string(),
    })?;

    let result = {
        let has = PathSum::solve(&req.nodes, req.root, req.target);
        json!({ "has_path": has, "target": req.target })
    };

    let solver = PathSum;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["check"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("PathSum completed."))
}

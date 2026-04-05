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
        if root == null { return false; }

        let mut stack = Vec::new();
        stack.push((root, nodes[root].2));

        AgentLogger::log(AgentFeedback::Info, format!("Searching for root-to-leaf path sum of {target}."));

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

        AgentLogger::log(AgentFeedback::Warning, "No matching root-to-leaf path sum found.");
        false
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "path_sum", description = "Use this for solving path sum problems. Trigger Keywords: path_sum, path sum, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

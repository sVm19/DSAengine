use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Binary Tree Maximum Path Sum
/// CATEGORY: trees-binary
/// DESCRIPTION: Finds the maximum path sum between any two nodes in a tree
///              using an iterative post-order traversal simulation to gather state.
pub struct MaxPathSum;

impl Complexity for MaxPathSum {
    fn name(&self) -> &'static str {
        "Maximum Path Sum (Iterative Post-order DP)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Every node visited essentially twice to simulate post-order unwinding."
    }

    fn space_complexity(&self) -> &'static str {
        "O(H) — Height of tree for the backtracking path-stack."
    }

    fn description(&self) -> &'static str {
        "Simulates post-order traversal by using a `last_visited` tracker. Propagates maximum branch sums upwards while continually updating a global max path via `left + right + root`."
    }
}

impl MaxPathSum {
    pub fn solve(nodes: &[(usize, usize, i32)], root: usize) -> i32 {
        let null = usize::MAX;
        if root == null {
            return 0;
        }

        let mut stack = Vec::new();
        let mut curr = root;
        let mut last_visited = null;
        let mut max_sum = i32::MIN;

        // DP array to map each node's maximum unbranched path down to leaves
        let mut max_branch = vec![0i32; nodes.len()];

        AgentLogger::log(
            AgentFeedback::Info,
            "Traversing tree iteratively to find global maximum path sum.",
        );

        while curr != null || !stack.is_empty() {
            if curr != null {
                stack.push(curr);
                curr = nodes[curr].0;
            } else {
                let peek = *stack.last().unwrap();
                let right = nodes[peek].1;

                if right != null && last_visited != right {
                    curr = right;
                } else {
                    let node_idx = stack.pop().unwrap();
                    let left = nodes[node_idx].0;

                    let left_branch = if left != null {
                        max_branch[left].max(0)
                    } else {
                        0
                    };
                    let right_branch = if right != null {
                        max_branch[right].max(0)
                    } else {
                        0
                    };
                    let node_val = nodes[node_idx].2;

                    // Update global max evaluating if the path arched over this node
                    max_sum = max_sum.max(node_val + left_branch + right_branch);

                    // Report to parent
                    max_branch[node_idx] = node_val + left_branch.max(right_branch);

                    last_visited = node_idx;
                }
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Global max path sum is {max_sum}."),
        );
        max_sum
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "max_path_sum",
    description = "Use this for solving max path sum problems. Trigger Keywords: max_path_sum, max path sum, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_max_path_sum(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_max_path_sum(payload: Value) -> DsaResult<ResultBox> {
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
        let s = MaxPathSum::solve(&req.nodes, req.root);
        json!({ "max_path_sum": s })
    };

    let solver = MaxPathSum;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["compute"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("MaxPathSum completed."))
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Balanced Binary Tree Checker
/// CATEGORY: trees-binary
/// DESCRIPTION: Verifies if a binary tree is height-balanced iteratively using
///              post-order traversal DP to compute subgroup heights globally.
pub struct BalanceChecker;

impl Complexity for BalanceChecker {
    fn name(&self) -> &'static str {
        "Balanced Tree Checker (Iterative Post-Order Heights)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Every node is visited systematically to compute its sub-heights entirely from scratch iteratively."
    }

    fn space_complexity(&self) -> &'static str {
        "O(H) — Strictly stack-bounded to tree height to avoid runaway recursion."
    }

    fn description(&self) -> &'static str {
        "Utilizes an iterative post-order emulator to pull height variables upwards efficiently. Unbalanced subtrees bubble up a failure flag halting height derivations."
    }
}

impl BalanceChecker {
    pub fn solve(nodes: &[(usize, usize, i32)], root: usize) -> bool {
        let null = usize::MAX;
        if root == null {
            return true;
        }

        let mut stack = Vec::new();
        let mut curr = root;
        let mut last_visited = null;
        let mut heights = vec![0i32; nodes.len()];

        AgentLogger::log(
            AgentFeedback::Info,
            "Evaluating tree balance iteratively via post-order traversal.",
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

                    let hl = if left != null { heights[left] } else { 0 };
                    let hr = if right != null { heights[right] } else { 0 };

                    if hl == -1 || hr == -1 || (hl - hr).abs() > 1 {
                        AgentLogger::log(
                            AgentFeedback::Warning,
                            format!("Balance failure originating around node {node_idx}."),
                        );
                        return false;
                    }

                    heights[node_idx] = 1 + hl.max(hr);
                    last_visited = node_idx;
                }
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            "Tree is completely height-balanced.",
        );
        true
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "balance_checker",
    description = "Use this for solving balance checker problems. Trigger Keywords: balance_checker, balance checker, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_balance_checker(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_balance_checker(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        nodes: Vec<(usize, usize, i32)>,
        root: usize,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'nodes' as [(left,right,val)] and 'root'.".to_string(),
    })?;

    let result = {
        let balanced = BalanceChecker::solve(&req.nodes, req.root);
        json!({ "is_balanced": balanced })
    };

    let solver = BalanceChecker;
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
    .with_description("BalanceChecker completed."))
}

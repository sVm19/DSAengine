use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::VecDeque;

/// SKILL: Same Tree
/// CATEGORY: trees-binary
/// DESCRIPTION: Evaluates whether two binary trees are structurally identical
///              with matching node values, using an iterative parallel traversal.
pub struct SameTree;

impl Complexity for SameTree {
    fn name(&self) -> &'static str {
        "Same Tree Checker (Iterative Parallel Zip)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(min(N, M)) — Halts and returns false upon finding the first discrepancy."
    }

    fn space_complexity(&self) -> &'static str {
        "O(min(H_P, H_Q)) — Queue sizes are bounded by the height of the trees during parallel descent."
    }

    fn description(&self) -> &'static str {
        "Enqueues node pairs from both trees simultaneously. Checks equality of values and structural presence at each step, returning false purely iteratively if bounds or values mismatch."
    }
}

impl SameTree {
    pub fn solve(
        p_nodes: &[(usize, usize, i32)],
        p: usize,
        q_nodes: &[(usize, usize, i32)],
        q: usize,
    ) -> bool {
        let null = usize::MAX;
        let mut queue = VecDeque::new();
        queue.push_back((p, q));

        AgentLogger::log(
            AgentFeedback::Info,
            "Checking tree isomorphism iteratively.",
        );

        while let Some((curr_p, curr_q)) = queue.pop_front() {
            if curr_p == null && curr_q == null {
                continue;
            }
            if curr_p == null || curr_q == null {
                AgentLogger::log(AgentFeedback::Warning, "Structural mismatch detected.");
                return false;
            }

            if p_nodes[curr_p].2 != q_nodes[curr_q].2 {
                AgentLogger::log(AgentFeedback::Warning, "Value mismatch detected.");
                return false;
            }

            queue.push_back((p_nodes[curr_p].0, q_nodes[curr_q].0));
            queue.push_back((p_nodes[curr_p].1, q_nodes[curr_q].1));
        }

        AgentLogger::log(
            AgentFeedback::Success,
            "Trees are structurally and valularly identical.",
        );
        true
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "same_tree",
    description = "Use this for solving same tree problems. Trigger Keywords: same_tree, same tree, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_same_tree(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_same_tree(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        p_nodes: Vec<(usize, usize, i32)>,
        p_root: usize,
        q_nodes: Vec<(usize, usize, i32)>,
        q_root: usize,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide two trees: p_nodes+p_root and q_nodes+q_root.".to_string(),
    })?;

    let result = {
        let same = SameTree::solve(&req.p_nodes, req.p_root, &req.q_nodes, req.q_root);
        json!({ "is_same": same })
    };

    let solver = SameTree;
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
    .with_description("SameTree completed."))
}

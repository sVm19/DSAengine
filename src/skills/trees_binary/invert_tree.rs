use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::VecDeque;

/// SKILL: Invert Tree
/// CATEGORY: trees-binary
/// DESCRIPTION: Iteratively flips all left and right children in a binary tree.
pub struct InvertTree;

impl Complexity for InvertTree {
    fn name(&self) -> &'static str {
        "Invert Binary Tree (Iterative Swap)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Every node is visited once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(W) — Uses a queue bounded by the maximum width of the tree."
    }

    fn description(&self) -> &'static str {
        "Iterates layer by layer (BFS). At each node, explicitly swaps the left and right index pointers in the arena representation."
    }
}

impl InvertTree {
    /// Inverts the binary tree in place. Returns the root index.
    pub fn solve(nodes: &mut [(usize, usize, i32)], root: usize) -> usize {
        let null = usize::MAX;
        if root == null { return null; }

        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut swaps = 0;

        AgentLogger::log(AgentFeedback::Info, "Inverting binary tree iteratively.");

        while let Some(curr) = queue.pop_front() {
            // Swap children
            let left = nodes[curr].0;
            let right = nodes[curr].1;
            
            nodes[curr].0 = right;
            nodes[curr].1 = left;
            swaps += 1;

            if left != null { queue.push_back(left); }
            if right != null { queue.push_back(right); }
        }

        AgentLogger::log(AgentFeedback::Success, format!("Tree inverted with {swaps} pointer swaps."));
        root
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "invert_tree", description = "Use this for solving invert tree problems. Trigger Keywords: invert_tree, invert tree, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

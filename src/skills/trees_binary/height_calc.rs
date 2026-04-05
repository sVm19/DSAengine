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
        if root == null { return 0; }

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

                if left != null { queue.push_back(left); }
                if right != null { queue.push_back(right); }
            }
        }

        AgentLogger::log(AgentFeedback::Success, format!("Tree depth is {depth}."));
        depth
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "height_calc", description = "Use this for solving height calc problems. Trigger Keywords: height_calc, height calc, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

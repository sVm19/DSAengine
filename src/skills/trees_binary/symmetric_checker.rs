use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::VecDeque;

/// SKILL: Symmetric Tree Checker
/// CATEGORY: trees-binary
/// DESCRIPTION: Checks if a tree is a mirror of itself using an iterative BFS check.
pub struct SymmetricChecker;

impl Complexity for SymmetricChecker {
    fn name(&self) -> &'static str {
        "Symmetric Tree Checker (Iterative BFS Mirror)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Evaluates every pair of mirrored nodes."
    }

    fn space_complexity(&self) -> &'static str {
        "O(W) — Sized by the maximum width of a tree level in the dual-evaluation queue."
    }

    fn description(&self) -> &'static str {
        "Pops two nodes simultaneously. Validates they match. Enqueues their children in mirrored order: (Left1, Right2) then (Right1, Left2)."
    }
}

impl SymmetricChecker {
    pub fn solve(nodes: &[(usize, usize, i32)], root: usize) -> bool {
        let null = usize::MAX;
        if root == null { return true; }

        let mut queue = VecDeque::new();
        queue.push_back((nodes[root].0, nodes[root].1));

        AgentLogger::log(AgentFeedback::Info, "Evaluating tree symmetry via iterative BFS mirror match.");

        while let Some((t1, t2)) = queue.pop_front() {
            if t1 == null && t2 == null { continue; }
            if t1 == null || t2 == null {
                AgentLogger::log(AgentFeedback::Warning, "Symmetry broken: structural mismatch.");
                return false;
            }

            if nodes[t1].2 != nodes[t2].2 {
                AgentLogger::log(AgentFeedback::Warning, "Symmetry broken: value mismatch.");
                return false;
            }

            queue.push_back((nodes[t1].0, nodes[t2].1)); // Outer pair
            queue.push_back((nodes[t1].1, nodes[t2].0)); // Inner pair
        }

        AgentLogger::log(AgentFeedback::Success, "Tree is perfectly symmetric.");
        true
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "symmetric_checker", description = "Use this for solving symmetric checker problems. Trigger Keywords: symmetric_checker, symmetric checker, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

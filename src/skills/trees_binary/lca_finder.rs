use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Lowest Common Ancestor (LCA)
/// CATEGORY: trees-binary
/// DESCRIPTION: Finds the LCA of two nodes via iterative path tracking.
pub struct LcaFinder;

impl Complexity for LcaFinder {
    fn name(&self) -> &'static str {
        "Lowest Common Ancestor (Iterative Path Matching)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Extracts paths to both targets using DFS, then matches diverging points."
    }

    fn space_complexity(&self) -> &'static str {
        "O(H) — Two arrays tracking node indices down to depth H."
    }

    fn description(&self) -> &'static str {
        "An elegant, strictly iterative mechanism replacing typical structural recursion. Collects full paths to p and q simultaneously, then zips them until they diverge to find the exact LCA index."
    }
}

impl LcaFinder {
    pub fn solve(nodes: &[(usize, usize, i32)], root: usize, p: usize, q: usize) -> usize {
        let null = usize::MAX;
        
        // Generate path helper iterating DFS
        let get_path = |target: usize| -> Vec<usize> {
            let mut stack = vec![(root, vec![root])];
            while let Some((node_idx, path)) = stack.pop() {
                if node_idx == target { return path; }
                let left = nodes[node_idx].0;
                let right = nodes[node_idx].1;

                if right != null {
                    let mut p = path.clone();
                    p.push(right);
                    stack.push((right, p));
                }
                if left != null {
                    let mut p = path.clone();
                    p.push(left);
                    stack.push((left, p));
                }
            }
            vec![]
        };

        AgentLogger::log(AgentFeedback::Info, format!("Searching for LCA iteractively for nodes {p} and {q}."));

        let path_p = get_path(p);
        let path_q = get_path(q);

        let mut lca = root;
        for (i, &n) in path_p.iter().enumerate() {
            if i < path_q.len() && path_q[i] == n {
                lca = n;
            } else {
                break;
            }
        }

        AgentLogger::log(AgentFeedback::Success, format!("LCA resolved to index {lca} (value={}).", nodes[lca].2));
        lca
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "lca_finder", description = "Use this for solving lca finder problems. Trigger Keywords: lca_finder, lca finder, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

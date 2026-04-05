use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Binary Search Tree Validator
/// CATEGORY: trees-binary
/// DESCRIPTION: Validates if a binary tree is a valid BST using Morris Inorder Traversal
///              to check strictly increasing bounds in O(1) space.
pub struct BstValidator;

impl Complexity for BstValidator {
    fn name(&self) -> &'static str {
        "BST Validator (Morris Inorder / O(1) Space)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Traverses every node using threaded links."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — No recursion stack. Uses Morris temporary threads."
    }

    fn description(&self) -> &'static str {
        "Executes a Morris Inorder traversal. If any visited node is less than or equal to the previously visited node, the BST property is violated."
    }
}

impl BstValidator {
    /// Returns true if the tree is a valid Binary Search Tree.
    pub fn solve(nodes: &mut [(usize, usize, i32)], root: usize) -> bool {
        let null = usize::MAX;
        let mut curr = root;
        let mut prev_val: Option<i32> = None;
        let mut is_bst = true;

        AgentLogger::log(AgentFeedback::Info, "Validating BST in O(1) space via Morris Traversal.");

        while curr != null {
            let left = nodes[curr].0;
            if left == null {
                if let Some(p) = prev_val {
                    if nodes[curr].2 <= p { is_bst = false; }
                }
                prev_val = Some(nodes[curr].2);
                curr = nodes[curr].1;
            } else {
                let mut pred = left;
                while nodes[pred].1 != null && nodes[pred].1 != curr {
                    pred = nodes[pred].1;
                }

                if nodes[pred].1 == null {
                    nodes[pred].1 = curr;
                    curr = left;
                } else {
                    nodes[pred].1 = null;
                    if let Some(p) = prev_val {
                        if nodes[curr].2 <= p { is_bst = false; }
                    }
                    prev_val = Some(nodes[curr].2);
                    curr = nodes[curr].1;
                }
            }
        }

        if is_bst {
            AgentLogger::log(AgentFeedback::Success, "Tree is a valid BST.");
        } else {
            AgentLogger::log(AgentFeedback::Warning, "Tree is NOT a valid BST (ordering violated).");
        }

        is_bst
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "bst_validator", description = "Use this for solving bst validator problems. Trigger Keywords: bst_validator, bst validator, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

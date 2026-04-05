use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Binary Tree Traversals
/// CATEGORY: trees-binary
/// DESCRIPTION: Performs Inorder, Preorder, and Postorder traversals.
///              Uses Morris Traversal to achieve strictly O(1) space.
pub struct Traversals;

impl Complexity for Traversals {
    fn name(&self) -> &'static str {
        "Tree Traversals (Morris Traversal O(1) Space)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Every edge is traversed at most 3 times."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Temporarily modifies right pointers of leaves instead of using a recursion stack."
    }

    fn description(&self) -> &'static str {
        "Morris Traversal creates a temporary link from a node's in-order predecessor to the node itself, allowing traversal without a stack. The tree is restored to its original state upon completion."
    }
}

impl Traversals {
    /// Inorder traversal (Left, Root, Right) in O(1) space.
    /// `nodes[i] = (left_idx, right_idx, value)`. `usize::MAX` = null.
    pub fn inorder(nodes: &mut [(usize, usize, i32)], root: usize) -> Vec<i32> {
        let null = usize::MAX;
        let mut curr = root;
        let mut ans = Vec::new();

        AgentLogger::log(
            AgentFeedback::Info,
            "Executing Morris Inorder Traversal in O(1) space.",
        );

        while curr != null {
            let left = nodes[curr].0;
            if left == null {
                ans.push(nodes[curr].2);
                curr = nodes[curr].1;
            } else {
                let mut predecessor = left;
                while nodes[predecessor].1 != null && nodes[predecessor].1 != curr {
                    predecessor = nodes[predecessor].1;
                }

                if nodes[predecessor].1 == null {
                    nodes[predecessor].1 = curr; // Make temporary thread
                    curr = left;
                } else {
                    nodes[predecessor].1 = null; // Revert temporary thread
                    ans.push(nodes[curr].2);
                    curr = nodes[curr].1;
                }
            }
        }

        AgentLogger::log(AgentFeedback::Success, format!("Inorder traversal collected {} nodes.", ans.len()));
        ans
    }

    /// Preorder traversal (Root, Left, Right) in O(1) space.
    pub fn preorder(nodes: &mut [(usize, usize, i32)], root: usize) -> Vec<i32> {
        let null = usize::MAX;
        let mut curr = root;
        let mut ans = Vec::new();

        while curr != null {
            let left = nodes[curr].0;
            if left == null {
                ans.push(nodes[curr].2);
                curr = nodes[curr].1;
            } else {
                let mut predecessor = left;
                while nodes[predecessor].1 != null && nodes[predecessor].1 != curr {
                    predecessor = nodes[predecessor].1;
                }

                if nodes[predecessor].1 == null {
                    ans.push(nodes[curr].2); // Process root before diving
                    nodes[predecessor].1 = curr; 
                    curr = left;
                } else {
                    nodes[predecessor].1 = null; 
                    curr = nodes[curr].1;
                }
            }
        }
        ans
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "traversals", description = "Use this for solving traversals problems. Trigger Keywords: traversals, traversals, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

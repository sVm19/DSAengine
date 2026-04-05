use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Construct Tree from Traversals
/// CATEGORY: trees-binary
/// DESCRIPTION: Rebuilds a binary tree from its Preorder and Inorder arrays
///              using an iterative stack-driven assembly approach.
pub struct ConstructFromTraversal;

impl Complexity for ConstructFromTraversal {
    fn name(&self) -> &'static str {
        "Construct Tree (Iterative Preorder/Inorder Sync)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Every node from the arrays is processed exactly once to form the structural mapping."
    }

    fn space_complexity(&self) -> &'static str {
        "O(H) — Uses a backing stack bounded by tree height to deduce ancestry lineages."
    }

    fn description(&self) -> &'static str {
        "Preorder gives the root-first sequence; Inorder dictates left/right boundaries. An explicit stack binds newly constructed nodes downwards optimally. Avoids massive array cloning associated with recursive reconstruction."
    }
}

impl ConstructFromTraversal {
    /// Iteratively reconstructs a tree into `nodes` and returns the root index.
    pub fn solve(nodes: &mut Vec<(usize, usize, i32)>, preorder: &[i32], inorder: &[i32]) -> usize {
        let null = usize::MAX;
        if preorder.is_empty() || inorder.is_empty() { return null; }

        nodes.push((null, null, preorder[0]));
        let root_idx = 0;

        let mut stack = Vec::new();
        stack.push(root_idx);

        let mut inorder_ptr = 0;

        AgentLogger::log(AgentFeedback::Info, format!("Reconstructing tree iteratively from {} preorder elements.", preorder.len()));

        for i in 1..preorder.len() {
            let node_val = preorder[i];
            
            nodes.push((null, null, node_val));
            let curr_idx = nodes.len() - 1;

            let mut prev_idx = null;

            while let Some(&top) = stack.last() {
                if nodes[top].2 == inorder[inorder_ptr] {
                    prev_idx = stack.pop().unwrap();
                    inorder_ptr += 1;
                } else {
                    break;
                }
            }

            if prev_idx != null {
                // We've popped matching inorder elements. The right child of the last popped node.
                nodes[prev_idx].1 = curr_idx;
                AgentLogger::log(AgentFeedback::Step, format!("Bound RIGHT child {node_val} to parent {}.", nodes[prev_idx].2));
            } else {
                // Inorder not matched yet, so it's the left child of the stack's top
                let top_idx = *stack.last().unwrap();
                nodes[top_idx].0 = curr_idx;
                AgentLogger::log(AgentFeedback::Step, format!("Bound LEFT child {node_val} to parent {}.", nodes[top_idx].2));
            }

            stack.push(curr_idx);
        }

        AgentLogger::log(AgentFeedback::Success, "Tree reconstruction fully completed.");
        root_idx
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "construct_from_traversal", description = "Use this for solving construct from traversal problems. Trigger Keywords: construct_from_traversal, construct from traversal, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

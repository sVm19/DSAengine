use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Flatten Multilevel Doubly Linked List
/// CATEGORY: linked-lists
/// DESCRIPTION: Flattens a multilevel doubly linked list where nodes can have a child
///              pointer, creating a single-level doubly linked list using an iterative DFS stack.
///
/// Arena layout: `nodes[i] = (prev_idx, next_idx, child_idx, value)`. `usize::MAX` = null.
pub struct FlattenMultilevel;

impl Complexity for FlattenMultilevel {
    fn name(&self) -> &'static str {
        "Flatten Multilevel List (Iterative DFS Stack)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) — Each node is visited and pushed/popped from the stack exactly once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(d) — Where d is the maximum depth of the child levels (size of explicit stack)."
    }

    fn description(&self) -> &'static str {
        "Uses an explicit stack to remember the `next` node when diving into a `child` pointer, appending the child to the main path and rewiring prev/next pointers as it returns."
    }
}

impl FlattenMultilevel {
    /// Flattens the list starting at `head`.
    /// 
    /// `nodes[i]` = `(prev, next, child, value)`. Mutates `next` and `prev` to flatten the list.
    /// Returns the head index.
    pub fn solve(nodes: &mut Vec<(usize, usize, usize, i32)>, head: usize) -> usize {
        let null = usize::MAX;
        if head == null {
            return null;
        }

        let mut stack = Vec::new();
        let mut prev = null;
        let mut step = 0usize;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Flattening multilevel list starting at head={head}."),
        );

        stack.push(head);

        while let Some(node_idx) = stack.pop() {
            if prev != null {
                // Link prev <-> curr
                nodes[prev].1 = node_idx;
                nodes[node_idx].0 = prev;
            }

            let next = nodes[node_idx].1;
            let child = nodes[node_idx].2;

            // Push next first so child is popped first (DFS)
            if next != null {
                stack.push(next);
            }

            if child != null {
                stack.push(child);
                nodes[node_idx].2 = null; // Clear the child pointer
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Step {step}: Divings into child list at child={child}."),
                );
            }

            prev = node_idx;
            step += 1;
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Flattens complete: processed {step} node(s)."),
        );

        head
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "flatten_multilevel", description = "Use this for solving flatten multilevel problems. Trigger Keywords: flatten_multilevel, flatten multilevel, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

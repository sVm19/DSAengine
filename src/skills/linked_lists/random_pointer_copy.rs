use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Copy List with Random Pointer
/// CATEGORY: linked-lists
/// DESCRIPTION: Deep copies a linked list where every node contains an additional
///              random pointer, which could point to any node in the list or null.
///
/// Arena layout: `nodes[i] = (next_idx, random_idx, value)`. `usize::MAX` = null.
pub struct RandomPointerCopy;

impl Complexity for RandomPointerCopy {
    fn name(&self) -> &'static str {
        "Random Pointer Copy (3-Pass Interleaving)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) — Three linear passes over the list: interleave, map random pointers, unweave."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — No extra hash maps used; modifies existing list structure temporarily to track mapping."
    }

    fn description(&self) -> &'static str {
        "Interleaves cloned nodes directly after original nodes. Pass 2 assigns random pointers to the cloned nodes since `curr.next.random = curr.random.next`. Pass 3 splits them back into two distinct lists."
    }
}

impl RandomPointerCopy {
    /// Deep copies the list starting at `head`.
    /// 
    /// `nodes[i]` = `(next_idx, random_idx, value)`.
    /// Returns the head index of the newly cloned list within the arena.
    pub fn solve(nodes: &mut Vec<(usize, usize, i32)>, head: usize) -> usize {
        let null = usize::MAX;
        if head == null {
            return null;
        }

        let n = nodes.len();
        AgentLogger::log(
            AgentFeedback::Info,
            format!("Copying linked list with random pointers starting at head={head}."),
        );

        // Pass 1: Interleave cloned nodes. A -> A' -> B -> B'
        let mut curr = head;
        while curr != null {
            let next = nodes[curr].0;
            let val = nodes[curr].2;
            
            nodes.push((next, null, val));
            let clone_idx = nodes.len() - 1;
            
            nodes[curr].0 = clone_idx;
            curr = next;
        }

        // Pass 2: Assign random pointers for cloned nodes
        let mut curr = head;
        while curr != null {
            let clone_idx = nodes[curr].0;
            let rand_idx = nodes[curr].1;
            
            if rand_idx != null {
                nodes[clone_idx].1 = nodes[rand_idx].0; // clone's random points to original random's clone
            }
            
            curr = nodes[clone_idx].0; // Jump to next original node
        }

        // Pass 3: Unweave lists
        let mut curr = head;
        let cloned_head = nodes[head].0;
        
        while curr != null {
            let clone_idx = nodes[curr].0;
            let next_orig = nodes[clone_idx].0;
            
            nodes[curr].0 = next_orig;
            if next_orig != null {
                nodes[clone_idx].0 = nodes[next_orig].0;
            }
            
            curr = next_orig;
        }

        let new_nodes_count = nodes.len() - n;
        AgentLogger::log(
            AgentFeedback::Success,
            format!("Deep copy completed: {new_nodes_count} new node(s) created; cloned_head={cloned_head}."),
        );

        cloned_head
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "random_pointer_copy", description = "Use this for solving random pointer copy problems. Trigger Keywords: random_pointer_copy, random pointer copy, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

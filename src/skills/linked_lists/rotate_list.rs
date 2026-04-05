use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Rotate List
/// CATEGORY: linked-lists
/// DESCRIPTION: Rotates a linked list to the right by k places.
///
/// Arena layout: `nodes[i] = (next_idx, value)`. `usize::MAX` = null.
pub struct RotateList;

impl Complexity for RotateList {
    fn name(&self) -> &'static str {
        "Rotate List (Ring Length Modulo)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) — One pass to find the length and attach tail to head to form a ring, then another partial pass to find the new tail."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Only index variables used."
    }

    fn description(&self) -> &'static str {
        "Finds the list length n and links the tail to the head. Calculate the effective rotation offset k % n. Steps forward n - (k % n) from the head to find the new tail, breaks the loop to form the new list."
    }
}

impl RotateList {
    /// Rotates the list shifting it `k` times right.
    /// 
    /// Returns the new head index.
    pub fn solve(nodes: &mut Vec<(usize, i32)>, head: usize, k: usize) -> usize {
        let null = usize::MAX;
        
        if head == null || nodes[head].0 == null || k == 0 {
            return head;
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Rotating list starting at head={head} by k={k} places."),
        );

        // Find length and tail node
        let mut tail = head;
        let mut len = 1;
        while nodes[tail].0 != null {
            tail = nodes[tail].0;
            len += 1;
        }

        let offset = k % len;
        if offset == 0 {
            AgentLogger::log(
                AgentFeedback::Success,
                format!("k is a multiple of length ({len}); list remains unchanged."),
            );
            return head;
        }

        AgentLogger::log(
            AgentFeedback::Step,
            format!("List length={len}; linking tail {tail} to head {head} forming a ring. Moving {} steps to new tail.", len - offset),
        );

        // Make it circular
        nodes[tail].0 = head;

        // Find new tail: (len - offset) steps from head, meaning (len - offset - 1) advances
        let mut new_tail = head;
        for _ in 0..(len - offset - 1) {
            new_tail = nodes[new_tail].0;
        }

        let new_head = nodes[new_tail].0;
        nodes[new_tail].0 = null; // Break the ring

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Rotation complete; new head={new_head}, new tail={new_tail}."),
        );

        new_head
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "rotate_list", description = "Use this for solving rotate list problems. Trigger Keywords: rotate_list, rotate list, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

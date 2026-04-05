use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Intersection of Two Linked Lists
/// CATEGORY: linked-lists
/// DESCRIPTION: Finds the node at which two singly linked lists intersect.
///              If they do not intersect, returns null.
///
/// Arena layout: `nodes[i] = (next_index, value)`. `usize::MAX` = null.
pub struct IntersectionFinder;

impl Complexity for IntersectionFinder {
    fn name(&self) -> &'static str {
        "Intersection Finder (Two-Pointer Alignment)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(m + n) — In the worst case, each pointer traverses both lists entirely."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Read-only traversal requires exactly two index cursors."
    }

    fn description(&self) -> &'static str {
        "Two pointers traverse their respective lists. When one reaches the end, it jumps to the head of the other list. This aligns them by distance from the intersection point."
    }
}

impl IntersectionFinder {
    /// Returns the intersecting node's index, or `usize::MAX` if no intersection exists.
    ///
    /// `nodes[i]` = `(next_idx, value)`.
    pub fn solve(nodes: &[(usize, i32)], head_a: usize, head_b: usize) -> usize {
        let null = usize::MAX;
        
        if head_a == null || head_b == null {
            return null;
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Finding intersection between head_a={head_a} and head_b={head_b}."),
        );

        let mut a = head_a;
        let mut b = head_b;
        let mut switches = 0;

        while a != b {
            if a != null {
                a = nodes[a].0;
            } else {
                a = head_b;
                switches += 1;
            }

            if b != null {
                b = nodes[b].0;
            } else {
                b = head_a;
                switches += 1;
            }
        }

        if a == null {
            AgentLogger::log(
                AgentFeedback::Warning,
                format!("No intersection found after {switches} pointer switch(es)."),
            );
        } else {
            AgentLogger::log(
                AgentFeedback::Success,
                format!("Intersection found at node {a}."),
            );
        }

        a
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "intersection_finder", description = "Use this for solving intersection finder problems. Trigger Keywords: intersection_finder, intersection finder, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

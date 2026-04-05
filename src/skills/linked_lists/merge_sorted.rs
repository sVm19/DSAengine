use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Merge Sorted Lists
/// CATEGORY: linked-lists
/// DESCRIPTION: Merges two (or k) sorted linked lists into one sorted list
///              using an iterative pointer-chase — no extra allocation.
///
/// Arena layout: `nodes[i] = (next_idx, value)`. `usize::MAX` = null.
pub struct MergeSorted;

impl Complexity for MergeSorted {
    fn name(&self) -> &'static str {
        "Merge Sorted Lists (Iterative Dummy-Head)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(m + n) for two lists; O(N log k) for k lists via min-heap — N = total nodes."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) for two-list merge — rewires existing next pointers, no new nodes. O(k) heap for k-way merge."
    }

    fn description(&self) -> &'static str {
        "Uses a dummy head node to avoid head-insertion edge cases; at each step appends the smaller head to the merged tail and advances that list's pointer."
    }
}

impl MergeSorted {
    /// Merges two sorted lists (by value) into one sorted list.
    /// Returns the new head index in the shared arena.
    pub fn merge_two(
        nodes: &mut Vec<(usize, i32)>,
        head_a: usize,
        head_b: usize,
    ) -> usize {
        let null = usize::MAX;

        // Dummy sentinel: index = nodes.len()
        nodes.push((null, i32::MIN));
        let dummy = nodes.len() - 1;
        let mut tail = dummy;

        let mut a = head_a;
        let mut b = head_b;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Merging two sorted lists: head_a={head_a}, head_b={head_b}."),
        );

        while a != null && b != null {
            if nodes[a].1 <= nodes[b].1 {
                nodes[tail].0 = a;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Appended node {a} (val={}) from list A.", nodes[a].1),
                );
                tail = a;
                a = nodes[a].0;
            } else {
                nodes[tail].0 = b;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Appended node {b} (val={}) from list B.", nodes[b].1),
                );
                tail = b;
                b = nodes[b].0;
            }
        }

        // Attach remaining tail.
        nodes[tail].0 = if a != null { a } else { b };

        let new_head = nodes[dummy].0;
        nodes.pop(); // Remove dummy.
        AgentLogger::log(
            AgentFeedback::Success,
            format!("Merge complete; new head={new_head}."),
        );
        new_head
    }

    /// Merges k sorted lists using a min-heap (index, value) approach.
    /// `heads` — start indices of each list in the shared `nodes` arena.
    pub fn merge_k(nodes: &mut Vec<(usize, i32)>, heads: &[usize]) -> usize {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let null = usize::MAX;
        // Heap entries: (Reverse(value), node_index) for min-heap behaviour.
        let mut heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();

        for &h in heads {
            if h != null {
                heap.push((Reverse(nodes[h].1), h));
            }
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!("k-way merge: {} list(s), {} initial heap entries.", heads.len(), heap.len()),
        );

        // Dummy sentinel.
        nodes.push((null, i32::MIN));
        let dummy = nodes.len() - 1;
        let mut tail = dummy;

        while let Some((Reverse(val), idx)) = heap.pop() {
            nodes[tail].0 = idx;
            tail = idx;

            AgentLogger::log(
                AgentFeedback::Step,
                format!("Popped node {idx} (val={val}) from heap; appended to merged list."),
            );

            let next = nodes[idx].0;
            if next != null {
                heap.push((Reverse(nodes[next].1), next));
            }
        }

        nodes[tail].0 = null;
        let new_head = nodes[dummy].0;
        nodes.pop();

        AgentLogger::log(
            AgentFeedback::Success,
            format!("k-way merge complete; new head={new_head}."),
        );
        new_head
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "merge_sorted", description = "Use this for solving merge sorted problems. Trigger Keywords: merge_sorted, merge sorted, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

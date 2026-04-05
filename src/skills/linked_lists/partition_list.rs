use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Partition List
/// CATEGORY: linked-lists
/// DESCRIPTION: Partitions a linked list such that all nodes with values less than x
///              come before nodes with values greater than or equal to x.
///
/// Arena layout: `nodes[i] = (next_index, value)`. `usize::MAX` = null.
pub struct PartitionList;

impl Complexity for PartitionList {
    fn name(&self) -> &'static str {
        "Partition List (Two Dummy Heads)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) — Single pass over the original list."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Requires in-place rewiring using two dummy heads."
    }

    fn description(&self) -> &'static str {
        "Maintains two separate lists (less than x, and greater/equal to x) using dummy heads, appending nodes as it scans, then joins the 'less' tail to the 'greater' head."
    }
}

impl PartitionList {
    /// Partitions the list around `x`.
    /// 
    /// returns the new head index.
    pub fn solve(nodes: &mut Vec<(usize, i32)>, head: usize, x: i32) -> usize {
        let null = usize::MAX;

        // Dummy nodes
        nodes.push((null, 0));
        let less_head = nodes.len() - 1;
        nodes.push((null, 0));
        let greater_head = nodes.len() - 1;

        let mut less_tail = less_head;
        let mut greater_tail = greater_head;
        let mut curr = head;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Partitioning list starting at head={head} around x={x}."),
        );

        let mut step = 0;
        while curr != null {
            let next = nodes[curr].0;
            let val = nodes[curr].1;

            if val < x {
                nodes[less_tail].0 = curr;
                less_tail = curr;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Node {curr} (val={val}) appended to LESS list."),
                );
            } else {
                nodes[greater_tail].0 = curr;
                greater_tail = curr;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Node {curr} (val={val}) appended to GREATER list."),
                );
            }

            curr = next;
            step += 1;
        }

        // Join the lists
        nodes[less_tail].0 = nodes[greater_head].0;
        nodes[greater_tail].0 = null;

        let new_head = nodes[less_head].0;

        // Cleanup dummy nodes if needed (to keep arena clean), but typically they remain at the end.
        // For strictness, if arena index stability is assumed, popping them is tricky if they were referenced.
        // Since they were added at the end, and we only referenced their struct fields, we could theoretically pop them 
        // if no one holds their direct indices, but `nodes[less_tail].0 = nodes[greater_head].0` resolves it. 
        // Best practice to leave them or swap-remove carefuly. Here we just leave them.

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Partition complete in {step} steps; new head={new_head}."),
        );

        new_head
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "partition_list", description = "Use this for solving partition list problems. Trigger Keywords: partition_list, partition list, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

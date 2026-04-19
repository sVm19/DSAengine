use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Reorder List
/// CATEGORY: linked-lists
/// DESCRIPTION: Reorders a singly linked list from L0 -> L1 -> ... -> Ln to
///              L0 -> Ln -> L1 -> Ln-1 -> L2 -> Ln-2 ...
///
/// Arena layout: `nodes[i] = (next_idx, value)`. `usize::MAX` = null.
pub struct ReorderList;

impl Complexity for ReorderList {
    fn name(&self) -> &'static str {
        "Reorder List (Find Mid + Reverse + Merge)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) — Traverses the list to find the middle, reverses the second half, and merges."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — All operations are done in-place via index pointer manipulation."
    }

    fn description(&self) -> &'static str {
        "Combines three skills: 1) Fast/Slow pointer to find the middle, 2) Iterative list reversal for the second half, 3) Alternating merge of the two halves."
    }
}

impl ReorderList {
    /// Reorders the list starting at `head`.
    ///
    /// Modifies the `nodes` arena in place. Returns the new head index (same as `head`).
    pub fn solve(nodes: &mut Vec<(usize, i32)>, head: usize) -> usize {
        let null = usize::MAX;
        if head == null || nodes[head].0 == null {
            return head;
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Reordering list starting at head={head}."),
        );

        // Step 1: Find middle
        let mut slow = head;
        let mut fast = head;
        while fast != null && nodes[fast].0 != null {
            slow = nodes[slow].0;
            let next_fast = nodes[fast].0;
            fast = nodes[next_fast].0;
        }

        let mut l2 = nodes[slow].0;
        nodes[slow].0 = null; // Split the lists

        AgentLogger::log(
            AgentFeedback::Step,
            format!("Split list at middle node {slow}; second half starts at {l2}."),
        );

        // Step 2: Reverse second half
        let mut prev = null;
        let mut curr = l2;
        while curr != null {
            let next = nodes[curr].0;
            nodes[curr].0 = prev;
            prev = curr;
            curr = next;
        }
        l2 = prev; // Head of reversed second half

        AgentLogger::log(
            AgentFeedback::Step,
            format!("Reversed second half; new second half head is {l2}."),
        );

        // Step 3: Merge alternately
        let mut l1 = head;
        let mut step = 0;
        while l1 != null && l2 != null {
            let next1 = nodes[l1].0;
            let next2 = nodes[l2].0;

            nodes[l1].0 = l2;
            nodes[l2].0 = next1;

            l1 = next1;
            l2 = next2;
            step += 1;
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Merge complete in {step} alternating steps."),
        );

        head
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "reorder_list",
    description = "Use this for solving reorder list problems. Trigger Keywords: reorder_list, reorder list, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_reorder_list(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_reorder_list(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        nodes: Vec<(usize, i32)>,
        head: usize,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'nodes' as [(next,val)] and 'head'.".to_string(),
    })?;

    let result = {
        let mut nc = req.nodes.clone();
        let h = ReorderList::solve(&mut nc, req.head);
        json!({ "head": h })
    };

    let solver = ReorderList;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["reorder"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("ReorderList completed."))
}

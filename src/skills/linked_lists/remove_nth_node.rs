use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Remove N-th Node From End of List
/// CATEGORY: linked-lists
/// DESCRIPTION: Removes the n-th node from the end of the list using a one-pass
///              two-pointer approach with a dummy head to handle edge cases.
///
/// Arena layout: `nodes[i] = (next_idx, value)`. `usize::MAX` = null.
pub struct RemoveNthNode;

impl Complexity for RemoveNthNode {
    fn name(&self) -> &'static str {
        "Remove N-th Node From End (Fast/Slow Pointers)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(L) — A single pass through the list where L is the number of nodes."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Only fast and slow index pointers are used."
    }

    fn description(&self) -> &'static str {
        "Uses a dummy head to safely remove the head if needed. A 'fast' pointer advances N steps first. Then both pointers advance together so 'slow' stops right before the node to remove."
    }
}

impl RemoveNthNode {
    /// Removes the `n`-th node from the end.
    /// Returns the new head index.
    pub fn solve(nodes: &mut Vec<(usize, i32)>, head: usize, n: usize) -> usize {
        let null = usize::MAX;

        // Dummy head setup
        nodes.push((head, 0));
        let dummy = nodes.len() - 1;

        let mut fast = dummy;
        let mut slow = dummy;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Removing {n}-th node from end for list starting at head={head}."),
        );

        // Advance fast by n steps
        for _ in 0..=n {
            if fast == null {
                AgentLogger::log(
                    AgentFeedback::Warning,
                    format!("n={n} is larger than the list length. Doing nothing."),
                );
                nodes.pop(); // cleanup dummy
                return head;
            }
            fast = nodes[fast].0;
        }

        // Move both until fast reaches the end
        while fast != null {
            fast = nodes[fast].0;
            slow = nodes[slow].0;
        }

        // Slow is now pointing to the node right before the one to be removed.
        let to_remove = nodes[slow].0;
        if to_remove != null {
            let next_node = nodes[to_remove].0;
            nodes[slow].0 = next_node;
            AgentLogger::log(
                AgentFeedback::Success,
                format!(
                    "Removed node {to_remove} (value={}); linked {slow} -> {}.",
                    nodes[to_remove].1,
                    if next_node == null {
                        "NULL".to_string()
                    } else {
                        next_node.to_string()
                    }
                ),
            );
        }

        let new_head = nodes[dummy].0;
        // Optional: keep dummy or pop if we strictly manage arena size. We'll leave it as typical.
        new_head
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "remove_nth_node",
    description = "Use this for solving remove nth node problems. Trigger Keywords: remove_nth_node, remove nth node, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_remove_nth_node(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_remove_nth_node(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        nodes: Vec<(usize, i32)>,
        head: usize,
        n: usize,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'nodes' as [(next,val)], 'head', 'n'.".to_string(),
    })?;

    let result = {
        let mut nc = req.nodes.clone();
        let h = RemoveNthNode::solve(&mut nc, req.head, req.n);
        json!({ "head": h, "removed_nth": req.n })
    };

    let solver = RemoveNthNode;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["remove"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("RemoveNthNode completed."))
}

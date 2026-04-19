use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::HashSet;

/// SKILL: Linked List Visualizer
/// CATEGORY: linked-lists
/// DESCRIPTION: Outputs string representations of index-based linked lists,
///              detecting loops to avoid infinite traversals.
pub struct Visualizer;

impl Complexity for Visualizer {
    fn name(&self) -> &'static str {
        "Linked List Visualizer (Cycle Aware)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Visits each node, halting if it detects a back-edge to an already visited node."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Visited HashSet and the output string."
    }

    fn description(&self) -> &'static str {
        "Traverses the arena list using a HashSet to detect cycles. Formats as 'val (idx) -> val (idx) -> ...', safely cutting off upon loop detection."
    }
}

impl Visualizer {
    /// Formats the single linked list starting at `head`.
    ///
    /// `nodes[i] = (next_idx, value)`
    pub fn to_string(nodes: &[(usize, i32)], head: usize) -> String {
        use std::collections::HashSet;

        let null = usize::MAX;
        let mut curr = head;
        let mut visited = HashSet::new();
        let mut out = String::from("List: ");

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Visualizing list starting from head={head}."),
        );

        while curr != null {
            if !visited.insert(curr) {
                out.push_str(&format!("[CYCLE to idx {}]", curr));
                break;
            }

            out.push_str(&format!("{} (idx:{}) -> ", nodes[curr].1, curr));
            curr = nodes[curr].0;
        }

        if curr == null {
            out.push_str("NULL");
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Visualization complete: {out}"),
        );

        out
    }

    /// Formats a list with random pointers.
    /// `nodes[i] = (next_idx, rand_idx, value)`
    pub fn to_string_random(nodes: &[(usize, usize, i32)], head: usize) -> String {
        let null = usize::MAX;
        let mut curr = head;
        let mut visited = HashSet::new();
        let mut out = String::from("List Random: ");

        while curr != null {
            if !visited.insert(curr) {
                out.push_str(&format!("[CYCLE to idx {}]", curr));
                break;
            }

            let rnd_str = if nodes[curr].1 == null {
                "NULL".to_string()
            } else {
                format!("idx:{}", nodes[curr].1)
            };
            out.push_str(&format!(
                "{} (idx:{}, rnd:{}) -> ",
                nodes[curr].2, curr, rnd_str
            ));
            curr = nodes[curr].0;
        }

        if curr == null {
            out.push_str("NULL");
        }

        out
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "visualizer",
    description = "Use this for solving visualizer problems. Trigger Keywords: visualizer, visualizer, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_linked_list_visualizer(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_linked_list_visualizer(payload: Value) -> DsaResult<ResultBox> {
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
        let viz = Visualizer::to_string(&req.nodes, req.head);
        json!({ "visualization": viz })
    };

    let solver = Visualizer;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["visualize"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Visualizer completed."))
}

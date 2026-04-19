use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Cycle Detection
/// CATEGORY: linked-lists
/// DESCRIPTION: Detects whether a linked list (index-arena) contains a cycle using
///              Floyd's two-pointer algorithm, and locates the cycle entry node.
///
/// Arena layout: `nodes[i] = (next_index, value)`. `usize::MAX` = null.
pub struct CycleDetection;

impl Complexity for CycleDetection {
    fn name(&self) -> &'static str {
        "Cycle Detection (Floyd's Tortoise & Hare)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) — Hare laps tortoise within at most 2n steps; cycle-entry phase adds ≤ n more steps."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Only two index cursors; the arena is traversed read-only."
    }

    fn description(&self) -> &'static str {
        "Slow pointer advances one node; fast pointer advances two. If they meet, a cycle exists. The entry node is found by resetting slow to head and advancing both one step at a time until they meet."
    }
}

impl CycleDetection {
    /// Returns `true` if the list starting at `head` contains a cycle.
    pub fn has_cycle(nodes: &[(usize, usize)], head: usize) -> bool {
        // nodes[i] = (next_idx, value); usize::MAX = null
        let null = usize::MAX;
        let mut slow = head;
        let mut fast = head;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Cycle detection: Floyd's algorithm from head={head}."),
        );

        loop {
            // Advance fast two steps.
            if fast == null {
                return false;
            }
            fast = nodes[fast].0;
            if fast == null {
                return false;
            }
            fast = nodes[fast].0;

            // Advance slow one step.
            slow = nodes[slow].0;

            AgentLogger::log(
                AgentFeedback::Step,
                format!("Tortoise={slow}, Hare={fast}."),
            );

            if slow == fast {
                AgentLogger::log(
                    AgentFeedback::Warning,
                    format!("Cycle detected: meeting point={slow}."),
                );
                return true;
            }
        }
    }

    /// Returns the index of the cycle entry node, or `None` if there is no cycle.
    pub fn find_entry(nodes: &[(usize, usize)], head: usize) -> Option<usize> {
        let null = usize::MAX;
        let mut slow = head;
        let mut fast = head;

        // Phase 1: detect meeting point.
        let meeting = loop {
            if fast == null {
                return None;
            }
            fast = nodes[fast].0;
            if fast == null {
                return None;
            }
            fast = nodes[fast].0;
            slow = nodes[slow].0;

            if slow == fast {
                break slow;
            }
        };

        AgentLogger::log(
            AgentFeedback::Step,
            format!("Phase 1 meeting point: node={meeting}."),
        );

        // Phase 2: reset slow to head; advance both one step until they meet.
        let mut entry_seeker = head;
        let mut from_meet = meeting;

        while entry_seeker != from_meet {
            entry_seeker = nodes[entry_seeker].0;
            from_meet = nodes[from_meet].0;
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Cycle entry node: {entry_seeker}."),
        );
        Some(entry_seeker)
    }

    /// Returns the length of the cycle if one exists.
    pub fn cycle_length(nodes: &[(usize, usize)], head: usize) -> Option<usize> {
        let entry = Self::find_entry(nodes, head).unwrap();
        let mut cur = nodes[entry].0;
        let mut length = 1usize;

        while cur != entry {
            cur = nodes[cur].0;
            length += 1;
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Cycle length: {length} node(s)."),
        );
        Some(length)
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "cycle_detection",
    description = "Use this for solving cycle detection problems. Trigger Keywords: cycle_detection, cycle detection, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_cycle_detection(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_cycle_detection(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        nodes: Vec<(usize, usize)>,
        head: usize,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'nodes' as [(next, _)], 'head'. Mode: 'detect'|'find_entry'|'length'."
            .to_string(),
    })?;

    let result = {
        match req.mode.as_deref().unwrap_or("detect") {
            "find_entry" => {
                let e = CycleDetection::find_entry(&req.nodes, req.head);
                json!({"mode":"find_entry","entry":e})
            }
            "length" => {
                let l = CycleDetection::cycle_length(&req.nodes, req.head);
                json!({"mode":"length","cycle_length":l})
            }
            _ => {
                let has = CycleDetection::has_cycle(&req.nodes, req.head);
                json!({"mode":"detect","has_cycle":has})
            }
        }
    };

    let solver = CycleDetection;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["detect", "find_entry", "length"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("CycleDetection completed."))
}

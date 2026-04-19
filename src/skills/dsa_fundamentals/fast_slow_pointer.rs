use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Fast & Slow Pointer Pattern (Tortoise and Hare)
/// CATEGORY: dsa-fundamentals
/// DESCRIPTION: Detects cycles in Linked Lists or Arrays and finds the middle of structures.
pub struct FastSlowPointer;

impl Complexity for FastSlowPointer {
    fn name(&self) -> &'static str {
        "Fast & Slow Pointer Detector"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Linear scan of the structure."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - Only two pointer variables required."
    }

    fn description(&self) -> &'static str {
        "Used for Cycle Detection (Floyd's), Finding Middle, or Happy Number problems."
    }
}

impl FastSlowPointer {
    /// Visualizes the gap between the Hare (Fast) and Tortoise (Slow).
    pub fn trace_cycle_step(slow_idx: usize, fast_idx: usize) {
        println!("🐢 Slow: [{}] | 🐇 Fast: [{}]", slow_idx, fast_idx);

        if slow_idx == fast_idx {
            AgentLogger::log(
                AgentFeedback::Success,
                "COLLISION DETECTED: Cycle confirmed at this node.",
            );
        }
    }

    /// Explains the mathematical proof behind why they meet.
    pub fn explain_floyd_logic() {
        println!("[FLOYD'S ALGORITHM]:");
        println!("  1. If a cycle exists, the fast pointer (2x speed) will eventually 'lap' the slow pointer.");
        println!("  2. Once they meet, moving one to the start and both at 1x speed finds the cycle's entrance.");
    }

    /// Logic to find the middle of a structure.
    pub fn is_middle_reached(fast_next: bool) -> bool {
        if !fast_next {
            AgentLogger::log(
                AgentFeedback::Info,
                "Fast pointer reached end. Slow is now at the middle.",
            );
            return true;
        }
        false
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "dsa_fundamentals.fast_slow_pointer",
    description = "Use this for solving fast slow pointer problems. Trigger Keywords: fast_slow_pointer, fast slow pointer, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_fast_slow_pointer(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FastSlowPointerRequest {
    pub next: Vec<Option<usize>>,
    pub start: Option<usize>,
}

async fn handle_fast_slow_pointer(payload: Value) -> DsaResult<ResultBox> {
    let req: FastSlowPointerRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid FastSlowPointerRequest: {e}"),
            hint: "Provide 'next' as an array of next indices or null, plus optional 'start'."
                .to_string(),
        })?;

    if req.next.is_empty() {
        return Err(DsaError::InvalidInput {
            message: "next cannot be empty.".to_string(),
            hint: "Provide at least one node entry.".to_string(),
        });
    }
    for (idx, next) in req.next.iter().enumerate() {
        if let Some(next_idx) = next {
            if *next_idx >= req.next.len() {
                return Err(DsaError::IndexOutOfBounds {
                    index: *next_idx,
                    bounds: req.next.len(),
                    context: format!("next pointer at index {idx}"),
                });
            }
        }
    }

    let start = req.start.unwrap_or(0);
    if start >= req.next.len() {
        return Err(DsaError::IndexOutOfBounds {
            index: start,
            bounds: req.next.len(),
            context: "start node".to_string(),
        });
    }

    let advance = |idx: Option<usize>, next: &[Option<usize>]| idx.and_then(|i| next[i]);
    let mut slow = Some(start);
    let mut fast = Some(start);
    let mut steps = 0usize;
    let mut meeting = None;

    loop {
        slow = advance(slow, &req.next);
        fast = advance(advance(fast, &req.next), &req.next);
        steps += 1;
        if let (Some(s), Some(f)) = (slow, fast) {
            FastSlowPointer::trace_cycle_step(s, f);
            if s == f {
                meeting = Some(s);
                break;
            }
        } else {
            break;
        }
    }

    let mut entrance = None;
    if let Some(mut meet) = meeting {
        let mut probe = start;
        while probe != meet {
            probe = req.next[probe].unwrap();
            meet = req.next[meet].unwrap();
        }
        entrance = Some(probe);
    }

    let solver = FastSlowPointer;
    Ok(ResultBox::success(json!({
        "has_cycle": meeting.is_some(),
        "meeting_index": meeting,
        "cycle_entrance": entrance,
        "steps": steps
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Fast/slow pointer cycle analysis completed."))
}

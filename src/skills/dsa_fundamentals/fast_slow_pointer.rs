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
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "dsa_fundamentals.fast_slow_pointer", description = "Use this for solving fast slow pointer problems. Trigger Keywords: fast_slow_pointer, fast slow pointer, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_fast_slow_pointer(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: In-place Reversal
/// CATEGORY: dsa-fundamentals
/// DESCRIPTION: Reverses a sequence of elements using only pointer manipulation.
pub struct InPlaceReversal;

impl Complexity for InPlaceReversal {
    fn name(&self) -> &'static str {
        "In-place Reversal Pattern"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Single pass to flip pointers/elements."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - Modifies the structure in-place."
    }

    fn description(&self) -> &'static str {
        "Crucial for reversing Linked Lists, Array portions, or words in a string."
    }
}

impl InPlaceReversal {
    /// Visualizes the pointer swap in a Linked List context.
    pub fn trace_pointer_flip(prev: &str, current: &str, next: &str) {
        println!(
            "  🔗 FLIP: [Prev: {}] <- [Curr: {}] | [Next: {}]",
            prev, current, next
        );
        AgentLogger::log(
            AgentFeedback::Step,
            "Redirecting 'next' pointer to 'previous' node.",
        );
    }

    /// Helper for Array-based swapping.
    pub fn trace_array_swap(left_idx: usize, right_idx: usize) {
        println!("  🔄 SWAP: Index [{}] <==> Index [{}]", left_idx, right_idx);
    }

    /// Explains the 3-pointer technique.
    pub fn explain_three_pointer_logic() {
        println!("💡 [LOGIC]: Requires three pointers (Previous, Current, Next).");
        println!("   1. Store 'Next' to avoid losing the rest of the list.");
        println!("   2. Point 'Current.next' to 'Previous'.");
        println!("   3. Move 'Previous' and 'Current' one step forward.");
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "dsa_fundamentals.in_place_reversal", description = "Use this for solving in place reversal problems. Trigger Keywords: in_place_reversal, in place reversal, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_in_place_reversal(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

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
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "dsa_fundamentals.in_place_reversal",
    description = "Use this for solving in place reversal problems. Trigger Keywords: in_place_reversal, in place reversal, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_in_place_reversal(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct InPlaceReversalRequest {
    pub values: Vec<Value>,
}

async fn handle_in_place_reversal(payload: Value) -> DsaResult<ResultBox> {
    let req: InPlaceReversalRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid InPlaceReversalRequest: {e}"),
            hint: "Provide 'values' as an array to reverse.".to_string(),
        })?;

    let mut reversed = req.values.clone();
    if !reversed.is_empty() {
        let mut left = 0usize;
        let mut right = reversed.len() - 1;
        while left < right {
            InPlaceReversal::trace_array_swap(left, right);
            reversed.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    let solver = InPlaceReversal;
    Ok(ResultBox::success(json!({
        "original": req.values,
        "reversed": reversed
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("In-place reversal simulation completed."))
}

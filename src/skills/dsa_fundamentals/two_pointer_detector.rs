use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Two Pointer Pattern Detector
/// CATEGORY: dsa-fundamentals
/// DESCRIPTION: Identifies if a problem can be solved using the Two-Pointer technique (e.g., Sorted Arrays, Palindromes).
pub struct TwoPointerDetector;

impl Complexity for TwoPointerDetector {
    fn name(&self) -> &'static str {
        "Two Pointer Pattern Detector"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Single pass through the data."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - Constant space for pointers."
    }

    fn description(&self) -> &'static str {
        "Heuristic tool for AI Agents to detect Two-Pointer compatibility in sorted structures."
    }
}

impl TwoPointerDetector {
    /// Evaluates a problem's constraints to see if Two Pointers are applicable.
    pub fn evaluate_compatibility(is_sorted: bool, search_target: bool) -> bool {
        if is_sorted && search_target {
            AgentLogger::log(
                AgentFeedback::Success,
                "Two-Pointer Match: High probability (Sorted Search).",
            );
            return true;
        }

        AgentLogger::log(
            AgentFeedback::Info,
            "Two-Pointer Match: Low probability (Unsorted or non-search).",
        );
        false
    }

    /// Visualizes the pointer movement for the AI/User.
    pub fn trace_pointers(left: usize, right: usize, left_val: i32, right_val: i32) {
        println!(
            "  [L: {} (Val: {})] <---> [R: {} (Val: {})]",
            left, left_val, right, right_val
        );

        if left >= right {
            AgentLogger::log(
                AgentFeedback::Step,
                "Pointers crossed or met. Search complete.",
            );
        }
    }

    /// Explains why this pattern reduces complexity from O(n^2) to O(n).
    pub fn explain_efficiency() {
        println!("⚡ Optimization: Eliminates nested loops by moving pointers toward each other based on conditions.");
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "dsa_fundamentals.two_pointer_detector", description = "Use this for solving two pointer detector problems. Trigger Keywords: two_pointer_detector, two pointer detector, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_two_pointer_detector(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

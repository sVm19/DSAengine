use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Sliding Window Pattern Detector
/// CATEGORY: dsa-fundamentals
/// DESCRIPTION: Detects if a problem can be optimized using a Fixed or Variable Sliding Window.
pub struct SlidingWindowDetector;

impl Complexity for SlidingWindowDetector {
    fn name(&self) -> &'static str {
        "Sliding Window Pattern Detector"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Single pass with window expansion/contraction."
    }

    fn space_complexity(&self) -> &'static str {
        "O(k) or O(1) - Depends on auxiliary tracking (e.g., HashSets)."
    }

    fn description(&self) -> &'static str {
        "Identifies Subarray or Substring problems where a contiguous range must be tracked."
    }
}

impl SlidingWindowDetector {
    /// Checks if a problem matches the 'Contiguous Subgroup' requirement.
    pub fn matches_pattern(is_contiguous: bool, is_linear: bool) -> bool {
        if is_contiguous && is_linear {
            AgentLogger::log(
                AgentFeedback::Success,
                "Sliding Window Match: High probability (Contiguous Subarray).",
            );
            return true;
        }
        false
    }

    /// Visualizes the "Window" moving across an array.
    pub fn trace_window(start: usize, end: usize, current_sum: i32) {
        let mut visual = String::new();
        for i in 0..(end + 1) {
            if i < start {
                visual.push_str(". ");
            } else {
                visual.push_str("[#] ");
            }
        }
        println!(
            "  {} | Window: [{}, {}] | Current Sum/State: {}",
            visual, start, end, current_sum
        );
    }

    /// Explains the difference between Fixed and Dynamic windows.
    pub fn explain_types() {
        println!("🪟 [FIXED]: Window size 'k' is constant (e.g., Max sum of k elements).");
        println!("📏 [DYNAMIC]: Window size expands/shrinks based on conditions (e.g., Smallest subarray > S).");
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "dsa_fundamentals.sliding_window_detector", description = "Use this for solving sliding window detector problems. Trigger Keywords: sliding_window_detector, sliding window detector, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_sliding_window_detector(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

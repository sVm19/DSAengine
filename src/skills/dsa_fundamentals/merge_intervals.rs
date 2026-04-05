use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Merge Intervals Pattern
/// CATEGORY: dsa-fundamentals
/// DESCRIPTION: Handles overlapping ranges and interval scheduling problems.
pub struct MergeIntervals;

impl Complexity for MergeIntervals {
    fn name(&self) -> &'static str {
        "Merge Intervals Pattern Detector"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n log n) - Required for initial sorting of intervals."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) - To store the merged output list."
    }

    fn description(&self) -> &'static str {
        "Essential for Calendar apps, meeting schedulers, and range-based data analysis."
    }
}

impl MergeIntervals {
    /// Checks if two intervals [a, b] and [c, d] overlap.
    /// Condition: max(a, c) <= min(b, d)
    pub fn check_overlap(a: (i32, i32), b: (i32, i32)) -> bool {
        let overlap = a.0.max(b.0) <= a.1.min(b.1);
        if overlap {
            AgentLogger::log(
                AgentFeedback::Step,
                format!("Overlap detected between {:?} and {:?}", a, b),
            );
        }
        overlap
    }

    /// Visualizes the merging of two intervals into one.
    pub fn trace_merge(current: (i32, i32), next: (i32, i32)) -> (i32, i32) {
        let merged = (current.0.min(next.0), current.1.max(next.1));
        println!("  🔀 Merging {:?} + {:?} ===> {:?}", current, next, merged);
        merged
    }

    /// Explains the "Sort-First" requirement.
    pub fn explain_sorting_importance() {
        println!("⚠️ [CRITICAL]: Intervals must be sorted by their start time.");
        println!("   This ensures that we only need to compare the current interval with the last merged one.");
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "dsa_fundamentals.merge_intervals", description = "Use this for solving merge intervals problems. Trigger Keywords: merge_intervals, merge intervals, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_merge_intervals(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

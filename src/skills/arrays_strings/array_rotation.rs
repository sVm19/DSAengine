use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Array Rotation
/// CATEGORY: arrays-strings
/// DESCRIPTION: Rotates a slice in place using the reversal trick to avoid extra allocations.
pub struct ArrayRotation;

impl Complexity for ArrayRotation {
    fn name(&self) -> &'static str {
        "Array Rotation"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Performs three in-place reversals over the slice."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - Rotation is done entirely in place."
    }

    fn description(&self) -> &'static str {
        "Uses reverse-all, reverse-prefix, reverse-suffix to rotate without cloning the input."
    }
}

impl ArrayRotation {
    pub fn solve<T>(values: &mut [T], k: usize) {
        Self::rotate_right(values, k);
    }

    pub fn rotate_right<T>(values: &mut [T], k: usize) {
        let len = values.len();
        if len < 2 {
            return;
        }

        let shift = k % len;
        if shift == 0 {
            AgentLogger::log(
                AgentFeedback::Info,
                "Rotation distance normalized to zero; slice stays unchanged.",
            );
            return;
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Normalizing right rotation to {} positions.", shift),
        );

        values.reverse();
        AgentLogger::log(
            AgentFeedback::Step,
            "Reversed the entire slice to expose the rotated split.",
        );

        values[..shift].reverse();
        AgentLogger::log(
            AgentFeedback::Step,
            format!(
                "Reversed the new prefix [0, {}) to restore its order.",
                shift
            ),
        );

        values[shift..].reverse();
        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Reversed the suffix [{}, {}) and completed the rotation.",
                shift, len
            ),
        );
    }

    pub fn rotate_left<T>(values: &mut [T], k: usize) {
        let len = values.len();
        if len < 2 {
            return;
        }

        let shift = k % len;
        if shift == 0 {
            AgentLogger::log(
                AgentFeedback::Info,
                "Left rotation distance normalized to zero; slice stays unchanged.",
            );
            return;
        }

        Self::rotate_right(values, len - shift);
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "arrays_strings.array_rotation", description = "Use this for solving array rotation problems. Trigger Keywords: array_rotation, array rotation, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_array_rotation(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

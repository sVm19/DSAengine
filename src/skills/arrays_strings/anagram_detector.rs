use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Anagram Detector
/// CATEGORY: arrays-strings
/// DESCRIPTION: Detects whether two byte strings are anagrams by balancing a fixed-size frequency table.
pub struct AnagramDetector;

impl Complexity for AnagramDetector {
    fn name(&self) -> &'static str {
        "Anagram Detector"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n + m) - One linear frequency pass per input string."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - Uses a fixed 256-slot byte frequency table."
    }

    fn description(&self) -> &'static str {
        "Balances byte counts instead of sorting, which keeps the detector linear and zero-copy."
    }
}

impl AnagramDetector {
    pub fn solve(left: &str, right: &str) -> bool {
        Self::are_anagrams(left, right)
    }

    pub fn are_anagrams(left: &str, right: &str) -> bool {
        if left.len() != right.len() {
            AgentLogger::log(
                AgentFeedback::Warning,
                format!(
                    "Length mismatch blocks an anagram match: left={}, right={}.",
                    left.len(),
                    right.len()
                ),
            );
            return false;
        }

        let mut frequency = [0i16; 256];
        for &byte in left.as_bytes() {
            frequency[byte as usize] += 1;
        }
        AgentLogger::log(
            AgentFeedback::Info,
            "Built frequency signature for the left operand.",
        );

        for (index, &byte) in right.as_bytes().iter().enumerate() {
            let slot = &mut frequency[byte as usize];
            *slot -= 1;

            if *slot < 0 {
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Byte '{}' exceeded its available count while scanning right operand at index {}.",
                        char::from(byte),
                        index
                    ),
                );
                return false;
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            "All byte counts balanced to zero. Inputs are anagrams.",
        );
        true
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "arrays_strings.anagram_detector", description = "Use this for solving anagram detector problems. Trigger Keywords: anagram_detector, anagram detector, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_anagram_detector(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

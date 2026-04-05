use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Rabin Karp
/// CATEGORY: arrays-strings
/// DESCRIPTION: Searches for a pattern with a rolling hash and byte-for-byte collision checks.
pub struct RabinKarp;

impl Complexity for RabinKarp {
    fn name(&self) -> &'static str {
        "Rabin Karp"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n + m) average, O(nm) worst-case - Rolling hashes are linear, collisions may force verification."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - Maintains only rolling-hash state and output matches."
    }

    fn description(&self) -> &'static str {
        "Uses modular rolling hashes to slide across the text and verifies only hash matches."
    }
}

impl RabinKarp {
    const BASE: u64 = 256;
    const MODULUS: u64 = 1_000_000_007;

    pub fn solve(text: &str, pattern: &str) -> Vec<usize> {
        Self::search(text, pattern)
    }

    pub fn search(text: &str, pattern: &str) -> Vec<usize> {
        let text_bytes = text.as_bytes();
        let pattern_bytes = pattern.as_bytes();

        if pattern_bytes.is_empty() || pattern_bytes.len() > text_bytes.len() {
            return Vec::new();
        }

        let window_len = pattern_bytes.len();
        let mut high_base = 1u64;
        for _ in 1..window_len {
            high_base = (high_base * Self::BASE) % Self::MODULUS;
        }

        let mut pattern_hash = 0u64;
        let mut window_hash = 0u64;
        for index in 0..window_len {
            pattern_hash =
                (pattern_hash * Self::BASE + pattern_bytes[index] as u64) % Self::MODULUS;
            window_hash = (window_hash * Self::BASE + text_bytes[index] as u64) % Self::MODULUS;
        }

        let mut matches = Vec::new();
        for start in 0..=text_bytes.len() - window_len {
            if pattern_hash == window_hash {
                if text_bytes[start..start + window_len] == *pattern_bytes {
                    matches.push(start);
                    AgentLogger::log(
                        AgentFeedback::Success,
                        format!("Verified hash match at byte offset {}.", start),
                    );
                } else {
                    AgentLogger::log(
                        AgentFeedback::Warning,
                        format!(
                            "Discarded a rolling-hash collision at byte offset {}.",
                            start
                        ),
                    );
                }
            }

            if start + window_len < text_bytes.len() {
                let outgoing = (text_bytes[start] as u64 * high_base) % Self::MODULUS;
                let trimmed = (window_hash + Self::MODULUS - outgoing) % Self::MODULUS;
                window_hash =
                    (trimmed * Self::BASE + text_bytes[start + window_len] as u64) % Self::MODULUS;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Rolled hash window from [{}..{}) to [{}..{}).",
                        start,
                        start + window_len,
                        start + 1,
                        start + window_len + 1
                    ),
                );
            }
        }

        matches
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "arrays_strings.rabin_karp", description = "Use this for solving rabin karp problems. Trigger Keywords: rabin_karp, rabin karp, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_rabin_karp(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

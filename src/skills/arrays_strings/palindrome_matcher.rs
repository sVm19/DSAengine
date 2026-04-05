use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Palindrome Matcher
/// CATEGORY: arrays-strings
/// DESCRIPTION: Validates palindromes with a two-pointer scan that ignores punctuation and case.
pub struct PalindromeMatcher;

impl Complexity for PalindromeMatcher {
    fn name(&self) -> &'static str {
        "Palindrome Matcher"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Two pointers scan inward across the string once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - Compares bytes in place without auxiliary storage."
    }

    fn description(&self) -> &'static str {
        "Skips non-alphanumeric bytes and compares mirrored characters case-insensitively."
    }
}

impl PalindromeMatcher {
    pub fn solve(text: &str) -> bool {
        Self::is_alphanumeric_palindrome(text)
    }

    pub fn is_alphanumeric_palindrome(text: &str) -> bool {
        let bytes = text.as_bytes();
        if bytes.is_empty() {
            return true;
        }

        let mut left = 0usize;
        let mut right = bytes.len() - 1;

        while left < right {
            while left < right && !bytes[left].is_ascii_alphanumeric() {
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Skipping non-alphanumeric byte at left index {}.", left),
                );
                left += 1;
            }

            while left < right && !bytes[right].is_ascii_alphanumeric() {
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Skipping non-alphanumeric byte at right index {}.", right),
                );
                right -= 1;
            }

            let left_byte = bytes[left].to_ascii_lowercase();
            let right_byte = bytes[right].to_ascii_lowercase();
            if left_byte != right_byte {
                AgentLogger::log(
                    AgentFeedback::Warning,
                    format!(
                        "Mismatch after normalization: '{}' != '{}'.",
                        char::from(left_byte),
                        char::from(right_byte)
                    ),
                );
                return false;
            }

            left += 1;
            right -= 1;
        }

        AgentLogger::log(
            AgentFeedback::Success,
            "Two-pointer scan finished without mirrored mismatches.",
        );
        true
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "arrays_strings.palindrome_matcher", description = "Use this for solving palindrome matcher problems. Trigger Keywords: palindrome_matcher, palindrome matcher, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_palindrome_matcher(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

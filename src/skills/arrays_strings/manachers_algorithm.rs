use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Manachers Algorithm
/// CATEGORY: arrays-strings
/// DESCRIPTION: Finds the longest palindromic substring in linear time with mirrored expansion radii.
pub struct ManachersAlgorithm;

impl Complexity for ManachersAlgorithm {
    fn name(&self) -> &'static str {
        "Manachers Algorithm"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - The transformed string is scanned once with mirrored reuse."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) - Stores the transformed representation and palindrome radii."
    }

    fn description(&self) -> &'static str {
        "Uses mirrored radii around a moving center to avoid re-expanding known palindromes."
    }
}

impl ManachersAlgorithm {
    pub fn solve(text: &str) -> Option<(usize, usize)> {
        Self::longest_palindrome(text)
    }

    pub fn longest_palindrome(text: &str) -> Option<(usize, usize)> {
        if text.is_empty() {
            return None;
        }

        let transformed = Self::transform(text.as_bytes());
        let mut radius = vec![0usize; transformed.len()];
        let mut center = 0usize;
        let mut right = 0usize;
        let mut best_center = 0usize;
        let mut best_radius = 0usize;

        for i in 1..transformed.len() - 1 {
            let mirror = (center as isize * 2) - i as isize;
            if i < right && mirror >= 0 {
                radius[i] = radius[mirror as usize].min(right - i);
            }

            while transformed[i + radius[i] + 1] == transformed[i - radius[i] - 1] {
                radius[i] += 1;
            }

            if i + radius[i] > right {
                center = i;
                right = i + radius[i];
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Shifted palindrome frontier to center {} with right edge {}.",
                        center, right
                    ),
                );
            }

            if radius[i] > best_radius {
                best_radius = radius[i];
                best_center = i;
                AgentLogger::log(
                    AgentFeedback::Success,
                    format!(
                        "Updated longest palindrome to transformed center {} with radius {}.",
                        best_center, best_radius
                    ),
                );
            }
        }

        let start = (best_center - best_radius) / 2;
        Some((start, best_radius))
    }

    fn transform(bytes: &[u8]) -> Vec<i16> {
        let mut transformed = Vec::with_capacity(bytes.len() * 2 + 3);
        transformed.push(-3);
        for &byte in bytes {
            transformed.push(-2);
            transformed.push(byte as i16);
        }
        transformed.push(-2);
        transformed.push(-1);
        transformed
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "arrays_strings.manachers_algorithm", description = "Use this for solving manachers algorithm problems. Trigger Keywords: manachers_algorithm, manachers algorithm, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_manachers_algorithm(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Knuth-Morris-Pratt (KMP) Search
/// CATEGORY: arrays-strings
/// DESCRIPTION: Efficiently finds all occurrences of a pattern in a text by skipping redundant comparisons.
pub struct KMPSearch;

impl Complexity for KMPSearch {
    fn name(&self) -> &'static str {
        "Knuth-Morris-Pratt (KMP) String Matching"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n + m) - Where n is text length and m is pattern length."
    }

    fn space_complexity(&self) -> &'static str {
        "O(m) - To store the LPS (Longest Prefix Suffix) table."
    }

    fn description(&self) -> &'static str {
        "Uses pre-processing to avoid re-scanning characters in the text upon a mismatch."
    }
}

impl KMPSearch {
    pub fn solve(text: &str, pattern: &str) -> Vec<usize> {
        Self::search(text, pattern)
    }

    /// Generates the LPS (Longest Prefix Suffix) table for the pattern.
    pub fn compute_lps(pattern: &[u8]) -> Vec<usize> {
        let m = pattern.len();
        let mut lps = vec![0; m];
        let mut len = 0;
        let mut i = 1;

        while i < m {
            if pattern[i] == pattern[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else if len != 0 {
                len = lps[len - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
        lps
    }

    /// Performs the search and returns all starting indices of the pattern in text.
    pub fn search(text: &str, pattern: &str) -> Vec<usize> {
        let text_bytes = text.as_bytes();
        let pattern_bytes = pattern.as_bytes();

        if pattern_bytes.is_empty() || pattern_bytes.len() > text_bytes.len() {
            return vec![];
        }

        let lps = Self::compute_lps(pattern_bytes);

        let mut result = Vec::new();
        let mut text_index = 0;
        let mut pattern_index = 0;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Searching for '{pattern}' using the LPS fallback table."),
        );

        while text_index < text_bytes.len() {
            if text_bytes[text_index] == pattern_bytes[pattern_index] {
                text_index += 1;
                pattern_index += 1;
            }

            if pattern_index == pattern_bytes.len() {
                let start = text_index - pattern_index;
                result.push(start);
                AgentLogger::log(
                    AgentFeedback::Success,
                    format!("Pattern found at byte offset {}.", start),
                );
                pattern_index = lps[pattern_index - 1];
            } else if text_index < text_bytes.len()
                && text_bytes[text_index] != pattern_bytes[pattern_index]
            {
                if pattern_index != 0 {
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!(
                            "Mismatch at text[{}]; falling back pattern index from {} to {}.",
                            text_index,
                            pattern_index,
                            lps[pattern_index - 1]
                        ),
                    );
                    pattern_index = lps[pattern_index - 1];
                } else {
                    text_index += 1;
                }
            }
        }
        result
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "arrays_strings.kmp_search", description = "Use this for solving kmp search problems. Trigger Keywords: kmp_search, kmp search, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_kmp_search(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

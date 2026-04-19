use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Longest Substring
/// CATEGORY: arrays-strings
/// DESCRIPTION: Finds the longest byte window with all distinct characters using a sliding window.
pub struct LongestSubstring;

impl Complexity for LongestSubstring {
    fn name(&self) -> &'static str {
        "Longest Substring"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Every byte enters and exits the window at most once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - Uses a fixed 256-slot last-seen table."
    }

    fn description(&self) -> &'static str {
        "Tracks the most recent position of each byte to keep the window duplicate-free."
    }
}

impl LongestSubstring {
    pub fn solve(text: &str) -> usize {
        Self::longest_window(text).map(|(_, len)| len).unwrap_or(0)
    }

    pub fn longest_window(text: &str) -> Option<(usize, usize)> {
        if text.is_empty() {
            return None;
        }

        let bytes = text.as_bytes();
        let mut last_seen = [usize::MAX; 256];
        let mut left = 0usize;
        let mut best_start = 0usize;
        let mut best_len = 0usize;

        for (right, &byte) in bytes.iter().enumerate() {
            let previous = last_seen[byte as usize];
            if previous != usize::MAX && previous >= left {
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Duplicate byte '{}' forced the window start from {} to {}.",
                        char::from(byte),
                        left,
                        previous + 1
                    ),
                );
                left = previous + 1;
            }
            last_seen[byte as usize] = right;

            let window_len = right - left + 1;
            if window_len > best_len {
                best_len = window_len;
                best_start = left;
                AgentLogger::log(
                    AgentFeedback::Success,
                    format!(
                        "Expanded the best unique window to byte range [{}..{}).",
                        best_start,
                        best_start + best_len
                    ),
                );
            }
        }

        Some((best_start, best_len))
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct LongestSubstringRequest {
    pub text: String,
}

#[macros::mcp_tool(
    name = "arrays_strings.longest_substring",
    description = "Use this for solving longest substring problems. Trigger Keywords: longest_substring, longest substring, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_longest_substring(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_longest_substring(payload: Value) -> DsaResult<ResultBox> {
    let req: LongestSubstringRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid LongestSubstringRequest: {e}"),
            hint: "Provide 'text'.".to_string(),
        })?;

    let length = LongestSubstring::solve(&req.text);
    let solver = LongestSubstring;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });

    Ok(ResultBox::success(json!({
        "length": length
    }))
    .with_complexity(complexity)
    .with_description("Longest-substring-without-repeats computed."))
}

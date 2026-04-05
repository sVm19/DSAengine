use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: String Toolkit
/// CATEGORY: arrays-strings
/// DESCRIPTION: Resolves the longest common prefix across borrowed strings without copying them.
pub struct StringToolkit;

impl Complexity for StringToolkit {
    fn name(&self) -> &'static str {
        "String Toolkit"
    }

    fn time_complexity(&self) -> &'static str {
        "O(total compared bytes) - Stops as soon as any word breaks the shared prefix."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - Returns a borrowed slice into the first string."
    }

    fn description(&self) -> &'static str {
        "Keeps trimming a borrowed prefix candidate until every word agrees on it."
    }
}

impl StringToolkit {
    pub fn solve<'a>(words: &[&'a str]) -> &'a str {
        Self::longest_common_prefix(words)
    }

    pub fn longest_common_prefix<'a>(words: &[&'a str]) -> &'a str {
        if words.is_empty() {
            return "";
        }

        let base = words[0];
        let base_bytes = base.as_bytes();
        let mut prefix_end = base.len();

        for (index, word) in words.iter().enumerate().skip(1) {
            let other = word.as_bytes();
            let limit = prefix_end.min(other.len());
            let mut matched = 0usize;

            while matched < limit && base_bytes[matched] == other[matched] {
                matched += 1;
            }

            while matched > 0 && !base.is_char_boundary(matched) {
                matched -= 1;
            }

            prefix_end = matched;
            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Trimmed the shared prefix to {} bytes after comparing word {}.",
                    prefix_end, index
                ),
            );

            if prefix_end == 0 {
                break;
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Resolved longest common prefix of {} bytes.", prefix_end),
        );
        &base[..prefix_end]
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "arrays_strings.string_toolkit", description = "Use this for solving string toolkit problems. Trigger Keywords: string_toolkit, string toolkit, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_string_toolkit(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

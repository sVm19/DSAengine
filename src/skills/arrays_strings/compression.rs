use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Compression
/// CATEGORY: arrays-strings
/// DESCRIPTION: Compresses repeated characters in place using run-length encoding.
pub struct Compression;

impl Complexity for Compression {
    fn name(&self) -> &'static str {
        "Compression"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Each character is read once and written at most once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - The encoded result is written back into the original slice."
    }

    fn description(&self) -> &'static str {
        "Applies in-place run-length encoding over a mutable character buffer."
    }
}

impl Compression {
    pub fn solve(chars: &mut [char]) -> usize {
        Self::compress(chars)
    }

    pub fn compress(chars: &mut [char]) -> usize {
        if chars.is_empty() {
            return 0;
        }

        let mut read = 0;
        let mut write = 0;

        while read < chars.len() {
            let current = chars[read];
            let run_start = read;
            while read < chars.len() && chars[read] == current {
                read += 1;
            }

            let run_length = read - run_start;
            chars[write] = current;
            write += 1;

            AgentLogger::log(
                AgentFeedback::Step,
                format!("Encoding run '{}' with length {}.", current, run_length),
            );

            if run_length > 1 {
                write += Self::write_run_length(chars, write, run_length);
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Compressed buffer now occupies {} cells.", write),
        );
        write
    }

    fn write_run_length(chars: &mut [char], mut write: usize, mut run_length: usize) -> usize {
        let start = write;
        let mut digits = ['0'; 20];
        let mut used = 0;

        while run_length > 0 {
            digits[used] = char::from(b'0' + (run_length % 10) as u8);
            used += 1;
            run_length /= 10;
        }

        for index in (0..used).rev() {
            chars[write] = digits[index];
            write += 1;
        }

        write - start
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "arrays_strings.compression", description = "Use this for solving compression problems. Trigger Keywords: compression, compression, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_compression(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::cmp::Ordering;

/// SKILL: Suffix Array Lite
/// CATEGORY: arrays-strings
/// DESCRIPTION: Builds a straightforward suffix array by sorting borrowed suffix offsets.
pub struct SuffixArrayLite;
pub struct SuffixArrayIndex<'a> {
    text: &'a [u8],
    order: Vec<usize>,
}

impl Complexity for SuffixArrayLite {
    fn name(&self) -> &'static str {
        "Suffix Array Lite"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n^2 log n) build, O(m log n + occ) query - Suffix comparisons are lexicographic over borrowed tails."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) - Stores one sorted starting index per suffix."
    }

    fn description(&self) -> &'static str {
        "A simple suffix-array index for substring search without copying the source text."
    }
}

impl SuffixArrayLite {
    pub fn build<'a>(text: &'a str) -> SuffixArrayIndex<'a> {
        let bytes = text.as_bytes();
        let mut order = (0..bytes.len()).collect::<Vec<_>>();
        order.sort_unstable_by(|&left, &right| bytes[left..].cmp(&bytes[right..]));

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Sorted {} suffix starting offsets.", order.len()),
        );

        SuffixArrayIndex { text: bytes, order }
    }
}

impl<'a> SuffixArrayIndex<'a> {
    pub fn search(&self, pattern: &str) -> Vec<usize> {
        let pattern = pattern.as_bytes();
        if pattern.is_empty() {
            return Vec::new();
        }

        let start = self.lower_bound(pattern);
        let mut matches = Vec::new();
        let mut index = start;

        while index < self.order.len() && self.text[self.order[index]..].starts_with(pattern) {
            matches.push(self.order[index]);
            index += 1;
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Suffix-array lookup produced {} match(es).", matches.len()),
        );
        matches
    }

    fn lower_bound(&self, pattern: &[u8]) -> usize {
        let mut left = 0usize;
        let mut right = self.order.len();

        while left < right {
            let mid = left + (right - left) / 2;
            let suffix = &self.text[self.order[mid]..];
            if Self::compare_suffix(suffix, pattern) == Ordering::Less {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }

    fn compare_suffix(suffix: &[u8], pattern: &[u8]) -> Ordering {
        let shared = suffix.len().min(pattern.len());
        match suffix[..shared].cmp(&pattern[..shared]) {
            Ordering::Equal => suffix.len().cmp(&pattern.len()),
            ordering => ordering,
        }
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "arrays_strings.suffix_array_lite",
    description = "Use this for solving suffix array lite problems. Trigger Keywords: suffix_array_lite, suffix array lite, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_suffix_array_lite(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SuffixArrayLiteRequest {
    pub text: String,
    pub pattern: String,
}

async fn handle_suffix_array_lite(payload: Value) -> DsaResult<ResultBox> {
    let req: SuffixArrayLiteRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid SuffixArrayLiteRequest: {e}"),
            hint: "Provide 'text' and 'pattern' strings.".to_string(),
        })?;

    let index = SuffixArrayLite::build(&req.text);
    let matches = index.search(&req.pattern);
    let solver = SuffixArrayLite;

    Ok(ResultBox::success(json!({
        "text_length": req.text.len(),
        "pattern": req.pattern,
        "matches": matches
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Suffix-array substring search completed."))
}

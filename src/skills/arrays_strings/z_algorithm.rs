use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Z-Algorithm
/// CATEGORY: arrays-strings
/// DESCRIPTION: A linear time string matching algorithm that finds all occurrences of a pattern in a text.
pub struct ZAlgorithm;

impl Complexity for ZAlgorithm {
    fn name(&self) -> &'static str {
        "Z-Algorithm (Linear String Matching)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n + m) - Single pass after concatenating pattern and text."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n + m) - To store the Z-array."
    }

    fn description(&self) -> &'static str {
        "Uses the Z-box approach to find the longest common prefix between the suffix and the prefix."
    }
}

impl ZAlgorithm {
    pub fn solve(text: &str, pattern: &str) -> Vec<usize> {
        Self::search(text, pattern)
    }

    /// Constructs the Z-array for a given integer sequence.
    pub fn calculate_z(sequence: &[u16]) -> Vec<usize> {
        let mut z = vec![0; sequence.len()];
        let mut left = 0usize;
        let mut right = 0usize;

        for i in 1..sequence.len() {
            if i < right {
                z[i] = (right - i).min(z[i - left]);
            }
            while i + z[i] < sequence.len() && sequence[z[i]] == sequence[i + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > right {
                left = i;
                right = i + z[i];
            }
        }
        z
    }

    /// Searches for a pattern in a text using the Z-array.
    /// Concatenates: Pattern + '$' + Text
    pub fn search(text: &str, pattern: &str) -> Vec<usize> {
        let text_bytes = text.as_bytes();
        let pattern_bytes = pattern.as_bytes();
        if pattern_bytes.is_empty() || pattern_bytes.len() > text_bytes.len() {
            return vec![];
        }

        let mut combined = Vec::with_capacity(pattern_bytes.len() + text_bytes.len() + 1);
        combined.extend(pattern_bytes.iter().map(|&byte| byte as u16));
        combined.push(256);
        combined.extend(text_bytes.iter().map(|&byte| byte as u16));

        let z = Self::calculate_z(&combined);
        let mut results = Vec::new();

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Built combined pattern/text buffer for '{pattern}'."),
        );

        for i in pattern_bytes.len() + 1..z.len() {
            if z[i] == pattern_bytes.len() {
                let start = i - pattern_bytes.len() - 1;
                results.push(start);
                AgentLogger::log(
                    AgentFeedback::Success,
                    format!("Z-box reported a pattern match at byte offset {}.", start),
                );
            }
        }
        results
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct ZAlgorithmRequest {
    pub text: String,
    pub pattern: String,
}

#[macros::mcp_tool(name = "arrays_strings.z_algorithm", description = "Use this for solving z algorithm problems. Trigger Keywords: z_algorithm, z algorithm, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_z_algorithm(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_z_algorithm(payload: Value) -> DsaResult<ResultBox> {
    let req: ZAlgorithmRequest = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid ZAlgorithmRequest: {e}"),
        hint: "Provide 'text' and 'pattern'.".to_string(),
    })?;
    let matches = ZAlgorithm::solve(&req.text, &req.pattern);
    let solver = ZAlgorithm;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });
    Ok(ResultBox::success(json!({
        "matches": matches
    }))
    .with_complexity(complexity)
    .with_description("Z-algorithm string matching completed."))
}

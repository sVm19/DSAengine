use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: String Hashing
/// CATEGORY: arrays-strings
/// DESCRIPTION: Builds prefix rolling hashes for constant-time substring comparisons.
pub struct StringHashing;
pub struct RollingHash<'a> {
    bytes: &'a [u8],
    prefix: Vec<u64>,
    power: Vec<u64>,
    base: u64,
    modulus: u64,
}

impl Complexity for StringHashing {
    fn name(&self) -> &'static str {
        "String Hashing"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) build, O(1) substring hash - Prefix hashes amortize later range queries."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) - Stores prefix hashes and powers for the source string."
    }

    fn description(&self) -> &'static str {
        "Uses polynomial rolling hashes to compare substrings without copying them."
    }
}

impl StringHashing {
    const BASE: u64 = 911_382_323;
    const MODULUS: u64 = 972_663_749;

    pub fn build<'a>(text: &'a str) -> RollingHash<'a> {
        let bytes = text.as_bytes();
        let mut prefix = vec![0u64; bytes.len() + 1];
        let mut power = vec![1u64; bytes.len() + 1];

        for (index, &byte) in bytes.iter().enumerate() {
            prefix[index + 1] = (prefix[index] * Self::BASE + byte as u64 + 1) % Self::MODULUS;
            power[index + 1] = (power[index] * Self::BASE) % Self::MODULUS;
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Built prefix hashes for {} bytes.", bytes.len()),
        );

        RollingHash {
            bytes,
            prefix,
            power,
            base: Self::BASE,
            modulus: Self::MODULUS,
        }
    }
}

impl<'a> RollingHash<'a> {
    pub fn hash(&self, left: usize, right: usize) -> Option<u64> {
        if left > right || right >= self.bytes.len() {
            return None;
        }

        let len = right - left + 1;
        let scaled = (self.prefix[left] * self.power[len]) % self.modulus;
        let value = (self.prefix[right + 1] + self.modulus - scaled) % self.modulus;
        Some(value)
    }

    pub fn equals(&self, left_a: usize, right_a: usize, left_b: usize, right_b: usize) -> bool {
        if right_a < left_a || right_b < left_b || right_a - left_a != right_b - left_b {
            return false;
        }

        let same_hash = self.hash(left_a, right_a) == self.hash(left_b, right_b);
        let same_bytes = self.bytes[left_a..=right_a] == self.bytes[left_b..=right_b];
        if same_hash && same_bytes {
            AgentLogger::log(
                AgentFeedback::Success,
                format!(
                    "Matched substrings [{}..={}] and [{}..={}].",
                    left_a, right_a, left_b, right_b
                ),
            );
        }
        same_hash && same_bytes
    }

    pub fn parameters(&self) -> (u64, u64) {
        (self.base, self.modulus)
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "arrays_strings.string_hashing",
    description = "Use this for solving string hashing problems. Trigger Keywords: string_hashing, string hashing, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_string_hashing(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct StringHashingRequest {
    pub text: String,
    pub ranges: Option<Vec<[usize; 2]>>,
    pub equals: Option<Vec<[usize; 4]>>,
}

async fn handle_string_hashing(payload: Value) -> DsaResult<ResultBox> {
    let req: StringHashingRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid StringHashingRequest: {e}"),
            hint: "Provide 'text', optional 'ranges' [left,right], and optional 'equals' [left_a,right_a,left_b,right_b].".to_string(),
        })?;

    let index = StringHashing::build(&req.text);
    let ranges = req.ranges.unwrap_or_else(|| {
        if req.text.is_empty() {
            Vec::new()
        } else {
            vec![[0, req.text.len() - 1]]
        }
    });
    let hashes: Vec<_> = ranges
        .iter()
        .map(|range| {
            json!({
                "left": range[0],
                "right": range[1],
                "hash": index.hash(range[0], range[1])
            })
        })
        .collect();

    let equals: Vec<_> = req
        .equals
        .unwrap_or_default()
        .iter()
        .map(|range| {
            json!({
                "left_a": range[0],
                "right_a": range[1],
                "left_b": range[2],
                "right_b": range[3],
                "equal": index.equals(range[0], range[1], range[2], range[3])
            })
        })
        .collect();
    let (base, modulus) = index.parameters();
    let solver = StringHashing;

    Ok(ResultBox::success(json!({
        "text_length": req.text.len(),
        "base": base,
        "modulus": modulus,
        "hashes": hashes,
        "equals": equals
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("String hashing queries completed."))
}

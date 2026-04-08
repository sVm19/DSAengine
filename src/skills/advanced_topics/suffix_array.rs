use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::cmp::Ordering;

/// SKILL: Suffix Array
/// CATEGORY: advanced-topics
/// DESCRIPTION: Builds a stronger suffix array with the prefix-doubling technique and binary-search lookup.
pub struct SuffixArray;
pub struct SuffixArrayIndex<'a> {
    text: &'a [u8],
    order: Vec<usize>,
    rank: Vec<usize>,
}

impl Complexity for SuffixArray {
    fn name(&self) -> &'static str {
        "Suffix Array"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n log^2 n) build and O(m log n + occ) search with prefix-doubling plus binary search."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) - Stores suffix order plus rank metadata."
    }

    fn description(&self) -> &'static str {
        "Ranks suffixes in doubling windows so substring searches can be answered over a sorted suffix order."
    }
}

impl SuffixArray {
    pub fn solve<'a>(text: &'a str) -> SuffixArrayIndex<'a> {
        Self::build(text)
    }

    pub fn build<'a>(text: &'a str) -> SuffixArrayIndex<'a> {
        let bytes = text.as_bytes();
        if bytes.is_empty() {
            return SuffixArrayIndex {
                text: bytes,
                order: Vec::new(),
                rank: Vec::new(),
            };
        }

        let len = bytes.len();
        let mut order = (0..len).collect::<Vec<_>>();
        let mut rank = bytes
            .iter()
            .map(|&byte| byte as usize + 1)
            .collect::<Vec<_>>();
        let mut next_rank = vec![0usize; len];
        let mut width = 1usize;

        while width < len {
            order.sort_unstable_by_key(|&index| {
                (
                    rank[index],
                    if index + width < len {
                        rank[index + width]
                    } else {
                        0
                    },
                )
            });

            next_rank[order[0]] = 1;
            for position in 1..len {
                let current = order[position];
                let previous = order[position - 1];
                let current_key = (
                    rank[current],
                    if current + width < len {
                        rank[current + width]
                    } else {
                        0
                    },
                );
                let previous_key = (
                    rank[previous],
                    if previous + width < len {
                        rank[previous + width]
                    } else {
                        0
                    },
                );
                next_rank[current] = next_rank[previous] + usize::from(current_key != previous_key);
            }

            rank.copy_from_slice(&next_rank);
            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Completed suffix-array ranking round with comparison width {}.",
                    width
                ),
            );

            if rank[order[len - 1]] == len {
                break;
            }
            width <<= 1;
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Advanced suffix array built for {} bytes.", len),
        );

        SuffixArrayIndex {
            text: bytes,
            order,
            rank,
        }
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
            format!("Suffix-array search returned {} match(es).", matches.len()),
        );
        matches
    }

    pub fn suffix_rank(&self, start: usize) -> Option<usize> {
        self.rank.get(start).copied().map(|rank| rank - 1)
    }

    fn lower_bound(&self, pattern: &[u8]) -> usize {
        let mut left = 0usize;
        let mut right = self.order.len();

        while left < right {
            let mid = left + (right - left) / 2;
            let suffix = &self.text[self.order[mid]..];
            if Self::compare_suffix_to_pattern(suffix, pattern) == Ordering::Less {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }

    fn compare_suffix_to_pattern(suffix: &[u8], pattern: &[u8]) -> Ordering {
        let shared = suffix.len().min(pattern.len());
        match suffix[..shared].cmp(&pattern[..shared]) {
            Ordering::Equal => suffix.len().cmp(&pattern.len()),
            ordering => ordering,
        }
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct SuffixArrayRequest {
    /// The text to build the suffix array for.
    pub text: String,
    /// Optional pattern to search for in the text.
    pub query: Option<String>,
}

#[macros::mcp_tool(name = "advanced_topics.suffix_array", description = "Use this for solving suffix array problems. Trigger Keywords: suffix_array, suffix array, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_suffix_array(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_suffix_array(payload: Value) -> DsaResult<ResultBox<serde_json::Value>> {
    let req: SuffixArrayRequest = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid SuffixArrayRequest: {e}"),
        hint: "Provide 'text' and an optional 'query'.".to_string(),
    })?;

    if req.text.is_empty() {
        return Err(DsaError::InvalidInput {
            message: "Text cannot be empty.".to_string(),
            hint: "Provide a non-empty string to build a suffix array.".to_string(),
        });
    }

    let sa_index = SuffixArray::solve(&req.text);
    let mut matches = Vec::new();

    if let Some(ref q) = req.query {
        matches = sa_index.search(q);
    }

    let solver = SuffixArray;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });

    let res_val = json!({
        "suffix_order": sa_index.order,
        "query_matches": if req.query.is_some() { Some(matches) } else { None }
    });

    Ok(ResultBox::success(res_val)
        .with_complexity(complexity)
        .with_description("Suffix array construction and search completed."))
}

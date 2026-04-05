use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Sparse Table
/// CATEGORY: arrays-strings
/// DESCRIPTION: Builds a static range-minimum-query index using overlapping powers of two.
pub struct SparseTable;
pub struct RangeMinimumTable<'a> {
    values: &'a [i32],
    table: Vec<Vec<usize>>,
    log2: Vec<usize>,
}

impl Complexity for SparseTable {
    fn name(&self) -> &'static str {
        "Sparse Table"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n log n) build, O(1) query - Precomputes powers-of-two intervals once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n log n) - Stores the best index for every interval size."
    }

    fn description(&self) -> &'static str {
        "Supports immutable range minimum queries by comparing two overlapping blocks."
    }
}

impl SparseTable {
    pub fn build<'a>(values: &'a [i32]) -> Option<RangeMinimumTable<'a>> {
        if values.is_empty() {
            return None;
        }

        let n = values.len();
        let levels = (usize::BITS - n.leading_zeros()) as usize;
        let mut log2 = vec![0usize; n + 1];
        for length in 2..=n {
            log2[length] = log2[length / 2] + 1;
        }

        let mut table: Vec<Vec<usize>> = Vec::with_capacity(levels);
        table.push((0..n).collect());

        for level in 1..levels {
            let width = 1usize << level;
            let half = width >> 1;
            let mut row = Vec::with_capacity(n - width + 1);
            for start in 0..=n - width {
                let left_index = table[level - 1][start];
                let right_index = table[level - 1][start + half];
                row.push(if values[left_index] <= values[right_index] {
                    left_index
                } else {
                    right_index
                });
            }
            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Built sparse-table level {} for interval width {}.",
                    level, width
                ),
            );
            table.push(row);
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Sparse table ready for {} source values.", n),
        );

        Some(RangeMinimumTable {
            values,
            table,
            log2,
        })
    }
}

impl<'a> RangeMinimumTable<'a> {
    pub fn query(&self, left: usize, right: usize) -> Option<(usize, i32)> {
        if left > right || right >= self.values.len() {
            return None;
        }

        let span = right - left + 1;
        let level = self.log2[span];
        let block = 1usize << level;
        let left_index = self.table[level][left];
        let right_index = self.table[level][right + 1 - block];
        let best_index = if self.values[left_index] <= self.values[right_index] {
            left_index
        } else {
            right_index
        };

        AgentLogger::log(
            AgentFeedback::Step,
            format!(
                "Answered RMQ [{}, {}] via level {} blocks anchored at {} and {}.",
                left,
                right,
                level,
                left,
                right + 1 - block
            ),
        );

        Some((best_index, self.values[best_index]))
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "arrays_strings.sparse_table", description = "Use this for solving sparse table problems. Trigger Keywords: sparse_table, sparse table, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_sparse_table(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

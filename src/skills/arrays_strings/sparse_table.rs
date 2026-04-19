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
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "arrays_strings.sparse_table",
    description = "Use this for solving sparse table problems. Trigger Keywords: sparse_table, sparse table, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_sparse_table(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SparseTableRequest {
    pub values: Vec<i32>,
    pub queries: Option<Vec<[usize; 2]>>,
}

async fn handle_sparse_table(payload: Value) -> DsaResult<ResultBox> {
    let req: SparseTableRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid SparseTableRequest: {e}"),
            hint: "Provide 'values' and optional 'queries' as [left, right] pairs.".to_string(),
        })?;

    let table = SparseTable::build(&req.values).ok_or_else(|| DsaError::InvalidInput {
        message: "values cannot be empty.".to_string(),
        hint: "Provide at least one integer.".to_string(),
    })?;

    let queries = req
        .queries
        .unwrap_or_else(|| vec![[0, req.values.len() - 1]]);
    let results: Vec<_> = queries
        .iter()
        .map(|query| {
            let result = table.query(query[0], query[1]);
            json!({
                "left": query[0],
                "right": query[1],
                "minimum": result.map(|(index, value)| json!({ "index": index, "value": value }))
            })
        })
        .collect();

    let solver = SparseTable;
    Ok(ResultBox::success(json!({
        "values": req.values,
        "queries": results
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Sparse-table range-minimum queries completed."))
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Matrix Search
/// CATEGORY: sorting-searching
/// DESCRIPTION: Searches for a target value in a 2D matrix where rows and columns
///              are sorted in ascending order from left-to-right and top-to-bottom.
pub struct MatrixSearch;

impl Complexity for MatrixSearch {
    fn name(&self) -> &'static str {
        "Matrix Search (Step-Wise Top-Right Reduction)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(M + N) — Each step moves either down a row or left a column. M rows, N columns."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Uses only x and y traversal coordinates."
    }

    fn description(&self) -> &'static str {
        "Starts search at the top-right corner. If target < curr, we can discard the entire column by moving left. If target > curr, we can discard the entire row by moving down."
    }
}

impl MatrixSearch {
    /// Returns `true` if `target` is in the `matrix`, otherwise `false`.
    pub fn solve(matrix: &[Vec<i32>], target: i32) -> bool {
        let rows = matrix.len();
        if rows == 0 {
            return false;
        }
        let cols = matrix[0].len();
        if cols == 0 {
            return false;
        }

        let mut r = 0usize;
        let mut c = cols - 1; // Top-right corner

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Matrix search in {rows}x{cols} grid for target {target}."),
        );

        while r < rows {
            let val = matrix[r][c];
            AgentLogger::log(
                AgentFeedback::Step,
                format!("Checking cell ({r}, {c}) = {val}."),
            );

            if val == target {
                AgentLogger::log(
                    AgentFeedback::Success,
                    format!("Target {target} found at ({r}, {c})."),
                );
                return true;
            } else if target < val {
                if c == 0 {
                    break;
                }
                c -= 1; // Move left
            } else {
                r += 1; // Move down
            }
        }

        AgentLogger::log(
            AgentFeedback::Warning,
            format!("Target {target} not found in matrix."),
        );
        false
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "sorting_searching.matrix_search",
    description = "Use this for solving matrix search problems. Trigger Keywords: sorting, searching, matrix_search. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_matrix_search(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct MatrixSearchRequest {
    pub matrix: Vec<Vec<i32>>,
    pub target: i32,
}

async fn handle_matrix_search(payload: Value) -> DsaResult<ResultBox> {
    let req: MatrixSearchRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid MatrixSearchRequest: {e}"),
            hint: "Provide 'matrix' as a sorted 2D integer array and 'target'.".to_string(),
        })?;

    if let Some(first) = req.matrix.first() {
        if req.matrix.iter().any(|row| row.len() != first.len()) {
            return Err(DsaError::InvalidInput {
                message: "matrix must be rectangular.".to_string(),
                hint: "Every row must have the same length.".to_string(),
            });
        }
    }

    let found = MatrixSearch::solve(&req.matrix, req.target);
    let solver = MatrixSearch;
    Ok(ResultBox::success(json!({
        "target": req.target,
        "found": found
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Matrix search completed."))
}

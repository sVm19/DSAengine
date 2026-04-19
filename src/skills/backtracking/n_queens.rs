use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: N-Queens
/// CATEGORY: backtracking
/// DESCRIPTION: Places N non-attacking queens on an N×N board iteratively,
///              using three bitsets to track column / diagonal / anti-diagonal conflicts.
pub struct NQueens;

impl Complexity for NQueens {
    fn name(&self) -> &'static str {
        "N-Queens (Bitset Conflict Tracking)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N!) upper bound — prunes columns and diagonals already under attack; actual solutions are far fewer."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — One queen placement per row is tracked in the path array; three u64 masks suffice for N ≤ 64."
    }

    fn description(&self) -> &'static str {
        "Uses column, forward-diagonal, and backward-diagonal bitmasks to prune attacks in O(1) per candidate, iteratively across rows."
    }
}

impl NQueens {
    /// Returns all distinct queen placement solutions for an N×N board.
    /// Each solution is a `Vec<usize>` where `solution[row] = column`.
    pub fn solve(n: usize) -> Vec<Vec<usize>> {
        if n == 0 || n > 64 {
            return Vec::new();
        }

        let mut results: Vec<Vec<usize>> = Vec::new();
        // placement[row] = column index of the queen placed in that row.
        let mut placement = vec![0usize; n];

        // Bitmask of taken columns (bit i = column i is occupied).
        let mut col_mask: u64 = 0;
        // Bitmask of taken "/" diagonals (indexed by row + col).
        let mut fwd_mask: u64 = 0;
        // Bitmask of taken "\" diagonals (indexed by row - col + n - 1).
        let mut bwd_mask: u64 = 0;

        // Iterative row-by-row stack: each slot stores the next column to try.
        let mut col_cursor = vec![0usize; n];
        let mut row = 0usize;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Starting N-Queens search for board size {n} with bitmask pruning."),
        );

        loop {
            if col_cursor[row] == n {
                // All columns exhausted at this row — backtrack.
                if row == 0 {
                    break;
                }
                col_cursor[row] = 0;
                row -= 1;

                // Undo the queen placed in `row` at column `placement[row]`.
                let col = placement[row];
                let fwd_bit = row + col;
                let bwd_bit = row + n - 1 - col;
                col_mask &= !(1u64 << col);
                fwd_mask &= !(1u64 << fwd_bit);
                bwd_mask &= !(1u64 << bwd_bit);

                col_cursor[row] += 1; // Try the next column at the restored row.
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Backtracked to row {row}, will try column {} next.",
                        col_cursor[row]
                    ),
                );
                continue;
            }

            let col = col_cursor[row];
            let fwd_bit = row + col;
            let bwd_bit = row + n - 1 - col;

            let under_attack = (col_mask >> col) & 1 == 1
                || (fwd_mask >> fwd_bit) & 1 == 1
                || (bwd_mask >> bwd_bit) & 1 == 1;

            if under_attack {
                col_cursor[row] += 1;
                continue;
            }

            // Place queen.
            placement[row] = col;
            col_mask |= 1u64 << col;
            fwd_mask |= 1u64 << fwd_bit;
            bwd_mask |= 1u64 << bwd_bit;

            AgentLogger::log(
                AgentFeedback::Step,
                format!("Placed queen at ({row}, {col}); col_mask={col_mask:#b}."),
            );

            if row + 1 == n {
                // Complete solution found.
                AgentLogger::log(
                    AgentFeedback::Success,
                    format!("Found a valid placement: {:?}.", placement),
                );
                results.push(placement.clone());

                // Immediately undo to search further.
                col_mask &= !(1u64 << col);
                fwd_mask &= !(1u64 << fwd_bit);
                bwd_mask &= !(1u64 << bwd_bit);
                col_cursor[row] += 1;
            } else {
                // Advance to next row.
                row += 1;
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "N-Queens({n}) found {} distinct solution(s).",
                results.len()
            ),
        );
        results
    }

    /// Returns only the solution count without storing boards — fast for large N.
    pub fn count(n: usize) -> usize {
        Self::solve(n).len()
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "backtracking.n_queens",
    description = "Use this for solving n queens problems. Trigger Keywords: n_queens, n queens, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_n_queens(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_n_queens(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        n: usize,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'n' (board size). Optional 'mode': 'solutions' | 'count'.".to_string(),
    })?;

    let result = match req.mode.as_deref().unwrap_or("") {
        "count" => {
            let count = NQueens::count(req.n);
            json!({ "mode": "count", "n": req.n, "total_solutions": count })
        }
        _ => {
            let solutions = NQueens::solve(req.n);
            let count = solutions.len();
            json!({ "mode": "solutions", "n": req.n, "total_solutions": count, "boards": solutions })
        }
    };

    let solver = NQueens;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["solutions", "count"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("N-Queens solver completed."))
}

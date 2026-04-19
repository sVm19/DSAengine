use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Sudoku Solver
/// CATEGORY: backtracking
/// DESCRIPTION: Solves a standard 9×9 Sudoku puzzle in-place using constraint-propagation
///              bitmasks (row, column, box) and iterative cell-by-cell backtracking.
pub struct SudokuSolver;

impl Complexity for SudokuSolver {
    fn name(&self) -> &'static str {
        "Sudoku Solver (Bitmask Constraint Backtracking)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(9^E) — E is the number of empty cells; bitmask pruning reduces branching deeply in practice."
    }

    fn space_complexity(&self) -> &'static str {
        "O(E) — The iterative stack holds one entry per empty cell; three 9-bit constraint masks per unit."
    }

    fn description(&self) -> &'static str {
        "Maintains row / column / box bitmasks so each candidate check is O(1). Iterates forward when a digit is placed and backtracks the stack when all digits are exhausted."
    }
}

/// Internal state for a single backtracking step.
struct Step {
    /// Flat index (0..81) of the cell being decided.
    cell: usize,
    /// Bitmask of candidates still to try at this cell (bits 1–9 = digits 1–9).
    remaining: u16,
    /// The digit placed during the last forward move from this step.
    placed: u8,
}

impl SudokuSolver {
    /// Solves the board in-place.  Returns `true` if a solution was found.
    ///
    /// `board` is a flat 81-element array (row-major).
    /// `0` represents an empty cell; values 1–9 are givens.
    pub fn solve(board: &mut [u8; 81]) -> bool {
        // Build initial constraint masks.
        let mut row_mask = [0u16; 9]; // bit d set → digit d+1 used in this row
        let mut col_mask = [0u16; 9];
        let mut box_mask = [0u16; 9];

        for cell in 0..81usize {
            let digit = board[cell];
            if digit != 0 {
                let row = cell / 9;
                let col = cell % 9;
                let bx = (row / 3) * 3 + col / 3;
                let bit = 1u16 << (digit - 1);
                row_mask[row] |= bit;
                col_mask[col] |= bit;
                box_mask[bx] |= bit;
            }
        }

        // Collect empty cells in order.
        let empty: Vec<usize> = (0..81).filter(|&c| board[c] == 0).collect();

        if empty.is_empty() {
            return Self::is_valid(board);
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Sudoku: {} empty cells detected; starting bitmask-constrained search.",
                empty.len()
            ),
        );

        // Build an iterative stack matching the empty-cell list.
        let mut stack: Vec<Step> = Vec::with_capacity(empty.len());

        // Push the first empty cell.
        let first = empty[0];
        let first_candidates = Self::candidates(first, &row_mask, &col_mask, &box_mask);
        stack.push(Step {
            cell: first,
            remaining: first_candidates,
            placed: 0,
        });

        loop {
            let frame = match stack.last_mut() {
                Some(f) => f,
                None => return false, // No more candidates anywhere → unsolvable.
            };

            // Undo the previously placed digit for this cell (if any).
            if frame.placed != 0 {
                let cell = frame.cell;
                let row = cell / 9;
                let col = cell % 9;
                let bx = (row / 3) * 3 + col / 3;
                let bit = 1u16 << (frame.placed - 1);
                row_mask[row] &= !bit;
                col_mask[col] &= !bit;
                box_mask[bx] &= !bit;
                board[cell] = 0;
                frame.placed = 0;
            }

            // Pick the next candidate digit from remaining.
            if frame.remaining == 0 {
                let cell = frame.cell;
                // Backtrack.
                stack.pop();
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Backtracking — no candidates remain for cell {}.", cell),
                );
                continue;
            }

            let digit_bit_index = frame.remaining.trailing_zeros() as u8; // 0-indexed
            let digit = digit_bit_index + 1;
            frame.remaining &= frame.remaining - 1; // Clear lowest set bit.
            frame.placed = digit;

            let cell = frame.cell;
            let row = cell / 9;
            let col = cell % 9;
            let bx = (row / 3) * 3 + col / 3;
            let bit = 1u16 << digit_bit_index;

            row_mask[row] |= bit;
            col_mask[col] |= bit;
            box_mask[bx] |= bit;
            board[cell] = digit;

            AgentLogger::log(
                AgentFeedback::Step,
                format!("Placed digit {digit} at cell ({row},{col}); box {bx} constraint updated."),
            );

            // Advance to the next empty cell.
            let next_depth = stack.len(); // stack.len() used as index into `empty`.
            if next_depth == empty.len() {
                // All empty cells filled — solution found.
                AgentLogger::log(AgentFeedback::Success, "Sudoku solved successfully.");
                return true;
            }

            let next_cell = empty[next_depth];
            let candidates = Self::candidates(next_cell, &row_mask, &col_mask, &box_mask);
            stack.push(Step {
                cell: next_cell,
                remaining: candidates,
                placed: 0,
            });
        }
    }

    /// Computes the candidate digit bitmask for a cell (bits 0–8 = digits 1–9).
    fn candidates(
        cell: usize,
        row_mask: &[u16; 9],
        col_mask: &[u16; 9],
        box_mask: &[u16; 9],
    ) -> u16 {
        let row = cell / 9;
        let col = cell % 9;
        let bx = (row / 3) * 3 + col / 3;
        let used = row_mask[row] | col_mask[col] | box_mask[bx];
        !used & 0x1FF // Only bits 0–8 are valid.
    }

    /// Checks that a completed board has no constraint violations.
    fn is_valid(board: &[u8; 81]) -> bool {
        for unit in 0..9usize {
            let mut row_seen = 0u16;
            let mut col_seen = 0u16;
            let mut box_seen = 0u16;
            for j in 0..9usize {
                let r_digit = board[unit * 9 + j];
                let c_digit = board[j * 9 + unit];
                let box_row = (unit / 3) * 3 + j / 3;
                let box_col = (unit % 3) * 3 + j % 3;
                let b_digit = board[box_row * 9 + box_col];

                for (digit, seen) in [
                    (r_digit, &mut row_seen),
                    (c_digit, &mut col_seen),
                    (b_digit, &mut box_seen),
                ] {
                    if digit == 0 {
                        return false;
                    }
                    let bit = 1u16 << (digit - 1);
                    if *seen & bit != 0 {
                        return false;
                    }
                    *seen |= bit;
                }
            }
        }
        true
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "sudoku_solver",
    description = "Use this for solving sudoku solver problems. Trigger Keywords: sudoku_solver, sudoku solver, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_sudoku_solver(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_sudoku_solver(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        board: Vec<u8>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'board' (array of 81 u8 values; 0=empty). Row-major order.".to_string(),
    })?;

    let result = {
        if req.board.len() != 81 {
            return Err(DsaError::InvalidInput {
                message: format!("Board must be exactly 81 cells, got {}.", req.board.len()),
                hint: "Provide a flat array of 81 u8 values (0-9) in row-major order.".to_string(),
            });
        }
        let mut board: [u8; 81] = [0; 81];
        board.copy_from_slice(&req.board);
        let solvable = SudokuSolver::solve(&mut board);
        json!({ "solvable": solvable, "board": board.to_vec() })
    };

    let solver = SudokuSolver;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["solve"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Sudoku solver completed."))
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Word Search
/// CATEGORY: backtracking
/// DESCRIPTION: Determines whether a target word exists in a 2-D character grid by
///              exploring 4-directional paths iteratively with a visited bitfield.
pub struct WordSearch;

impl Complexity for WordSearch {
    fn name(&self) -> &'static str {
        "Word Search (Iterative 4-Dir DFS)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(R * C * 4^L) — L is word length; each of R*C starting cells drives a bounded 4-directional walk."
    }

    fn space_complexity(&self) -> &'static str {
        "O(R*C / 64) visited bitfield + O(L) stack depth — no recursion."
    }

    fn description(&self) -> &'static str {
        "Launches an iterative DFS from every cell matching word[0]; a visited bitfield prevents reuse mid-path and is restored on backtrack."
    }
}

/// A single DFS stack frame for the word-search traversal.
struct SearchFrame {
    row: usize,
    col: usize,
    /// Index of the next direction to explore from this cell.
    dir_index: usize,
    /// The word index already confirmed by reaching this cell.
    word_index: usize,
}

/// 4-directional deltas: Up, Down, Left, Right.
const DELTA: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl WordSearch {
    /// Returns `true` if `word` can be traced through adjacent cells of `grid`.
    /// Each byte in `word` must match a distinct cell (no reuse within the same path).
    pub fn solve(grid: &[&[u8]], word: &[u8]) -> bool {
        let rows = grid.len();
        if rows == 0 || word.is_empty() {
            return word.is_empty();
        }
        let cols = grid[0].len();
        let total = rows * cols;
        let mask_words = (total + 63) / 64;

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Word-search on {rows}×{cols} grid for \"{}\"; scanning for entry points matching first byte.",
                std::str::from_utf8(word).unwrap_or("?")
            ),
        );

        // Try every cell as a potential starting position for word[0].
        for start_row in 0..rows {
            for start_col in 0..cols {
                if grid[start_row][start_col] != word[0] {
                    continue;
                }

                // Only single-character word? Immediate hit.
                if word.len() == 1 {
                    AgentLogger::log(
                        AgentFeedback::Success,
                        format!("Single-char word found at ({start_row},{start_col})."),
                    );
                    return true;
                }

                // Build a fresh visited bitfield.
                let mut visited = vec![0u64; mask_words];
                let flat_start = start_row * cols + start_col;
                visited[flat_start / 64] |= 1u64 << (flat_start % 64);

                let mut stack: Vec<SearchFrame> = Vec::with_capacity(word.len());
                stack.push(SearchFrame {
                    row: start_row,
                    col: start_col,
                    dir_index: 0,
                    word_index: 0,
                });

                'dfs: while let Some(frame) = stack.last_mut() {
                    let row = frame.row;
                    let col = frame.col;
                    let next_word_index = frame.word_index + 1;

                    let mut moved = false;
                    while frame.dir_index < DELTA.len() {
                        let (dr, dc) = DELTA[frame.dir_index];
                        frame.dir_index += 1;

                        let nr = row as i32 + dr;
                        let nc = col as i32 + dc;
                        if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                            continue;
                        }
                        let (nr, nc) = (nr as usize, nc as usize);
                        let flat = nr * cols + nc;

                        if (visited[flat / 64] >> (flat % 64)) & 1 == 1 {
                            continue; // Already on this path.
                        }
                        if grid[nr][nc] != word[next_word_index] {
                            continue;
                        }

                        // Match found at (nr, nc) for word[next_word_index].
                        if next_word_index + 1 == word.len() {
                            AgentLogger::log(
                                AgentFeedback::Success,
                                format!(
                                    "Completed word at ({nr},{nc}) after {} matched bytes.",
                                    word.len()
                                ),
                            );
                            return true;
                        }

                        visited[flat / 64] |= 1u64 << (flat % 64);
                        AgentLogger::log(
                            AgentFeedback::Step,
                            format!(
                                "Matched byte '{}' (index {next_word_index}) at ({nr},{nc}).",
                                word[next_word_index] as char
                            ),
                        );
                        stack.push(SearchFrame {
                            row: nr,
                            col: nc,
                            dir_index: 0,
                            word_index: next_word_index,
                        });
                        moved = true;
                        break;
                    }

                    if !moved {
                        // Backtrack: unmark this cell.
                        let flat = row * cols + col;
                        visited[flat / 64] &= !(1u64 << (flat % 64));
                        stack.pop();
                        AgentLogger::log(
                            AgentFeedback::Step,
                            format!("Backtracked from ({row},{col})."),
                        );
                        if stack.is_empty() {
                            break 'dfs;
                        }
                    }
                }
            }
        }

        AgentLogger::log(
            AgentFeedback::Warning,
            "Word not found in grid after exhausting all starting cells.",
        );
        false
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "word_search", description = "Use this for solving word search problems. Trigger Keywords: word_search, word search, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Rat In Maze
/// CATEGORY: backtracking
/// DESCRIPTION: Finds all paths from the top-left to the bottom-right of a binary grid
///              using iterative DFS with an explicit position/direction stack.
pub struct RatInMaze;

impl Complexity for RatInMaze {
    fn name(&self) -> &'static str {
        "Rat In Maze (Iterative DFS)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(4^(R*C)) worst-case — Up to 4 directional choices per cell; a visited bitfield prunes revisits in O(1)."
    }

    fn space_complexity(&self) -> &'static str {
        "O(R*C / 64) visited bitfield + O(R+C) maximum stack depth — no recursion."
    }

    fn description(&self) -> &'static str {
        "Iteratively explores Down / Right / Up / Left moves via an explicit stack frame, marking visited cells with a flat bitfield and backtracking when all directions are exhausted."
    }
}

/// Directions: Down, Right, Up, Left — standard maze traversal order.
const DIRS: [(i32, i32, &str); 4] = [
    (1, 0, "D"),
    (0, 1, "R"),
    (-1, 0, "U"),
    (0, -1, "L"),
];

/// Stack frame: current cell coordinates and index of next direction to try.
struct Frame {
    row: usize,
    col: usize,
    dir_index: usize,
}

impl RatInMaze {
    /// Returns all paths from (0,0) to (rows-1, cols-1) in a binary grid.
    ///
    /// `grid[r][c] == 1` → open cell; `0` → blocked.
    /// Each path is returned as a string of direction characters (e.g., "DDRR").
    pub fn solve(grid: &[&[u8]]) -> Vec<String> {
        let rows = grid.len();
        if rows == 0 {
            return Vec::new();
        }
        let cols = grid[0].len();
        if cols == 0 || grid[0][0] == 0 || grid[rows - 1][cols - 1] == 0 {
            return Vec::new();
        }

        let mut results: Vec<String> = Vec::new();
        // Flat visited bitfield packed into u64 words.
        let total = rows * cols;
        let mut visited = vec![0u64; (total + 63) / 64];
        let mut path_dirs: Vec<u8> = Vec::with_capacity(rows + cols);

        let mut stack: Vec<Frame> = vec![Frame {
            row: 0,
            col: 0,
            dir_index: 0,
        }];
        // Mark (0,0) as visited.
        visited[0] |= 1u64;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Rat-in-maze DFS on {rows}×{cols} grid from (0,0) to ({},{}).", rows - 1, cols - 1),
        );

        while let Some(frame) = stack.last_mut() {
            let row = frame.row;
            let col = frame.col;

            if row == rows - 1 && col == cols - 1 {
                // Goal reached: emit the current path.
                let path: String = path_dirs.iter().map(|&b| b as char).collect();
                AgentLogger::log(
                    AgentFeedback::Success,
                    format!("Found path to exit: \"{path}\"."),
                );
                results.push(path);

                // Unmark goal and backtrack to find further paths.
                let flat = row * cols + col;
                visited[flat / 64] &= !(1u64 << (flat % 64));
                stack.pop();
                path_dirs.pop();
                continue;
            }

            // Try the next untried direction from this frame.
            let mut moved = false;
            while frame.dir_index < DIRS.len() {
                let (dr, dc, label) = DIRS[frame.dir_index];
                frame.dir_index += 1;

                let nr = row as i32 + dr;
                let nc = col as i32 + dc;
                if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                let flat_next = nr * cols + nc;

                if grid[nr][nc] == 0 || (visited[flat_next / 64] >> (flat_next % 64)) & 1 == 1 {
                    continue;
                }

                // Advance into (nr, nc).
                visited[flat_next / 64] |= 1u64 << (flat_next % 64);
                path_dirs.push(label.as_bytes()[0]);

                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Moving {} to ({nr},{nc}); path depth now {}.", label, path_dirs.len()),
                );

                stack.push(Frame {
                    row: nr,
                    col: nc,
                    dir_index: 0,
                });
                moved = true;
                break;
            }

            if !moved {
                // All directions exhausted at this cell — backtrack.
                let flat = row * cols + col;
                visited[flat / 64] &= !(1u64 << (flat % 64));
                stack.pop();
                if !path_dirs.is_empty() {
                    path_dirs.pop();
                }
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Backtracked from ({row},{col})."),
                );
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Rat-in-maze complete: {} total path(s) found.", results.len()),
        );
        results
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "rat_in_maze", description = "Use this for solving rat in maze problems. Trigger Keywords: rat_in_maze, rat in maze, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

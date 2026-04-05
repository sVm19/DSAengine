use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Island Counter
/// CATEGORY: graphs
/// DESCRIPTION: Counts the number of distinct islands in a binary grid using
///              iterative flood-fill (BFS) that sinks each island as it is discovered.
pub struct IslandCounter;

impl Complexity for IslandCounter {
    fn name(&self) -> &'static str {
        "Island Counter (BFS Flood-Fill)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(R * C) — Each cell is enqueued and dequeued at most once across all flood-fills."
    }

    fn space_complexity(&self) -> &'static str {
        "O(min(R, C)) — BFS queue holds at most the diagonal of the grid at peak."
    }

    fn description(&self) -> &'static str {
        "Scans for unvisited '1' cells; each find triggers a BFS that sinks all connected '1's, incrementing the island count."
    }
}

impl IslandCounter {
    /// Counts islands in a binary char grid: '1' = land, '0' = water.
    /// Mutates the grid in-place by marking visited cells as '0'.
    pub fn solve(grid: &mut Vec<Vec<u8>>) -> usize {
        let rows = grid.len();
        if rows == 0 { return 0; }
        let cols = grid[0].len();

        let mut count = 0usize;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Island-counter BFS on {rows}×{cols} grid."),
        );

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == b'1' {
                    count += 1;
                    Self::bfs_sink(grid, r, c, rows, cols);
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Island #{count} discovered starting at ({r},{c})."),
                    );
                }
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Total islands: {count}."),
        );
        count
    }

    fn bfs_sink(grid: &mut Vec<Vec<u8>>, sr: usize, sc: usize, rows: usize, cols: usize) {
        let mut queue = std::collections::VecDeque::new();
        grid[sr][sc] = b'0';
        queue.push_back((sr, sc));

        const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some((r, c)) = queue.pop_front() {
            for (dr, dc) in DIRS {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if grid[nr][nc] == b'1' {
                        grid[nr][nc] = b'0';
                        queue.push_back((nr, nc));
                    }
                }
            }
        }
    }

    /// Non-mutating version — counts islands without modifying the input.
    pub fn count_readonly(grid: &[&[u8]]) -> usize {
        let rows = grid.len();
        if rows == 0 { return 0; }
        let cols = grid[0].len();

        let mut visited = vec![vec![false; cols]; rows];
        let mut count = 0usize;
        const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for start_r in 0..rows {
            for start_c in 0..cols {
                if grid[start_r][start_c] == b'1' && !visited[start_r][start_c] {
                    count += 1;
                    visited[start_r][start_c] = true;
                    let mut queue = std::collections::VecDeque::new();
                    queue.push_back((start_r, start_c));
                    while let Some((r, c)) = queue.pop_front() {
                        for (dr, dc) in DIRS {
                            let nr = r as i32 + dr;
                            let nc = c as i32 + dc;
                            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                                let (nr, nc) = (nr as usize, nc as usize);
                                if grid[nr][nc] == b'1' && !visited[nr][nc] {
                                    visited[nr][nc] = true;
                                    queue.push_back((nr, nc));
                                }
                            }
                        }
                    }
                }
            }
        }

        AgentLogger::log(AgentFeedback::Success, format!("Read-only island count: {count}."));
        count
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "island_counter", description = "Use this for solving island counter problems. Trigger Keywords: graph, island_counter, shortest path, traversal. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

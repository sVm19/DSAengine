use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::VecDeque;

/// SKILL: Rotting Oranges
/// CATEGORY: graphs
/// DESCRIPTION: Finds the minimum minutes until all oranges rot using multi-source BFS,
///              or returns -1 if any fresh orange remains unreachable.
pub struct RottingOranges;

impl Complexity for RottingOranges {
    fn name(&self) -> &'static str {
        "Rotting Oranges (Multi-Source BFS)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(R * C) — Every cell is enqueued at most once across the entire multi-source BFS."
    }

    fn space_complexity(&self) -> &'static str {
        "O(R * C) — BFS queue in the worst case holds all rotten cells simultaneously."
    }

    fn description(&self) -> &'static str {
        "Seeds the BFS queue with ALL initially rotten oranges simultaneously; each BFS level equals one minute; fresh oranges unreachable after BFS ends signal -1."
    }
}

impl RottingOranges {
    /// Returns the minimum number of minutes for all reachable fresh oranges to rot.
    ///
    /// Grid cell values: 0=empty, 1=fresh, 2=rotten.
    /// Returns -1 if any fresh orange cannot be reached.
    pub fn solve(grid: &mut Vec<Vec<u8>>) -> i32 {
        let rows = grid.len();
        if rows == 0 { return 0; }
        let cols = grid[0].len();

        let mut queue = VecDeque::new();
        let mut fresh = 0usize;

        // Seed all initially rotten cells and count fresh ones.
        for r in 0..rows {
            for c in 0..cols {
                match grid[r][c] {
                    2 => queue.push_back((r, c, 0u32)),
                    1 => fresh += 1,
                    _ => {}
                }
            }
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Rotting-oranges BFS on {rows}×{cols}: {} initially rotten, {fresh} fresh.",
                queue.len()
            ),
        );

        if fresh == 0 {
            AgentLogger::log(AgentFeedback::Success, "No fresh oranges; answer is 0 minutes.");
            return 0;
        }

        const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut minutes = 0u32;

        while let Some((r, c, minute)) = queue.pop_front() {
            for (dr, dc) in DIRS {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if grid[nr][nc] == 1 {
                        grid[nr][nc] = 2; // Rot it.
                        fresh -= 1;
                        minutes = minutes.max(minute + 1);
                        queue.push_back((nr, nc, minute + 1));
                        AgentLogger::log(
                            AgentFeedback::Step,
                            format!(
                                "Orange at ({nr},{nc}) rotted at minute {}; {fresh} fresh remaining.",
                                minute + 1
                            ),
                        );
                    }
                }
            }
        }

        if fresh > 0 {
            AgentLogger::log(
                AgentFeedback::Warning,
                format!("{fresh} fresh orange(s) unreachable; returning -1."),
            );
            return -1;
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("All oranges rotted in {minutes} minute(s)."),
        );
        minutes as i32
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "rotting_oranges", description = "Use this for solving rotting oranges problems. Trigger Keywords: graph, rotting_oranges, shortest path, traversal. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

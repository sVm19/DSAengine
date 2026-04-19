use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::VecDeque;

/// SKILL: Max Area Island
/// CATEGORY: graphs
/// DESCRIPTION: Finds the area of the largest island in a binary grid using
///              iterative BFS — each island's area is the number of '1' cells connected to it.
pub struct MaxAreaIsland;

impl Complexity for MaxAreaIsland {
    fn name(&self) -> &'static str {
        "Max Area Island (BFS Area Measurement)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(R * C) — Every cell is visited at most once; visited flag prevents re-queuing."
    }

    fn space_complexity(&self) -> &'static str {
        "O(R * C) — A visited boolean grid; BFS queue peaks at O(min(R,C)) in the worst case."
    }

    fn description(&self) -> &'static str {
        "Scans every unvisited land cell; BFS counts connected cells, tracking the running maximum area."
    }
}

const DIRS_4: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl MaxAreaIsland {
    /// Returns the area of the largest island in a binary grid ('1'=land, '0'=water).
    pub fn solve(grid: &[&[u8]]) -> usize {
        let rows = grid.len();
        if rows == 0 {
            return 0;
        }
        let cols = grid[0].len();

        let mut visited = vec![vec![false; cols]; rows];
        let mut max_area = 0usize;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Max-area-island BFS on {rows}×{cols} grid."),
        );

        for start_r in 0..rows {
            for start_c in 0..cols {
                if grid[start_r][start_c] != b'1' || visited[start_r][start_c] {
                    continue;
                }

                // Measure this island via BFS.
                let mut area = 0usize;
                visited[start_r][start_c] = true;
                let mut queue = VecDeque::new();
                queue.push_back((start_r, start_c));

                while let Some((r, c)) = queue.pop_front() {
                    area += 1;
                    for (dr, dc) in DIRS_4 {
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

                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Island at ({start_r},{start_c}) has area={area}; current max={max_area}."
                    ),
                );

                if area > max_area {
                    max_area = area;
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("New maximum island area: {max_area}."),
                    );
                }
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Largest island area: {max_area}."),
        );
        max_area
    }

    /// Returns both the max area and the (row,col) of the top-left cell of the largest island.
    pub fn solve_with_location(grid: &[&[u8]]) -> (usize, Option<(usize, usize)>) {
        let rows = grid.len();
        if rows == 0 {
            return (0, None);
        }
        let cols = grid[0].len();

        let mut visited = vec![vec![false; cols]; rows];
        let mut max_area = 0usize;
        let mut best_origin: Option<(usize, usize)> = None;

        for start_r in 0..rows {
            for start_c in 0..cols {
                if grid[start_r][start_c] != b'1' || visited[start_r][start_c] {
                    continue;
                }

                let mut area = 0usize;
                visited[start_r][start_c] = true;
                let mut queue = VecDeque::new();
                queue.push_back((start_r, start_c));

                while let Some((r, c)) = queue.pop_front() {
                    area += 1;
                    for (dr, dc) in DIRS_4 {
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

                if area > max_area {
                    max_area = area;
                    best_origin = Some((start_r, start_c));
                }
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Largest island: area={max_area}, origin={best_origin:?}."),
        );
        (max_area, best_origin)
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "max_area_island",
    description = "Use this for solving max area island problems. Trigger Keywords: graph, max_area_island, shortest path, traversal. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_max_area_island(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_max_area_island(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        grid: Vec<Vec<u8>>,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'grid' (2D array of 0/1). Optional 'mode': 'max_area' | 'with_location'."
            .to_string(),
    })?;

    let result = match req.mode.as_deref().unwrap_or("") {
        "with_location" => {
            let grid_refs: Vec<&[u8]> = req.grid.iter().map(|r| r.as_slice()).collect();
            let (area, loc) = MaxAreaIsland::solve_with_location(&grid_refs);
            json!({ "mode": "with_location", "max_area": area, "start_cell": loc })
        }
        _ => {
            let grid_refs: Vec<&[u8]> = req.grid.iter().map(|r| r.as_slice()).collect();
            let area = MaxAreaIsland::solve(&grid_refs);
            json!({ "max_area": area })
        }
    };

    let solver = MaxAreaIsland;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["max_area", "with_location"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Max area island completed."))
}

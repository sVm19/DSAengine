use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::VecDeque;

/// SKILL: BFS Generator
/// CATEGORY: graphs
/// DESCRIPTION: Breadth-first traversal of an unweighted graph producing level-by-level
///              node visits, shortest hop-counts from a source, and connected-component discovery.
pub struct BfsGenerator;

impl Complexity for BfsGenerator {
    fn name(&self) -> &'static str {
        "BFS Generator (Level-Order Graph Traversal)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(V + E) — Every vertex is enqueued exactly once; every edge is inspected exactly once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(V) — Queue holds at most V nodes; visited array is size V."
    }

    fn description(&self) -> &'static str {
        "Processes nodes level by level using a VecDeque; guarantees shortest hop-distance from source in unweighted graphs."
    }
}

impl BfsGenerator {
    /// Returns BFS order and shortest hop-distances from `src`.
    /// `adj[u]` = list of neighbour node indices.
    pub fn solve(adj: &[Vec<usize>], src: usize) -> (Vec<usize>, Vec<Option<usize>>) {
        let n = adj.len();
        let mut visited = vec![false; n];
        let mut dist: Vec<Option<usize>> = vec![None; n];
        let mut order = Vec::with_capacity(n);
        let mut queue = VecDeque::new();

        visited[src] = true;
        dist[src] = Some(0);
        queue.push_back(src);

        AgentLogger::log(
            AgentFeedback::Info,
            format!("BFS from src={src} over {n} node(s)."),
        );

        while let Some(u) = queue.pop_front() {
            order.push(u);
            let current_dist = dist[u].unwrap_or(0);

            for &v in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    dist[v] = Some(current_dist + 1);
                    queue.push_back(v);
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!(
                            "Enqueued node {v} at hop-distance {} from src={src}.",
                            current_dist + 1
                        ),
                    );
                }
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("BFS complete: visited {} of {n} node(s).", order.len()),
        );
        (order, dist)
    }

    /// Returns the shortest path (list of node IDs) from `src` to `dst`,
    /// or `None` if no path exists.
    pub fn shortest_path(adj: &[Vec<usize>], src: usize, dst: usize) -> Option<Vec<usize>> {
        let n = adj.len();
        let mut parent: Vec<Option<usize>> = vec![None; n];
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();

        visited[src] = true;
        queue.push_back(src);

        while let Some(u) = queue.pop_front() {
            if u == dst {
                break;
            }
            for &v in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    parent[v] = Some(u);
                    queue.push_back(v);
                }
            }
        }

        if !visited[dst] {
            AgentLogger::log(
                AgentFeedback::Warning,
                format!("No BFS path from {src} to {dst}."),
            );
            return None;
        }

        let mut path = Vec::new();
        let mut cur = dst;
        while let Some(p) = parent[cur] {
            path.push(cur);
            cur = p;
        }
        path.push(src);
        path.reverse();

        AgentLogger::log(
            AgentFeedback::Success,
            format!("BFS path {src}→{dst}: {} hop(s).", path.len() - 1),
        );
        Some(path)
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "bfs_generator",
    description = "Use this for solving bfs generator problems. Trigger Keywords: graph, bfs_generator, shortest path, traversal. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_bfs_generator(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_bfs_generator(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        adj: Vec<Vec<usize>>,
        source: usize,
        destination: Option<usize>,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'adj' (adjacency list) and 'source'. Optional 'destination', 'mode': 'traverse' | 'shortest_path'.".to_string(),
    })?;

    let result = match req.mode.as_deref().unwrap_or("") {
        "shortest_path" => {
            let dst = req.destination.unwrap_or(0);
            let path = BfsGenerator::shortest_path(&req.adj, req.source, dst);
            json!({ "mode": "shortest_path", "path": path, "reachable": path.is_some() })
        }
        _ => {
            let (order, parents) = BfsGenerator::solve(&req.adj, req.source);
            json!({ "mode": "traverse", "bfs_order": order, "parents": parents })
        }
    };

    let solver = BfsGenerator;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["traverse", "shortest_path"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("BFS completed."))
}

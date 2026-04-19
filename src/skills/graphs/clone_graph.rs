use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::HashMap;

/// SKILL: Clone Graph
/// CATEGORY: graphs
/// DESCRIPTION: Deep-copies a graph represented as an adjacency list mapping node IDs
///              to neighbour lists, using iterative BFS to process nodes without recursion.
pub struct CloneGraph;

impl Complexity for CloneGraph {
    fn name(&self) -> &'static str {
        "Clone Graph (BFS Deep Copy)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(V + E) — Each node and each directed edge is visited exactly once during the BFS clone."
    }

    fn space_complexity(&self) -> &'static str {
        "O(V) — The mapping table from original to cloned node ID; BFS queue holds at most V entries."
    }

    fn description(&self) -> &'static str {
        "BFS from node 0; each newly discovered node is immediately registered in a clone map; edges are wired after all neighbours are known."
    }
}

impl CloneGraph {
    /// Clones a graph represented as an adjacency list.
    ///
    /// `adj[u]` = list of node IDs that u connects to.
    /// Returns a new `Vec<Vec<usize>>` that is a structurally identical deep copy.
    pub fn solve(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
        let n = adj.len();
        if n == 0 {
            return Vec::new();
        }

        // The clone preserves the same node count and edge structure.
        let mut cloned: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut visited = vec![false; n];
        let mut queue = std::collections::VecDeque::new();

        visited[0] = true;
        queue.push_back(0usize);

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Cloning graph with {n} node(s) via BFS from node 0."),
        );

        while let Some(u) = queue.pop_front() {
            for &v in &adj[u] {
                // Wire the cloned edge u → v.
                cloned[u].push(v);
                AgentLogger::log(AgentFeedback::Step, format!("Cloned edge {u}→{v}."));

                if !visited[v] {
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }

        let cloned_edges: usize = cloned.iter().map(|nb| nb.len()).sum();
        AgentLogger::log(
            AgentFeedback::Success,
            format!("Clone complete: {n} node(s), {cloned_edges} directed edge(s) copied."),
        );
        cloned
    }

    /// Clones a graph represented as a HashMap (supports non-contiguous IDs).
    pub fn clone_map(adj: &HashMap<usize, Vec<usize>>) -> HashMap<usize, Vec<usize>> {
        if adj.is_empty() {
            return HashMap::new();
        }

        let start = *adj.keys().next().unwrap();
        let mut cloned: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut visited: std::collections::HashSet<usize> = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();

        visited.insert(start);
        queue.push_back(start);

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Cloning HashMap graph with {} node(s) via BFS.", adj.len()),
        );

        while let Some(u) = queue.pop_front() {
            let neighbours = adj.get(&u).map(Vec::as_slice).unwrap_or(&[]);
            let clone_entry = cloned.entry(u).or_default();

            for &v in neighbours {
                clone_entry.push(v);
                if visited.insert(v) {
                    queue.push_back(v);
                }
            }

            AgentLogger::log(
                AgentFeedback::Step,
                format!("Cloned node {u}: {} neighbour(s).", neighbours.len()),
            );
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("HashMap clone complete: {} node(s) copied.", cloned.len()),
        );
        cloned
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "clone_graph",
    description = "Use this for solving clone graph problems. Trigger Keywords: graph, clone_graph, shortest path, traversal. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_clone_graph(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_clone_graph(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        adj: Vec<Vec<usize>>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'adj' (adjacency list as Vec<Vec<usize>>).".to_string(),
    })?;

    let result = {
        let cloned = CloneGraph::solve(&req.adj);
        json!({ "original": req.adj, "cloned": cloned, "nodes": cloned.len() })
    };

    let solver = CloneGraph;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["clone"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Graph cloned."))
}

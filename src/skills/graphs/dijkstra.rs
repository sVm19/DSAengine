use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// SKILL: Dijkstra's Shortest Path
/// CATEGORY: graphs
/// DESCRIPTION: Computes single-source shortest paths from a source node in a
///              weighted graph with non-negative edge weights using a min-heap.
pub struct Dijkstra;

impl crate::utils::complexity::Complexity for Dijkstra {
    fn name(&self) -> &'static str {
        "Dijkstra's Shortest Path (Min-Heap)"
    }

    fn time_complexity(&self) -> &'static str {
        "O((V + E) log V) — Each vertex is settled once; each edge relaxation pushes to a binary heap."
    }

    fn space_complexity(&self) -> &'static str {
        "O(V + E) — Distance array of V entries; heap holds at most E stale entries."
    }

    fn description(&self) -> &'static str {
        "Greedily settles the nearest unsettled vertex by popping from a min-heap and relaxing its outgoing edges."
    }
}

impl Dijkstra {
    /// Returns shortest distances from `src` to every reachable node.
    ///
    /// `adj[u]` = list of `(v, weight)` edges from node u.
    /// Returns a map: node → shortest distance from `src`.
    pub fn solve(adj: &[Vec<(usize, u64)>], src: usize) -> Vec<u64> {
        let n = adj.len();
        let inf = u64::MAX;
        let mut dist = vec![inf; n];
        dist[src] = 0;

        // Min-heap: (distance, node)
        let mut heap: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
        heap.push(Reverse((0, src)));

        while let Some(Reverse((d, u))) = heap.pop() {
            // Stale entry — this node was already settled at a shorter distance.
            if d > dist[u] {
                continue;
            }

            for &(v, w) in &adj[u] {
                let candidate = d.saturating_add(w);
                if candidate < dist[v] {
                    dist[v] = candidate;
                    heap.push(Reverse((candidate, v)));
                }
            }
        }

        dist
    }

    /// Returns the shortest path from `src` to `dst`, or `None` if unreachable.
    /// Includes node traceback using a `parent` array.
    pub fn shortest_path(
        adj: &[Vec<(usize, u64)>],
        src: usize,
        dst: usize,
    ) -> Option<(u64, Vec<usize>)> {
        let n = adj.len();
        let inf = u64::MAX;
        let mut dist = vec![inf; n];
        let mut parent: Vec<Option<usize>> = vec![None; n];
        dist[src] = 0;

        let mut heap: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
        heap.push(Reverse((0, src)));

        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] { continue; }
            if u == dst { break; } // Early exit when destination settled.
            for &(v, w) in &adj[u] {
                let candidate = d.saturating_add(w);
                if candidate < dist[v] {
                    dist[v] = candidate;
                    parent[v] = Some(u);
                    heap.push(Reverse((candidate, v)));
                }
            }
        }

        if dist[dst] == inf {
            return None;
        }

        // Reconstruct path via parent chain.
        let mut path = Vec::new();
        let mut cur = dst;
        while let Some(p) = parent[cur] {
            path.push(cur);
            cur = p;
        }
        path.push(src);
        path.reverse();

        Some((dist[dst], path))
    }

    /// Builds an adjacency list from an edge list `(u, v, weight)`.
    pub fn build_adj(num_nodes: usize, edges: &[(usize, usize, u64)]) -> Vec<Vec<(usize, u64)>> {
        let mut adj = vec![Vec::new(); num_nodes];
        for &(u, v, w) in edges {
            adj[u].push((v, w));
            adj[v].push((u, w)); // undirected
        }
        adj
    }
}


// --- AXUM WEB BRIDGE ---

#[macros::mcp_tool(name = "graphs.dijkstra", description = "Use this for solving dijkstra problems. Trigger Keywords: graph, dijkstra, shortest path, traversal. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_dijkstra(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

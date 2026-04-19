use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use serde_json::{json, Value};

/// SKILL: Bellman-Ford
/// CATEGORY: graphs
/// DESCRIPTION: Computes single-source shortest paths in graphs that may contain
///              negative-weight edges, and detects negative-weight cycles.
pub struct BellmanFord;

impl Complexity for BellmanFord {
    fn name(&self) -> &'static str {
        "Bellman-Ford (Negative-Weight SSSP)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(V * E) — V-1 relaxation passes over all E edges; one extra pass detects negative cycles."
    }

    fn space_complexity(&self) -> &'static str {
        "O(V) — Distance and parent arrays; edge list is read-only."
    }

    fn description(&self) -> &'static str {
        "Relaxes all edges V-1 times; any further improvement on the V-th pass reveals a negative-weight cycle reachable from the source."
    }
}

/// An edge in the graph: (from, to, weight).
pub type Edge = (usize, usize, i64);

impl BellmanFord {
    /// Returns shortest distances from `src`, or `None` if a negative cycle is reachable.
    pub fn solve(edges: &[Edge], num_nodes: usize, src: usize) -> Option<Vec<i64>> {
        let inf = i64::MAX / 2;
        let mut dist = vec![inf; num_nodes];
        if src < num_nodes {
            dist[src] = 0;
        } else {
            return Some(dist); // Source out of bounds
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Bellman-Ford: {num_nodes} node(s), {} edge(s), src={src}.",
                edges.len()
            ),
        );

        // V-1 relaxation rounds.
        for _round in 1..num_nodes {
            let mut updated = false;
            for &(u, v, w) in edges {
                if u < num_nodes && v < num_nodes && dist[u] < inf {
                    let candidate = dist[u] + w;
                    if candidate < dist[v] {
                        dist[v] = candidate;
                        updated = true;
                    }
                }
            }
            if !updated {
                break;
            }
        }

        // V-th pass: detect negative cycles.
        for &(u, v, w) in edges {
            if u < num_nodes && v < num_nodes && dist[u] < inf && dist[u] + w < dist[v] {
                AgentLogger::log(
                    AgentFeedback::Warning,
                    format!("Negative-weight cycle detected via edge {u}→{v}."),
                );
                return None;
            }
        }

        Some(dist)
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};

#[macros::mcp_tool(
    name = "bellman_ford",
    description = "Use this for solving bellman ford problems. Trigger Keywords: graph, bellman_ford, shortest path, traversal. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_bellman_ford(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_bellman_ford(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        edges: Vec<(usize, usize, i64)>,
        num_nodes: usize,
        source: usize,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'edges' as [(from, to, weight)], 'num_nodes', 'source'.".to_string(),
    })?;

    let result = {
        let result = BellmanFord::solve(&req.edges, req.num_nodes, req.source);
        match result {
            Some(dists) => {
                json!({ "source": req.source, "distances": dists, "has_negative_cycle": false })
            }
            None => json!({ "source": req.source, "has_negative_cycle": true }),
        }
    };

    let solver = BellmanFord;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["shortest_paths"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Bellman-Ford completed."))
}

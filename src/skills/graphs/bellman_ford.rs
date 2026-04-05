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
use axum::{Json, response::IntoResponse, http::StatusCode};

#[macros::mcp_tool(name = "bellman_ford", description = "Use this for solving bellman ford problems. Trigger Keywords: graph, bellman_ford, shortest path, traversal. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

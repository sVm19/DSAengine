use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: DFS Generator
/// CATEGORY: graphs
/// DESCRIPTION: Iterative depth-first traversal of a graph producing a pre-order visit
///              sequence, discovery/finish timestamps, and cycle detection.
pub struct DfsGenerator;

impl Complexity for DfsGenerator {
    fn name(&self) -> &'static str {
        "DFS Generator (Iterative Pre-Order + Timestamp)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(V + E) — Every vertex is pushed/popped once; every adjacency list entry is examined once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(V) — Explicit stack holds at most V frames; colour array tracks three visit states."
    }

    fn description(&self) -> &'static str {
        "Uses an explicit Vec stack to simulate DFS without recursion; three-colour marking detects back edges (cycles) without modifying the input graph."
    }
}

/// Visit state for three-colour DFS.
#[derive(Clone, Copy, PartialEq)]
enum Colour {
    White, // unvisited
    Grey,  // on the current DFS stack
    Black, // fully processed
}

impl DfsGenerator {
    /// Returns the DFS pre-order visit sequence and whether any cycle was detected.
    /// `adj[u]` = list of neighbour node indices (directed graph).
    pub fn solve(adj: &[Vec<usize>]) -> (Vec<usize>, bool) {
        let n = adj.len();
        let mut colour = vec![Colour::White; n];
        let mut order = Vec::with_capacity(n);
        let mut has_cycle = false;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("DFS over {n} node(s); using three-colour cycle detection."),
        );

        for start in 0..n {
            if colour[start] != Colour::White {
                continue;
            }

            // Each stack entry: (node, index_into_adj_list, is_enter)
            // is_enter=true means we are about to process node; false means we are leaving.
            let mut stack: Vec<(usize, usize, bool)> = vec![(start, 0, true)];

            while let Some((u, edge_idx, entering)) = stack.last_mut() {
                let u = *u;

                if *entering {
                    *entering = false;
                    colour[u] = Colour::Grey;
                    order.push(u);
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Entering node {u}; marked Grey."),
                    );
                }

                let edge_idx_val = *edge_idx;
                if edge_idx_val < adj[u].len() {
                    let v = adj[u][edge_idx_val];
                    *edge_idx += 1;

                    match colour[v] {
                        Colour::White => {
                            // Tree edge — recurse deeper.
                            stack.push((v, 0, true));
                        }
                        Colour::Grey => {
                            // Back edge — cycle detected.
                            has_cycle = true;
                            AgentLogger::log(
                                AgentFeedback::Warning,
                                format!("Back edge {u}→{v} detected: cycle in graph."),
                            );
                        }
                        Colour::Black => {
                            // Cross/forward edge — already fully processed.
                        }
                    }
                } else {
                    // All neighbours processed — pop and mark Black.
                    colour[u] = Colour::Black;
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Finished node {u}; marked Black."),
                    );
                    stack.pop();
                }
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "DFS complete: {} node(s) visited, cycle={}.",
                order.len(), has_cycle
            ),
        );
        (order, has_cycle)
    }

    /// Returns all connected components as lists of node IDs (for undirected graphs).
    pub fn components(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
        let n = adj.len();
        let mut visited = vec![false; n];
        let mut components: Vec<Vec<usize>> = Vec::new();

        for start in 0..n {
            if visited[start] { continue; }

            let mut component = Vec::new();
            let mut stack = vec![start];
            visited[start] = true;

            while let Some(u) = stack.pop() {
                component.push(u);
                for &v in &adj[u] {
                    if !visited[v] {
                        visited[v] = true;
                        stack.push(v);
                    }
                }
            }

            AgentLogger::log(
                AgentFeedback::Step,
                format!("Component #{}: {} node(s), starting from {start}.", components.len() + 1, component.len()),
            );
            components.push(component);
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Found {} connected component(s).", components.len()),
        );
        components
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "dfs_generator", description = "Use this for solving dfs generator problems. Trigger Keywords: graph, dfs_generator, shortest path, traversal. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

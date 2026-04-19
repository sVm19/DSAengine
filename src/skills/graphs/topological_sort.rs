use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Topological Sort
/// CATEGORY: graphs
/// DESCRIPTION: Produces a topological ordering of a DAG nodes using
///              Kahn's algorithm (iterative BFS on in-degree), which also detects cycles.
pub struct TopologicalSort;

impl Complexity for TopologicalSort {
    fn name(&self) -> &'static str {
        "Topological Sort (Kahn's BFS / In-Degree)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(V + E) — Computing in-degrees is O(E); each node and edge is processed once in the BFS queue."
    }

    fn space_complexity(&self) -> &'static str {
        "O(V) — In-degree array and queue hold at most V entries."
    }

    fn description(&self) -> &'static str {
        "Initialises a queue with all zero-in-degree nodes; processes each, reducing neighbour in-degrees and enqueuing any that reach zero. A cycle exists if the output contains fewer than V nodes."
    }
}

impl TopologicalSort {
    /// Returns a topological ordering of nodes `0..num_nodes`, or `None` if a cycle exists.
    ///
    /// `adj[u]` = directed edges from u.
    pub fn solve(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
        let n = adj.len();
        let mut in_degree = vec![0usize; n];

        for u in 0..n {
            for &v in &adj[u] {
                in_degree[v] += 1;
            }
        }

        // Enqueue all nodes with zero in-degree.
        let mut queue: std::collections::VecDeque<usize> =
            (0..n).filter(|&u| in_degree[u] == 0).collect();

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Kahn's topological sort: {n} node(s); {} zero-in-degree source(s).",
                queue.len()
            ),
        );

        let mut order = Vec::with_capacity(n);

        while let Some(u) = queue.pop_front() {
            order.push(u);
            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Settled node {u}; reducing in-degrees of {} neighbour(s).",
                    adj[u].len()
                ),
            );

            for &v in &adj[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push_back(v);
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Node {v} reached zero in-degree; enqueued."),
                    );
                }
            }
        }

        if order.len() < n {
            AgentLogger::log(
                AgentFeedback::Warning,
                format!(
                    "Cycle detected: only {}/{n} node(s) settled before queue emptied.",
                    order.len()
                ),
            );
            return None;
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Topological order: {:?}.", order),
        );
        Some(order)
    }

    /// Returns all valid topological orderings count (for small V ≤ 20).
    pub fn count_orderings(adj: &[Vec<usize>]) -> u64 {
        // Use the same Kahn structure but enumerate freely with bitmask DP.
        let n = adj.len();
        if n > 20 {
            return 0;
        } // Guard for exponential enumeration.

        let mut in_degree = vec![0usize; n];
        for u in 0..n {
            for &v in &adj[u] {
                in_degree[v] += 1;
            }
        }

        // Recursive count via simulation — bounded at n=20.
        fn count_rec(adj: &[Vec<usize>], in_deg: &mut Vec<usize>, remaining: usize) -> u64 {
            if remaining == 0 {
                return 1;
            }
            let mut total = 0u64;
            for u in 0..adj.len() {
                if in_deg[u] == 0 {
                    // Temporarily remove u.
                    in_deg[u] = usize::MAX;
                    for &v in &adj[u] {
                        in_deg[v] -= 1;
                    }
                    total += count_rec(adj, in_deg, remaining - 1);
                    // Restore u.
                    in_deg[u] = 0;
                    for &v in &adj[u] {
                        in_deg[v] += 1;
                    }
                }
            }
            total
        }

        count_rec(adj, &mut in_degree, n)
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "topological_sort",
    description = "Use this for solving topological sort problems. Trigger Keywords: graph, topological_sort, shortest path, traversal. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_topological_sort(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_topological_sort(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        adj: Vec<Vec<usize>>,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'adj' (adjacency list of DAG). Optional 'mode': 'sort' | 'count'."
            .to_string(),
    })?;

    let result = match req.mode.as_deref().unwrap_or("") {
        "count" => {
            let count = TopologicalSort::count_orderings(&req.adj);
            json!({ "mode": "count", "total_orderings": count })
        }
        _ => {
            let order = TopologicalSort::solve(&req.adj);
            json!({ "mode": "sort", "has_valid_order": order.is_some(), "order": order })
        }
    };

    let solver = TopologicalSort;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["sort", "count"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Topological sort completed."))
}

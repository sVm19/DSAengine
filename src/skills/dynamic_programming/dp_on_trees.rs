use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: DP on Trees
/// CATEGORY: dynamic-programming
/// DESCRIPTION: Computes the diameter and maximum-weight path of a tree represented
///              as an adjacency list, using iterative post-order DFS with an explicit stack.
pub struct DpOnTrees;

impl Complexity for DpOnTrees {
    fn name(&self) -> &'static str {
        "DP on Trees (Post-Order Iterative DFS)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(V + E) — Each node and each edge is visited exactly once in the post-order traversal."
    }

    fn space_complexity(&self) -> &'static str {
        "O(V) — The dp/parent arrays hold one entry per node; the explicit stack depth is bounded by tree height."
    }

    fn description(&self) -> &'static str {
        "Uses an iterative post-order DFS; after all children of a node are processed, dp[node] aggregates child results (max depth / path weight) in O(degree) per node."
    }
}

/// A simple tree represented as a list of (parent, child, weight) edges.
/// Nodes are 0-indexed integers.
pub struct WeightedTree {
    /// Adjacency list: adj[u] = Vec<(v, weight)>
    adj: Vec<Vec<(usize, i64)>>,
    pub num_nodes: usize,
}

impl WeightedTree {
    /// Builds a tree from `num_nodes` nodes and a list of undirected edges.
    pub fn new(num_nodes: usize, edges: &[(usize, usize, i64)]) -> Self {
        let mut adj = vec![Vec::new(); num_nodes];
        for &(u, v, w) in edges {
            adj[u].push((v, w));
            adj[v].push((u, w));
        }
        WeightedTree { adj, num_nodes }
    }
}

impl DpOnTrees {
    /// Returns the weighted diameter (longest root-to-leaf path sum through any pair of nodes)
    /// of the tree rooted conceptually at node 0.
    ///
    /// Uses iterative post-order DFS: for each node, dp[node] = max weighted depth from node downward.
    pub fn solve(tree: &WeightedTree) -> i64 {
        let n = tree.num_nodes;
        if n == 0 { return 0; }
        if n == 1 { return 0; }

        let mut depth = vec![0i64; n];   // max weighted depth from each node downward
        let mut parent = vec![usize::MAX; n];
        let mut order: Vec<usize> = Vec::with_capacity(n); // post-order sequence

        // Iterative DFS to compute post-order.
        let mut stack = vec![(0usize, usize::MAX)]; // (node, parent)
        while let Some((node, par)) = stack.pop() {
            parent[node] = par;
            order.push(node);
            for &(child, _) in &tree.adj[node] {
                if child != par {
                    stack.push((child, node));
                }
            }
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Tree DP: {n} node(s); traversing post-order (reversed DFS)."),
        );

        // Process in reverse order (children before parents = post-order).
        let mut diameter = 0i64;

        for &node in order.iter().rev() {
            let mut best1 = 0i64; // heaviest child arm
            let mut best2 = 0i64; // second-heaviest child arm

            for &(child, weight) in &tree.adj[node] {
                if child == parent[node] { continue; }

                let arm = depth[child] + weight;
                if arm >= best1 {
                    best2 = best1;
                    best1 = arm;
                } else if arm > best2 {
                    best2 = arm;
                }
            }

            depth[node] = best1;
            let local_diam = best1 + best2;
            if local_diam > diameter {
                diameter = local_diam;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Node {node}: updated diameter to {diameter} (arms {best1} + {best2})."
                    ),
                );
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Tree weighted diameter: {diameter}."),
        );
        diameter
    }

    /// Returns the maximum sum of any root-to-leaf path in the given rooted tree.
    pub fn max_root_to_leaf(tree: &WeightedTree, root: usize) -> i64 {
        let n = tree.num_nodes;
        let mut depth = vec![0i64; n];
        let mut parent = vec![usize::MAX; n];
        let mut order: Vec<usize> = Vec::with_capacity(n);

        let mut stack = vec![(root, usize::MAX)];
        while let Some((node, par)) = stack.pop() {
            parent[node] = par;
            order.push(node);
            for &(child, _) in &tree.adj[node] {
                if child != par { stack.push((child, node)); }
            }
        }

        for &node in order.iter().rev() {
            for &(child, weight) in &tree.adj[node] {
                if child == parent[node] { continue; }
                let arm = depth[child] + weight;
                if arm > depth[node] {
                    depth[node] = arm;
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Node {node}: max-depth updated to {} via child {child}.", depth[node]),
                    );
                }
            }
        }

        let result = depth[root];
        AgentLogger::log(AgentFeedback::Success, format!("Max root-to-leaf path: {result}."));
        result
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "dp_on_trees", description = "Use this for solving dp on trees problems. Trigger Keywords: dp_on_trees, dp on trees, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

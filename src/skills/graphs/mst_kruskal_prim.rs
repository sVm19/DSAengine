use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use serde_json::{json, Value};

/// SKILL: MST Kruskal & Prim
/// CATEGORY: graphs
/// DESCRIPTION: Builds a Minimum Spanning Tree using either Kruskal's algorithm
///              (sorted edges + Union-Find) or Prim's (min-heap greedy vertex picks).
pub struct MstKruskalPrim;

impl Complexity for MstKruskalPrim {
    fn name(&self) -> &'static str {
        "MST — Kruskal (Union-Find) / Prim (Min-Heap)"
    }

    fn time_complexity(&self) -> &'static str {
        "Kruskal O(E log E) — dominated by edge sort. Prim O((V + E) log V) — with binary heap."
    }

    fn space_complexity(&self) -> &'static str {
        "Kruskal O(V) — Union-Find arrays. Prim O(V + E) — adjacency list + heap."
    }

    fn description(&self) -> &'static str {
        "Kruskal adds the globally cheapest safe edge that doesn't form a cycle (Union-Find). Prim grows the MST from a seed by always pulling the cheapest edge crossing the cut."
    }
}

/// Union-Find (path-compressed, union by rank).
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect::<Vec<_>>(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]]; // Path halving.
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return false;
        }
        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less => self.parent[ra] = rb,
            std::cmp::Ordering::Greater => self.parent[rb] = ra,
            std::cmp::Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
        true
    }
}

impl MstKruskalPrim {
    /// Kruskal's MST — returns the MST edges and total weight.
    pub fn kruskal(
        num_nodes: usize,
        edges: &mut [(usize, usize, i64)],
    ) -> Option<(Vec<(usize, usize, i64)>, i64)> {
        edges.sort_unstable_by_key(|&(_, _, w)| w);

        let mut uf = UnionFind::new(num_nodes);
        let mut mst: Vec<(usize, usize, i64)> = Vec::with_capacity(num_nodes.saturating_sub(1));
        let mut total_weight = 0i64;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Kruskal MST: {num_nodes} node(s), {} edge(s).", edges.len()),
        );

        for &(u, v, w) in edges.iter() {
            if u < num_nodes && v < num_nodes && uf.union(u, v) {
                mst.push((u, v, w));
                total_weight += w;
                if mst.len() == num_nodes - 1 {
                    break;
                }
            }
        }

        if num_nodes > 1 && mst.len() < num_nodes - 1 {
            return None;
        }

        Some((mst, total_weight))
    }
}


// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};

#[macros::mcp_tool(name = "mst_kruskal_prim", description = "Use this for solving mst kruskal prim problems. Trigger Keywords: graph, mst_kruskal_prim, shortest path, traversal. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

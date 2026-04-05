use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Fenwick Tree (Binary Indexed Tree)
/// CATEGORY: trees-advanced
/// DESCRIPTION: Provides O(log N) prefix sum updates and queries securely using
///              bitwise operations (`i & -i`) without requiring deep structural trees.
pub struct FenwickTree;

impl Complexity for FenwickTree {
    fn name(&self) -> &'static str {
        "Fenwick Tree (Iterative Bitwise Array)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(log N) — Updates and queries traverse the tree depth iteratively via bit flips."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Operates entirely atop a 1-indexed backing array."
    }

    fn description(&self) -> &'static str {
        "Leverages `i & -i` to iteratively climb indices updating affected segments, or descending indices accumulating partial sums continuously. Perfectly zero-recursive."
    }
}

pub struct BinaryIndexedTree {
    tree: Vec<i32>,
}

impl BinaryIndexedTree {
    pub fn new(size: usize) -> Self {
        AgentLogger::log(AgentFeedback::Info, format!("Initialized Fenwick Tree with capacity {size}."));
        Self { tree: vec![0; size + 1] }
    }

    /// Iteratively adds `delta` to index `i` (1-based internally).
    pub fn add(&mut self, mut i: usize, delta: i32) {
        let n = self.tree.len();
        AgentLogger::log(AgentFeedback::Step, format!("Fenwick: Adding {delta} at index {i}."));
        
        while i < n {
            self.tree[i] += delta;
            let step = (i as isize & -(i as isize)) as usize;
            i += step;
        }
    }

    /// Iteratively queries the prefix sum up to index `i` (1-based internally).
    pub fn query(&self, mut i: usize) -> i32 {
        let mut sum = 0;
        
        while i > 0 {
            sum += self.tree[i];
            let step = (i as isize & -(i as isize)) as usize;
            i -= step;
        }

        sum
    }

    /// Queries the range sum from `l` to `r`.
    pub fn range_query(&self, l: usize, r: usize) -> i32 {
        let val = self.query(r) - self.query(l - 1);
        AgentLogger::log(AgentFeedback::Step, format!("Fenwick Range Query [{l}, {r}] = {val}."));
        val
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "fenwick_tree", description = "Use this for solving fenwick tree problems. Trigger Keywords: fenwick_tree, fenwick tree, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

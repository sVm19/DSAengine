use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Union Find
/// CATEGORY: advanced-topics
/// DESCRIPTION: Implements disjoint-set union with path compression and union by size.
pub struct UnionFind;
pub struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl Complexity for UnionFind {
    fn name(&self) -> &'static str {
        "Union Find"
    }

    fn time_complexity(&self) -> &'static str {
        "O((u + q) * alpha(n)) across batches, with near-constant amortized union/find."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) - Stores one parent and one component-size entry per node."
    }

    fn description(&self) -> &'static str {
        "Maintains dynamic connectivity by compressing find paths and attaching smaller trees beneath larger ones."
    }
}

impl UnionFind {
    pub fn solve(
        node_count: usize,
        unions: &[(usize, usize)],
        queries: &[(usize, usize)],
    ) -> Vec<bool> {
        let mut sets = Self::build(node_count);
        for &(left, right) in unions {
            sets.union(left, right);
        }
        queries
            .iter()
            .map(|&(left, right)| sets.connected(left, right).unwrap_or(false))
            .collect()
    }

    pub fn build(node_count: usize) -> DisjointSet {
        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Initializing disjoint-set forest with {} nodes.",
                node_count
            ),
        );

        DisjointSet {
            parent: (0..node_count).collect(),
            size: vec![1; node_count],
            components: node_count,
        }
    }
}

impl DisjointSet {
    pub fn find(&mut self, node: usize) -> Option<usize> {
        if node >= self.parent.len() {
            return None;
        }

        let mut root = node;
        while self.parent[root] != root {
            root = self.parent[root];
        }

        let mut current = node;
        while self.parent[current] != current {
            let next = self.parent[current];
            self.parent[current] = root;
            current = next;
        }

        Some(root)
    }

    pub fn union(&mut self, left: usize, right: usize) -> bool {
        let Some(mut root_left) = self.find(left) else {
            return false;
        };
        let Some(mut root_right) = self.find(right) else {
            return false;
        };

        if root_left == root_right {
            AgentLogger::log(
                AgentFeedback::Info,
                format!(
                    "Nodes {} and {} are already in the same component.",
                    left, right
                ),
            );
            return false;
        }

        if self.size[root_left] < self.size[root_right] {
            std::mem::swap(&mut root_left, &mut root_right);
        }

        self.parent[root_right] = root_left;
        self.size[root_left] += self.size[root_right];
        self.components -= 1;

        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Merged root {} into root {}. Component count is now {}.",
                root_right, root_left, self.components
            ),
        );
        true
    }

    pub fn connected(&mut self, left: usize, right: usize) -> Option<bool> {
        Some(self.find(left).unwrap() == self.find(right)?)
    }

    pub fn component_size(&mut self, node: usize) -> Option<usize> {
        let root = self.find(node).unwrap();
        Some(self.size[root])
    }

    pub fn components(&self) -> usize {
        self.components
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "union_find",
    description = "Use this for solving union find problems. Trigger Keywords: union_find, union find, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_union_find(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_union_find(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        node_count: usize,
        unions: Vec<(usize, usize)>,
        query: Option<(usize, usize)>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'node_count' and 'unions' as [(a, b)]. Optional 'query': (a, b) to check connectivity.".to_string(),
    })?;

    let result = {
        let mut ds = UnionFind::build(req.node_count);
        for (a, b) in &req.unions {
            ds.union(*a, *b);
        }
        let connected = req.query.map(|(a, b)| ds.connected(a, b));
        json!({ "node_count": req.node_count, "unions_processed": req.unions.len(), "components": ds.components(), "query_result": connected })
    };

    let solver = UnionFind;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["process_unions"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Union-Find completed."))
}

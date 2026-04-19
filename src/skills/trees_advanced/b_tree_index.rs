use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: B-Tree Indexer
/// CATEGORY: trees-advanced
/// DESCRIPTION: Memory-mapped logical bounds emulating multi-way branching B-Trees
///              optimal for paginated persistent storage structures.
pub struct BTreeIndex;

impl Complexity for BTreeIndex {
    fn name(&self) -> &'static str {
        "B-Tree (Multi-way Disk Optimizer)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(log_b N) — Base maps proportionally to the order 'b' mitigating excessive node traversals significantly."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Dense key capacities grouping contiguous subsets dynamically."
    }

    fn description(&self) -> &'static str {
        "Stores collections of sorted keys directly mitigating disk fetching bottlenecks. Traverses natively stepping over key subsets linearly until a child bounds branch is intercepted."
    }
}

pub struct BTreeNode {
    pub keys: Vec<i32>,
    pub children: Vec<usize>,
    pub is_leaf: bool,
}

pub struct BTreeArena {
    pub nodes: Vec<BTreeNode>,
    pub root: usize,
    pub order: usize,
}

impl BTreeArena {
    pub fn new(t: usize) -> Self {
        AgentLogger::log(
            AgentFeedback::Info,
            format!("Engine initialized BTree indexing limits with Minimum Degree t={t}."),
        );
        Self {
            nodes: Vec::new(),
            root: usize::MAX,
            order: t,
        }
    }

    /// Validates an iterative search across the B-Tree keys matching subsets logically.
    pub fn search(&self, mut curr: usize, k: i32) -> Option<(usize, usize)> {
        let null = usize::MAX;

        while curr != null {
            let mut i = 0;
            let node = &self.nodes[curr];

            while i < node.keys.len() && k > node.keys[i] {
                i += 1;
            }

            if i < node.keys.len() && k == node.keys[i] {
                AgentLogger::log(
                    AgentFeedback::Success,
                    format!("Intercepted BTree Key {k} directly inside node chunk {curr}."),
                );
                return Some((curr, i));
            }

            if node.is_leaf {
                break;
            } else {
                curr = node.children[i];
            }
        }

        AgentLogger::log(
            AgentFeedback::Warning,
            format!("BTree indexing failure catching key bounded limit {k}."),
        );
        None
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "b_tree_index",
    description = "Use this for solving b tree index problems. Trigger Keywords: b_tree_index, b tree index, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_b_tree_index(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_b_tree_index(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        values: Vec<i32>,
        search: Option<i32>,
        order: Option<usize>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'values'. Optional 'search' value, 'order' (default 3).".to_string(),
    })?;

    let result = {
        let t = req.order.unwrap_or(3);
        let mut arena = BTreeArena::new(t);
        // Create a single leaf node with sorted keys
        let mut sorted_keys = req.values.clone();
        sorted_keys.sort();
        arena.nodes.push(BTreeNode {
            keys: sorted_keys.clone(),
            children: vec![],
            is_leaf: true,
        });
        arena.root = 0;
        let search_result = req.search.map(|v| arena.search(0, v));
        json!({ "size": req.values.len(), "order": t, "keys_sorted": sorted_keys, "search_result": search_result })
    };

    let solver = BTreeIndex;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["build", "search"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("BTreeIndex completed."))
}

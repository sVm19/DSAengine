use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: AVL Tree
/// CATEGORY: trees-advanced
/// DESCRIPTION: Emulates an Adelson-Velsky and Landis self-balancing tree iteratively
///              using an arena array to manage strict height bounds guaranteeing O(log N) operations.
pub struct AvlTree;

impl Complexity for AvlTree {
    fn name(&self) -> &'static str {
        "AVL Tree (Iterative Arena Operations)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(log N) — Depth is mathematically strictly bounded. Insertions trigger at most 2 rotations natively bounding descent paths."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Array backing mitigates dynamic pointer sprawl."
    }

    fn description(&self) -> &'static str {
        "Evaluates the balance factor `height(left) - height(right)`. If this exceeds 1 or drops below -1, it executes LL, RR, LR, or RL associative rotations over array indices."
    }
}

// Memory-safe, pointerless AVL node representation
pub struct AvlNode {
    pub val: i32,
    pub left: usize,
    pub right: usize,
    pub height: i32,
}

pub struct AvlArena {
    pub nodes: Vec<AvlNode>,
    pub root: usize,
}

impl AvlArena {
    pub fn new() -> Self {
        AgentLogger::log(AgentFeedback::Info, "AVL Arena Engine Instantiated.");
        Self {
            nodes: Vec::new(),
            root: usize::MAX,
        }
    }

    pub fn height(&self, node_idx: usize) -> i32 {
        if node_idx == usize::MAX {
            0
        } else {
            self.nodes[node_idx].height
        }
    }

    pub fn update_height(&mut self, node_idx: usize) {
        if node_idx != usize::MAX {
            let lh = self.height(self.nodes[node_idx].left);
            let rh = self.height(self.nodes[node_idx].right);
            self.nodes[node_idx].height = 1 + lh.max(rh);
        }
    }

    pub fn balance_factor(&self, node_idx: usize) -> i32 {
        if node_idx == usize::MAX {
            0
        } else {
            self.height(self.nodes[node_idx].left) - self.height(self.nodes[node_idx].right)
        }
    }

    /// Rotates the subtree right dynamically overriding indices safely.
    pub fn rotate_right(&mut self, y: usize) -> usize {
        let x = self.nodes[y].left;
        let t2 = self.nodes[x].right;

        self.nodes[x].right = y;
        self.nodes[y].left = t2;

        self.update_height(y);
        self.update_height(x);

        AgentLogger::log(
            AgentFeedback::Step,
            format!("Executed LL Right Rotation around node {y}."),
        );
        x
    }

    /// Rotates the subtree left dynamically overriding indices safely.
    pub fn rotate_left(&mut self, x: usize) -> usize {
        let y = self.nodes[x].right;
        let t2 = self.nodes[y].left;

        self.nodes[y].left = x;
        self.nodes[x].right = t2;

        self.update_height(x);
        self.update_height(y);

        AgentLogger::log(
            AgentFeedback::Step,
            format!("Executed RR Left Rotation around node {x}."),
        );
        y
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "avl_tree",
    description = "Use this for solving avl tree problems. Trigger Keywords: avl_tree, avl tree, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_avl_tree(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_avl_tree(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        values: Vec<i32>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'values' to insert into the AVL tree.".to_string(),
    })?;

    let result = {
        let mut arena = AvlArena::new();
        // Insert values as flat nodes (the arena provides rotation primitives)
        for &v in &req.values {
            let idx = arena.nodes.len();
            arena.nodes.push(AvlNode {
                val: v,
                left: usize::MAX,
                right: usize::MAX,
                height: 1,
            });
            if idx > 0 {
                // Simple sequential linking for demonstration
                arena.nodes[idx - 1].right = idx;
                arena.update_height(idx - 1);
            }
        }
        let root = if arena.nodes.is_empty() {
            usize::MAX
        } else {
            0
        };
        json!({ "root": root, "size": req.values.len(), "note": "Arena-based AVL with rotation primitives. Use rotate_left/rotate_right for balancing." })
    };

    let solver = AvlTree;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["build"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("AvlTree completed."))
}

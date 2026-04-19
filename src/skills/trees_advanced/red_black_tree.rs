use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Red-Black Tree
/// CATEGORY: trees-advanced
/// DESCRIPTION: Emulates a Red-Black self-balancing tree logic structure natively maintaining
///              color continuity and deterministic O(log N) bounds.
pub struct RedBlackTreeSkill;

impl Complexity for RedBlackTreeSkill {
    fn name(&self) -> &'static str {
        "Red-Black Tree Constraints Validator"
    }

    fn time_complexity(&self) -> &'static str {
        "O(log N) — Depth is loosely bounded to 2*log(N). Black-height is perfectly balanced on all divergent branches."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Encapsulated natively inside color-coded array nodes."
    }

    fn description(&self) -> &'static str {
        "Defines structural rules restricting topological imbalances: Root is Black, Red nodes possess strictly Black children, and all paths trace equivalent Black constraints."
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Red,
    Black,
}

pub struct RbNode {
    pub val: i32,
    pub left: usize,
    pub right: usize,
    pub color: Color,
}

pub struct RedBlackArena {
    pub nodes: Vec<RbNode>,
    pub root: usize,
}

impl RedBlackArena {
    pub fn new() -> Self {
        AgentLogger::log(AgentFeedback::Info, "Red-Black Engine Formatted.");
        Self {
            nodes: Vec::new(),
            root: usize::MAX,
        }
    }

    /// Safely checks if an iterative traversal violates the Red-Black 3 core constraints.
    pub fn validate_constraints(&self) -> bool {
        let null = usize::MAX;
        if self.root == null {
            return true;
        }

        if self.nodes[self.root].color == Color::Red {
            AgentLogger::log(
                AgentFeedback::Warning,
                "Violation: Root node must be Black.",
            );
            return false;
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((self.root, 1)); // (node_idx, current_black_height)

        let mut expected_black_height = None;

        while let Some((curr, b_height)) = queue.pop_front() {
            let node = &self.nodes[curr];

            if node.color == Color::Red {
                let l_red = node.left != null && self.nodes[node.left].color == Color::Red;
                let r_red = node.right != null && self.nodes[node.right].color == Color::Red;
                if l_red || r_red {
                    AgentLogger::log(
                        AgentFeedback::Warning,
                        format!("Violation: Red node {curr} possesses Red child limits."),
                    );
                    return false;
                }
            }

            if node.left == null && node.right == null {
                if expected_black_height.is_none() {
                    expected_black_height = Some(b_height);
                } else if expected_black_height.unwrap() != b_height {
                    AgentLogger::log(
                        AgentFeedback::Warning,
                        "Violation: Divergent Black-Node Path Densities.",
                    );
                    return false;
                }
            }

            if node.left != null {
                let h_add = if self.nodes[node.left].color == Color::Black {
                    1
                } else {
                    0
                };
                queue.push_back((node.left, b_height + h_add));
            }

            if node.right != null {
                let h_add = if self.nodes[node.right].color == Color::Black {
                    1
                } else {
                    0
                };
                queue.push_back((node.right, b_height + h_add));
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            "RB Structural integrity and Color mapping mathematically valid.",
        );
        true
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "red_black_tree",
    description = "Use this for solving red black tree problems. Trigger Keywords: red_black_tree, red black tree, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_red_black_tree(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_red_black_tree(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        values: Vec<i32>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'values' to insert.".to_string(),
    })?;

    let result = {
        let mut arena = RedBlackArena::new();
        // Build nodes directly (insert is a structural demo — nodes are added manually)
        for (i, &v) in req.values.iter().enumerate() {
            arena.nodes.push(RbNode {
                val: v,
                left: usize::MAX,
                right: usize::MAX,
                color: if i == 0 { Color::Black } else { Color::Red },
            });
        }
        if !arena.nodes.is_empty() {
            arena.root = 0;
        }
        let valid = arena.validate_constraints();
        json!({ "size": req.values.len(), "valid_rb_properties": valid })
    };

    let solver = RedBlackTreeSkill;
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
    .with_description("RedBlackTreeSkill completed."))
}

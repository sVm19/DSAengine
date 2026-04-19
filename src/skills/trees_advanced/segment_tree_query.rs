use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Segment Tree Range Query
/// CATEGORY: trees-advanced
/// DESCRIPTION: Executes a segment tree range query matching constraints continuously
///              without recurring down structural depths, evaluating boundary subsets internally.
pub struct SegmentTreeQuery;

impl Complexity for SegmentTreeQuery {
    fn name(&self) -> &'static str {
        "Segment Tree Query (Iterative Half-Open Bounds)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(log N) — Steps progressively inwards adjusting values seamlessly without touching intermediate nodes."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Resolves directly inside existing 2N allocated arrays."
    }

    fn description(&self) -> &'static str {
        "Validates the half open window `[L, R)`. Checks if L is an odd right child. If true, adds it and shifts inward. Checks if R is odd, subtracts, adds, loops logarithmically."
    }
}

impl SegmentTreeQuery {
    /// Iteratively resolves sums for window `[l, r)` using the 2N segment array.
    pub fn solve(tree: &[i32], mut l: usize, mut r: usize) -> i32 {
        let n = tree.len() / 2;
        l += n;
        r += n;

        let mut sum = 0;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Executing SegTree interval resolution bounded by [L={l}, R={r})."),
        );

        while l < r {
            if l & 1 == 1 {
                // l is odd -> right child, so its parent's range isn't fully covered
                sum += tree[l];
                l += 1;
            }
            if r & 1 == 1 {
                // r is odd -> right child, but since it's half-open [l, r) we shift left
                r -= 1;
                sum += tree[r];
            }
            l >>= 1;
            r >>= 1;
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("SegTree interval sum natively resolved to {sum}."),
        );
        sum
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct SegmentTreeQueryRequest {
    pub tree: Vec<i32>,
    pub left: usize,
    pub right: usize,
}

#[macros::mcp_tool(
    name = "trees_advanced.segment_tree_query",
    description = "Use this for solving segment tree query problems. Trigger Keywords: segment_tree_query, segment tree query, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_segment_tree_query(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_segment_tree_query(payload: Value) -> DsaResult<ResultBox> {
    let req: SegmentTreeQueryRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid SegmentTreeQueryRequest: {e}"),
            hint: "Provide 'tree', 'left', and 'right' for range [left, right).".to_string(),
        })?;

    if req.tree.is_empty() || req.tree.len() % 2 != 0 {
        return Err(DsaError::InvalidInput {
            message: "tree must be a non-empty 2N segment-tree array.".to_string(),
            hint: "Build a tree first via segment_tree_builder and pass that output.".to_string(),
        });
    }
    let n = req.tree.len() / 2;
    if req.left > req.right || req.right > n {
        return Err(DsaError::InvalidInput {
            message: format!(
                "Invalid range [{}, {}) for base size {}.",
                req.left, req.right, n
            ),
            hint: "Use 0 <= left <= right <= N where N is original array length.".to_string(),
        });
    }

    let sum = SegmentTreeQuery::solve(&req.tree, req.left, req.right);

    let solver = SegmentTreeQuery;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });

    Ok(ResultBox::success(json!({
        "sum": sum
    }))
    .with_complexity(complexity)
    .with_description("Segment tree range query completed."))
}

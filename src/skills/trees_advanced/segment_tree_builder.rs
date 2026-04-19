use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Segment Tree Builder
/// CATEGORY: trees-advanced
/// DESCRIPTION: Constructs a segment tree iteratively over a precisely constrained `2 * N`
///              array block, avoiding completely the O(N) overhead of pointer nodes.
pub struct SegmentTreeBuilder;

impl Complexity for SegmentTreeBuilder {
    fn name(&self) -> &'static str {
        "Segment Tree Construction (Bottom-Up 2N Array)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Initializes leaves iteratively, then walks backwards N-1 to 1 setting parental states."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Exactly 2*N sized allocation replacing massive class-based pointer networks completely."
    }

    fn description(&self) -> &'static str {
        "Builds atop the exact mathematical layout `parent = i >> 1`, `left = i << 1`, `right = (i << 1) | 1`. Leaves map into indices [N, 2N - 1]."
    }
}

impl SegmentTreeBuilder {
    /// Builds a summation SegmentTree array from an input slice `arr`.
    pub fn solve(arr: &[i32]) -> Vec<i32> {
        let n = arr.len();
        if n == 0 {
            return Vec::new();
        }

        let mut tree = vec![0; 2 * n];
        AgentLogger::log(
            AgentFeedback::Info,
            format!("Building 2N Segment Tree dynamically. Array size = {n}."),
        );

        // Inject leaves starting continuously at index N
        for i in 0..n {
            tree[n + i] = arr[i];
        }

        // Fold parents sequentially downward backwards preventing any recursion
        for i in (1..n).rev() {
            tree[i] = tree[i << 1] + tree[(i << 1) | 1];
        }

        AgentLogger::log(
            AgentFeedback::Success,
            "Segment Tree natively built avoiding structs.",
        );
        tree
    }

    /// Iteratively adjusts a specific index `idx` to value `val` strictly climbing up.
    pub fn point_update(tree: &mut [i32], mut idx: usize, val: i32) {
        let n = tree.len() / 2;
        idx += n;

        AgentLogger::log(
            AgentFeedback::Step,
            format!("SegTree point update at index {idx} to {val}."),
        );
        tree[idx] = val;

        while idx > 1 {
            idx >>= 1; // move to parent
            tree[idx] = tree[idx << 1] + tree[(idx << 1) | 1];
        }
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct SegmentTreePointUpdate {
    pub index: usize,
    pub value: i32,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct SegmentTreeBuilderRequest {
    pub nums: Vec<i32>,
    pub updates: Option<Vec<SegmentTreePointUpdate>>,
}

#[macros::mcp_tool(
    name = "trees_advanced.segment_tree_builder",
    description = "Use this for solving segment tree builder problems. Trigger Keywords: segment_tree_builder, segment tree builder, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_segment_tree_builder(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_segment_tree_builder(payload: Value) -> DsaResult<ResultBox> {
    let req: SegmentTreeBuilderRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid SegmentTreeBuilderRequest: {e}"),
            hint: "Provide 'nums' and optional 'updates' entries with {index, value}.".to_string(),
        })?;

    if req.nums.is_empty() {
        return Err(DsaError::InvalidInput {
            message: "nums cannot be empty.".to_string(),
            hint: "Provide at least one integer in 'nums'.".to_string(),
        });
    }

    let mut tree = SegmentTreeBuilder::solve(&req.nums);
    if let Some(updates) = req.updates {
        let n = req.nums.len();
        for update in updates {
            if update.index >= n {
                return Err(DsaError::IndexOutOfBounds {
                    index: update.index,
                    bounds: n,
                    context: "segment tree point update index".to_string(),
                });
            }
            SegmentTreeBuilder::point_update(&mut tree, update.index, update.value);
        }
    }

    let solver = SegmentTreeBuilder;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });

    Ok(ResultBox::success(json!({
        "tree": tree
    }))
    .with_complexity(complexity)
    .with_description("Segment tree built successfully."))
}

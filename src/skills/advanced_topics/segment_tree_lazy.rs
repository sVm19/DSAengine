use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Segment Tree Lazy
/// CATEGORY: advanced-topics
/// DESCRIPTION: Implements an iterative lazy-propagation segment tree for range-add and range-sum workloads.
pub struct SegmentTreeLazy;
pub struct LazyRangeSumTree {
    len: usize,
    tree: Vec<i64>,
    lazy: Vec<i64>,
}

impl Complexity for SegmentTreeLazy {
    fn name(&self) -> &'static str {
        "Segment Tree Lazy"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) build and O(log n) amortized per range update/query with lazy propagation."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) - Stores tree aggregates and deferred lazy deltas."
    }

    fn description(&self) -> &'static str {
        "Defers range updates until needed, which keeps repeated interval operations logarithmic."
    }
}

impl SegmentTreeLazy {
    pub fn solve(values: &[i64]) -> Option<LazyRangeSumTree> {
        Self::build(values)
    }

    pub fn build(values: &[i64]) -> Option<LazyRangeSumTree> {
        if values.is_empty() {
            return None;
        }

        let mut tree = LazyRangeSumTree {
            len: values.len(),
            tree: vec![0; values.len() * 4 + 4],
            lazy: vec![0; values.len() * 4 + 4],
        };
        tree.build_iterative(values);

        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Lazy segment tree built for {} source values.",
                values.len()
            ),
        );
        Some(tree)
    }
}

impl LazyRangeSumTree {
    pub fn range_add(&mut self, query_left: usize, query_right: usize, delta: i64) -> bool {
        if query_left > query_right || query_right >= self.len {
            return false;
        }

        let mut stack = vec![(1usize, 0usize, self.len - 1, false)];
        while let Some((node, left, right, revisit)) = stack.pop() {
            if query_right < left || right < query_left {
                continue;
            }

            if revisit {
                self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
                continue;
            }

            if query_left <= left && right <= query_right {
                self.apply(node, left, right, delta);
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Applied lazy delta {} to fully covered segment [{}..={}].",
                        delta, left, right
                    ),
                );
                continue;
            }

            self.push(node, left, right);
            stack.push((node, left, right, true));
            let mid = left + (right - left) / 2;
            if query_left <= mid {
                stack.push((node * 2, left, mid, false));
            }
            if query_right > mid {
                stack.push((node * 2 + 1, mid + 1, right, false));
            }
        }

        true
    }

    pub fn range_sum(&mut self, query_left: usize, query_right: usize) -> Option<i64> {
        if query_left > query_right || query_right >= self.len {
            return None;
        }

        let mut total = 0i64;
        let mut stack = vec![(1usize, 0usize, self.len - 1)];
        while let Some((node, left, right)) = stack.pop() {
            if query_right < left || right < query_left {
                continue;
            }

            if query_left <= left && right <= query_right {
                total += self.tree[node];
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Consumed segment [{}..={}] with aggregate {}.",
                        left, right, self.tree[node]
                    ),
                );
                continue;
            }

            self.push(node, left, right);
            let mid = left + (right - left) / 2;
            if query_left <= mid {
                stack.push((node * 2, left, mid));
            }
            if query_right > mid {
                stack.push((node * 2 + 1, mid + 1, right));
            }
        }

        Some(total)
    }

    fn build_iterative(&mut self, values: &[i64]) {
        let mut stack = vec![(1usize, 0usize, self.len - 1, false)];
        while let Some((node, left, right, revisit)) = stack.pop() {
            if left == right {
                self.tree[node] = values[left];
                continue;
            }

            if revisit {
                self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
                continue;
            }

            stack.push((node, left, right, true));
            let mid = left + (right - left) / 2;
            stack.push((node * 2 + 1, mid + 1, right, false));
            stack.push((node * 2, left, mid, false));
        }
    }

    fn apply(&mut self, node: usize, left: usize, right: usize, delta: i64) {
        self.tree[node] += delta * (right - left + 1) as i64;
        self.lazy[node] += delta;
    }

    fn push(&mut self, node: usize, left: usize, right: usize) {
        let pending = self.lazy[node];
        if pending == 0 || left == right {
            return;
        }

        let mid = left + (right - left) / 2;
        self.apply(node * 2, left, mid, pending);
        self.apply(node * 2 + 1, mid + 1, right, pending);
        self.lazy[node] = 0;

        AgentLogger::log(
            AgentFeedback::Step,
            format!(
                "Pushed deferred delta {} from segment [{}..={}] into its children.",
                pending, left, right
            ),
        );
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "segment_tree_lazy",
    description = "Use this for solving segment tree lazy problems. Trigger Keywords: segment_tree_lazy, segment tree lazy, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_segment_tree_lazy(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_segment_tree_lazy(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        values: Vec<i64>,
        operations: Option<Vec<(String, usize, usize, Option<i64>)>>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'values' (array). Optional 'operations' as [(op, left, right, delta)]. op= 'sum' or 'add'.".to_string(),
    })?;

    let result = {
        let mut tree = SegmentTreeLazy::build(&req.values).ok_or(DsaError::InvalidInput {
            message: "Cannot build tree from empty values.".to_string(),
            hint: "Provide at least one value.".to_string(),
        })?;
        let mut results = vec![];
        if let Some(ops) = &req.operations {
            for (op, l, r, delta) in ops {
                match op.as_str() {
                    "add" => {
                        tree.range_add(*l, *r, delta.unwrap_or(0));
                        results.push(json!({"op":"add","l":l,"r":r,"delta":delta}));
                    }
                    _ => {
                        let s = tree.range_sum(*l, *r);
                        results.push(json!({"op":"sum","l":l,"r":r,"result":s}));
                    }
                }
            }
        }
        json!({ "size": req.values.len(), "operations_results": results })
    };

    let solver = SegmentTreeLazy;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["build_and_query"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Segment tree with lazy propagation completed."))
}

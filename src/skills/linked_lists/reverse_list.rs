use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Reverse List
/// CATEGORY: linked-lists
/// DESCRIPTION: Reverses a singly linked list represented as an index-based arena
///              using a three-pointer iterative walk — no recursion, no allocation.
///
/// Arena layout: `nodes[i] = (next_index, value)`.
/// `usize::MAX` is the null sentinel.
pub struct ReverseList;

impl Complexity for ReverseList {
    fn name(&self) -> &'static str {
        "Reverse List (Three-Pointer Iterative)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) — Single pass: each node visited exactly once to rewire its `next` pointer."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Only three index variables (prev, curr, next); arena mutated in place."
    }

    fn description(&self) -> &'static str {
        "Iterates with prev=NULL, curr=head; at each step saves curr.next, points curr.next←prev, advances prev=curr, curr=saved_next."
    }
}

impl ReverseList {
    /// Reverses the list in the arena starting at `head`.
    ///
    /// `nodes` — mutable arena of `(next_idx, value)` pairs; `usize::MAX` = null.
    /// Returns the new head index.
    pub fn solve(nodes: &mut Vec<(usize, i32)>, head: usize) -> usize {
        let null = usize::MAX;
        let n = Self::length(nodes, head);

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Reversing list of {n} node(s); head={head}."),
        );

        let mut prev = null;
        let mut curr = head;
        let mut step = 0usize;

        while curr != null {
            let next = nodes[curr].0; // save curr.next
            nodes[curr].0 = prev; // rewire curr.next ← prev

            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Step {step}: node[{curr}].next {prev_str} → {next_str}; advancing prev={curr}.",
                    prev_str = if prev == null { "NULL".to_string() } else { prev.to_string() },
                    next_str = if prev == null { "NULL".to_string() } else { prev.to_string() },
                ),
            );

            prev = curr;
            curr = next;
            step += 1;
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Reversal complete after {step} step(s); new head={prev}."),
        );
        prev // new head
    }

    /// Reverses only the sub-list from node index `left_idx` to `right_idx` (0-based positions
    /// counted from `head`; both inclusive).
    pub fn reverse_between(
        nodes: &mut Vec<(usize, i32)>,
        head: usize,
        left_pos: usize,
        right_pos: usize,
    ) -> usize {
        let _null = usize::MAX;
        if left_pos == right_pos {
            return head;
        }

        // Find the node just before `left_pos` (use a dummy sentinel index trick).
        let dummy_val = 0i32;
        nodes.push((head, dummy_val));
        let dummy = nodes.len() - 1;

        let mut pre = dummy;
        for _ in 0..left_pos {
            pre = nodes[pre].0;
        }
        // `pre` now points to the node before position `left_pos`.
        let curr = nodes[pre].0;

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Partial reverse: positions {left_pos}..={right_pos}; pre={pre}, start={curr}."
            ),
        );

        // Reverse `(right_pos - left_pos)` times.
        for step in 0..(right_pos - left_pos) {
            let next = nodes[curr].0;
            nodes[curr].0 = nodes[next].0;
            nodes[next].0 = nodes[pre].0;
            nodes[pre].0 = next;
            AgentLogger::log(
                AgentFeedback::Step,
                format!("Partial-reverse step {step}: moved node {next} → front of sub-list."),
            );
        }

        let new_head = nodes[dummy].0;
        nodes.pop(); // Remove dummy sentinel.
        AgentLogger::log(
            AgentFeedback::Success,
            format!("Partial reversal done; new head={new_head}."),
        );
        new_head
    }

    /// Returns the length of the list starting at `head`.
    pub fn length(nodes: &[(usize, i32)], mut head: usize) -> usize {
        let mut count = 0;
        while head != usize::MAX {
            head = nodes[head].0;
            count += 1;
        }
        count
    }

    /// Collects list values from `head` into a Vec for easy inspection.
    pub fn to_vec(nodes: &[(usize, i32)], mut head: usize) -> Vec<i32> {
        let mut out = Vec::new();
        while head != usize::MAX {
            out.push(nodes[head].1);
            head = nodes[head].0;
        }
        out
    }

    /// Builds an arena from a slice of values (left-to-right order).
    pub fn from_slice(values: &[i32]) -> (Vec<(usize, i32)>, usize) {
        if values.is_empty() {
            return (Vec::new(), usize::MAX);
        }
        let n = values.len();
        let nodes: Vec<(usize, i32)> = (0..n)
            .map(|i| (if i + 1 < n { i + 1 } else { usize::MAX }, values[i]))
            .collect();
        (nodes, 0)
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "reverse_list",
    description = "Use this for solving reverse list problems. Trigger Keywords: reverse_list, reverse list, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_reverse_list(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_reverse_list(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        values: Option<Vec<i32>>,
        nodes: Option<Vec<(usize, i32)>>,
        head: Option<usize>,
        left: Option<usize>,
        right: Option<usize>,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'values' or 'nodes'+'head'. Mode: 'full'|'partial'.".to_string(),
    })?;

    let result = {
        if let Some(values) = &req.values {
            let (mut n, h) = ReverseList::from_slice(values);
            let nh = ReverseList::solve(&mut n, h);
            let rv = ReverseList::to_vec(&n, nh);
            json!({ "mode":"full", "original":values, "reversed":rv })
        } else if let Some(nodes) = &req.nodes {
            let mut nc = nodes.clone();
            let h = req.head.unwrap_or(0);
            match req.mode.as_deref().unwrap_or("full") {
                "partial" => {
                    let l = req.left.unwrap_or(0);
                    let r = req.right.unwrap_or(0);
                    let nh = ReverseList::reverse_between(&mut nc, h, l, r);
                    json!({"mode":"partial","head":nh})
                }
                _ => {
                    let nh = ReverseList::solve(&mut nc, h);
                    json!({"mode":"full","head":nh})
                }
            }
        } else {
            return Err(DsaError::InvalidInput {
                message: "Missing input.".to_string(),
                hint: "Provide values or nodes.".to_string(),
            });
        }
    };

    let solver = ReverseList;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["full", "partial", "from_slice"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("ReverseList completed."))
}

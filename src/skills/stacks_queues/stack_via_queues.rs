use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::VecDeque;

/// SKILL: Implement Stack using Queues
/// CATEGORY: stacks-queues
/// DESCRIPTION: Emulates a LIFO stack using two FIFO queues (or practically one).
pub struct StackViaQueuesSkill;

impl Complexity for StackViaQueuesSkill {
    fn name(&self) -> &'static str {
        "Stack via Queue (O(N) Push, O(1) Pop)"
    }

    fn time_complexity(&self) -> &'static str {
        "Push: O(N) — Must cycle the queue N times to rotate the new element to the front. Pop: O(1)."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Internal queue size."
    }

    fn description(&self) -> &'static str {
        "Uses a single `VecDeque` acting strictly as a queue (push_back, pop_front). To simulate a stack push, it enqueues the element, then dequeues and re-enqueues all N prior elements to force the new one to the front."
    }
}

pub struct MyStack {
    q: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        AgentLogger::log(AgentFeedback::Info, "Initialized MyStack via Queue.");
        Self { q: VecDeque::new() }
    }

    pub fn push(&mut self, x: i32) {
        self.q.push_back(x);
        let n = self.q.len();

        // Cycle the queue to bring the newly pushed element to the "front"
        for _ in 1..n {
            if let Some(front) = self.q.pop_front() {
                self.q.push_back(front);
            }
        }

        AgentLogger::log(
            AgentFeedback::Step,
            format!("Stack pushed {x} (Queue rotated {n} times)."),
        );
    }

    pub fn pop(&mut self) -> Option<i32> {
        let val = self.q.pop_front();
        if let Some(v) = val {
            AgentLogger::log(AgentFeedback::Step, format!("Stack popped {v}."));
        }
        val
    }

    pub fn top(&self) -> Option<i32> {
        self.q.front().copied()
    }

    pub fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StackViaQueuesOperation {
    Push { value: i32 },
    Pop,
    Top,
    Empty,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StackViaQueuesRequest {
    pub operations: Vec<StackViaQueuesOperation>,
}

#[macros::mcp_tool(
    name = "stacks_queues.stack_via_queues",
    description = "Use this for solving stack via queues problems. Trigger Keywords: stack_via_queues, stack via queues, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_stack_via_queues(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_stack_via_queues(payload: Value) -> DsaResult<ResultBox> {
    let req: StackViaQueuesRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid StackViaQueuesRequest: {e}"),
            hint: "Provide 'operations' with entries like {\"type\":\"push\",\"value\":3}."
                .to_string(),
        })?;

    let mut stack = MyStack::new();
    let mut trace = Vec::new();

    for op in req.operations {
        match op {
            StackViaQueuesOperation::Push { value } => {
                stack.push(value);
                trace.push(json!({ "op": "push", "value": value }));
            }
            StackViaQueuesOperation::Pop => {
                trace.push(json!({ "op": "pop", "value": stack.pop() }));
            }
            StackViaQueuesOperation::Top => {
                trace.push(json!({ "op": "top", "value": stack.top() }));
            }
            StackViaQueuesOperation::Empty => {
                trace.push(json!({ "op": "empty", "value": stack.empty() }));
            }
        }
    }

    let solver = StackViaQueuesSkill;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });

    Ok(ResultBox::success(json!({
        "trace": trace
    }))
    .with_complexity(complexity)
    .with_description("Stack-via-queues operations completed."))
}

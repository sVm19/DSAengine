use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Implement Queue using Stacks
/// CATEGORY: stacks-queues
/// DESCRIPTION: Emulates a FIFO queue using two LIFO stacks.
pub struct QueueViaStacksSkill;

impl Complexity for QueueViaStacksSkill {
    fn name(&self) -> &'static str {
        "Queue via Two Stacks (Amortized O(1))"
    }

    fn time_complexity(&self) -> &'static str {
        "Push: O(1). Pop/Peek: Amortized O(1), Worst O(N) when flipping stacks."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — To store the N elements."
    }

    fn description(&self) -> &'static str {
        "Push goes straight to `stack_in`. Pop/Peek pulls from `stack_out`. If `stack_out` is empty, elements are flushed from `in` to `out`, reversing their order to FIFO."
    }
}

pub struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        AgentLogger::log(AgentFeedback::Info, "Initialized MyQueue via Stacks.");
        Self {
            stack_in: Vec::new(),
            stack_out: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.stack_in.push(x);
        AgentLogger::log(AgentFeedback::Step, format!("Queue pushed {x}."));
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.flush();
        let val = self.stack_out.pop();
        if let Some(v) = val {
            AgentLogger::log(AgentFeedback::Step, format!("Queue popped {v}."));
        }
        val
    }

    pub fn peek(&mut self) -> Option<i32> {
        self.flush();
        self.stack_out.last().copied()
    }

    pub fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }

    fn flush(&mut self) {
        if self.stack_out.is_empty() {
            let n = self.stack_in.len();
            if n > 0 {
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Flushing {n} item(s) from stack_in to stack_out."),
                );
                while let Some(v) = self.stack_in.pop() {
                    self.stack_out.push(v);
                }
            }
        }
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum QueueViaStacksOperation {
    Push { value: i32 },
    Pop,
    Peek,
    Empty,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct QueueViaStacksRequest {
    pub operations: Vec<QueueViaStacksOperation>,
}

#[macros::mcp_tool(name = "stacks_queues.queue_via_stacks", description = "Use this for solving queue via stacks problems. Trigger Keywords: queue_via_stacks, queue via stacks, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_queue_via_stacks(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_queue_via_stacks(payload: Value) -> DsaResult<ResultBox> {
    let req: QueueViaStacksRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid QueueViaStacksRequest: {e}"),
            hint: "Provide 'operations' with entries like {\"type\":\"push\",\"value\":3}."
                .to_string(),
        })?;

    let mut queue = MyQueue::new();
    let mut trace = Vec::new();

    for op in req.operations {
        match op {
            QueueViaStacksOperation::Push { value } => {
                queue.push(value);
                trace.push(json!({ "op": "push", "value": value }));
            }
            QueueViaStacksOperation::Pop => {
                trace.push(json!({ "op": "pop", "value": queue.pop() }));
            }
            QueueViaStacksOperation::Peek => {
                trace.push(json!({ "op": "peek", "value": queue.peek() }));
            }
            QueueViaStacksOperation::Empty => {
                trace.push(json!({ "op": "empty", "value": queue.empty() }));
            }
        }
    }

    let solver = QueueViaStacksSkill;
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
    .with_description("Queue-via-stacks operations completed."))
}

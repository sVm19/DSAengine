use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Min Stack
/// CATEGORY: stacks-queues
/// DESCRIPTION: A stack implementation that supports push, pop, top, and retrieving
///              the minimum element in strictly constant O(1) time.
pub struct MinStackSkill;

impl Complexity for MinStackSkill {
    fn name(&self) -> &'static str {
        "Min Stack (Value/Min Pairs)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(1) — Push, pop, top, and get_min operations are all constant time."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Every element stores both its value and the minimum value up to its level."
    }

    fn description(&self) -> &'static str {
        "Stores elements as tuples `(value, current_min)`. Because a stack's state is strictly LIFO, the minimum at depth D never changes regardless of what happens above D."
    }
}

/// The actual engine-compatible struct.
pub struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        AgentLogger::log(AgentFeedback::Info, "Initialized new MinStack.");
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        let current_min = if let Some(top) = self.stack.last() {
            std::cmp::min(val, top.1)
        } else {
            val
        };
        self.stack.push((val, current_min));
        AgentLogger::log(
            AgentFeedback::Step,
            format!("Pushed {val}; current min is {current_min}."),
        );
    }

    pub fn pop(&mut self) -> Option<i32> {
        if let Some((val, _)) = self.stack.pop() {
            AgentLogger::log(AgentFeedback::Step, format!("Popped {val}."));
            Some(val)
        } else {
            None
        }
    }

    pub fn top(&self) -> Option<i32> {
        self.stack.last().map(|&(val, _)| val)
    }

    pub fn get_min(&self) -> Option<i32> {
        self.stack.last().map(|&(_, min_val)| min_val)
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MinStackOperation {
    Push { value: i32 },
    Pop,
    Top,
    GetMin,
    Empty,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct MinStackRequest {
    pub operations: Vec<MinStackOperation>,
}

#[macros::mcp_tool(
    name = "stacks_queues.min_stack",
    description = "Use this for solving min stack problems. Trigger Keywords: min_stack, min stack, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_min_stack(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_min_stack(payload: Value) -> DsaResult<ResultBox> {
    let req: MinStackRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid MinStackRequest: {e}"),
            hint: "Provide 'operations' with entries like {\"type\":\"push\",\"value\":3}."
                .to_string(),
        })?;

    let mut stack = MinStack::new();
    let mut trace = Vec::new();

    for op in req.operations {
        match op {
            MinStackOperation::Push { value } => {
                stack.push(value);
                trace.push(json!({ "op": "push", "value": value }));
            }
            MinStackOperation::Pop => {
                trace.push(json!({ "op": "pop", "value": stack.pop() }));
            }
            MinStackOperation::Top => {
                trace.push(json!({ "op": "top", "value": stack.top() }));
            }
            MinStackOperation::GetMin => {
                trace.push(json!({ "op": "get_min", "value": stack.get_min() }));
            }
            MinStackOperation::Empty => {
                trace.push(json!({ "op": "empty", "value": stack.is_empty() }));
            }
        }
    }

    let solver = MinStackSkill;
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
    .with_description("Min stack operations completed."))
}

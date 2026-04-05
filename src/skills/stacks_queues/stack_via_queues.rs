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
        Self {
            q: VecDeque::new(),
        }
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
        
        AgentLogger::log(AgentFeedback::Step, format!("Stack pushed {x} (Queue rotated {n} times)."));
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
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "stacks_queues.stack_via_queues", description = "Use this for solving stack via queues problems. Trigger Keywords: stack_via_queues, stack via queues, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_stack_via_queues(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Heap Priority Queue
/// CATEGORY: trees-advanced
/// DESCRIPTION: Custom Min-Heap built entirely atop a flat array slice via iterative
///              `sift_up` and `sift_down` bounds.
pub struct HeapPriorityQueue;

impl Complexity for HeapPriorityQueue {
    fn name(&self) -> &'static str {
        "Array Min-Heap (Iterative Sifting)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(log N) — Pushes and Pops strictly climb or descend structural arrays iteratively."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Underlying contiguous contiguous array."
    }

    fn description(&self) -> &'static str {
        "Implicit mathematical tree structure on array: children are `2i+1` and `2i+2`. Uses iterative `swap` chains instead of recursive bubble paths."
    }
}

pub struct MinHeap {
    data: Vec<i32>,
}

impl MinHeap {
    pub fn new() -> Self {
        AgentLogger::log(AgentFeedback::Info, "Initialized new Iterative MinHeap.");
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        self.data.push(val);
        AgentLogger::log(
            AgentFeedback::Step,
            format!("MinHeap pushed {val}. Executing sift_up."),
        );
        self.sift_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let n = self.data.len();
        if n == 0 {
            return None;
        }

        self.data.swap(0, n - 1);
        let val = self.data.pop();

        if self.data.len() > 1 {
            self.sift_down(0);
        }

        AgentLogger::log(
            AgentFeedback::Step,
            format!("MinHeap popped {:?}. Executed sift_down.", val),
        );
        val
    }

    pub fn peek(&self) -> Option<i32> {
        self.data.first().copied()
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.data[idx] >= self.data[parent] {
                break;
            }
            self.data.swap(idx, parent);
            idx = parent;
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        let n = self.data.len();
        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut smallest = idx;

            if left < n && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < n && self.data[right] < self.data[smallest] {
                smallest = right;
            }

            if smallest == idx {
                break;
            }
            self.data.swap(idx, smallest);
            idx = smallest;
        }
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "heap_priority_queue",
    description = "Use this for solving heap priority queue problems. Trigger Keywords: sorting, searching, heap_priority_queue. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_heap_priority_queue(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_heap_priority_queue(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        values: Vec<i32>,
        pop_count: Option<usize>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'values'. Optional 'pop_count' to pop N min elements.".to_string(),
    })?;

    let result = {
        let mut heap = MinHeap::new();
        for &v in &req.values {
            heap.push(v);
        }
        let mut popped = vec![];
        let count = req.pop_count.unwrap_or(0);
        for _ in 0..count {
            if let Some(v) = heap.pop() {
                popped.push(v);
            }
        }
        json!({ "size": req.values.len(), "popped": popped, "peek": heap.peek() })
    };

    let solver = HeapPriorityQueue;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["build", "push", "pop"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("HeapPriorityQueue completed."))
}

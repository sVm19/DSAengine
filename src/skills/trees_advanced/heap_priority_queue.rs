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
        AgentLogger::log(AgentFeedback::Step, format!("MinHeap pushed {val}. Executing sift_up."));
        self.sift_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let n = self.data.len();
        if n == 0 { return None; }
        
        self.data.swap(0, n - 1);
        let val = self.data.pop();
        
        if self.data.len() > 1 {
            self.sift_down(0);
        }

        AgentLogger::log(AgentFeedback::Step, format!("MinHeap popped {:?}. Executed sift_down.", val));
        val
    }

    pub fn peek(&self) -> Option<i32> {
        self.data.first().copied()
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.data[idx] >= self.data[parent] { break; }
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

            if smallest == idx { break; }
            self.data.swap(idx, smallest);
            idx = smallest;
        }
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "heap_priority_queue", description = "Use this for solving heap priority queue problems. Trigger Keywords: sorting, searching, heap_priority_queue. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

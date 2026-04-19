use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// SKILL: Median of Data Stream
/// CATEGORY: trees-advanced
/// DESCRIPTION: Locates streaming medians flawlessly leveraging Two-Heap interactions
///              binding max elements in the lower spectrum, and min elements in the upper spectrum.
pub struct MedianStream;

impl Complexity for MedianStream {
    fn name(&self) -> &'static str {
        "Rolling Median Tracker (Two-Heap Pivot)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(log N) — Inserting takes heap height time limits. Median extraction is explicitly O(1)."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Exact sum arrays."
    }

    fn description(&self) -> &'static str {
        "Uses a MaxHeap for the left (smaller) half, and a MinHeap for the right (larger) half. Balancing strictly to a `|left| - |right| <= 1` tolerance limits ensures the median rests directly at the root."
    }
}

pub struct MedianFinder {
    low_max_heap: BinaryHeap<i32>,
    high_min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        AgentLogger::log(
            AgentFeedback::Info,
            "Initialized new Two-Heap Median Streamer.",
        );
        Self {
            low_max_heap: BinaryHeap::new(),
            high_min_heap: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        // Enqueue into Low normally first
        self.low_max_heap.push(num);

        // Balance values (Max of Low must be <= Min of High)
        let max_of_low = self.low_max_heap.pop().unwrap();
        self.high_min_heap.push(Reverse(max_of_low));

        // Balance scales (Low should be equal to or 1 larger than High)
        if self.high_min_heap.len() > self.low_max_heap.len() {
            let Reverse(min_of_high) = self.high_min_heap.pop().unwrap();
            self.low_max_heap.push(min_of_high);
        }

        AgentLogger::log(
            AgentFeedback::Step,
            format!("Added {num}. Structural parity balanced."),
        );
    }

    pub fn find_median(&self) -> f64 {
        if self.low_max_heap.len() > self.high_min_heap.len() {
            let med = *self.low_max_heap.peek().unwrap() as f64;
            AgentLogger::log(
                AgentFeedback::Step,
                format!("Median resolved via distinct middle: {med}."),
            );
            med
        } else {
            let low = *self.low_max_heap.peek().unwrap() as f64;
            let Reverse(high) = *self.high_min_heap.peek().unwrap();
            let med = (low + high as f64) / 2.0;
            AgentLogger::log(
                AgentFeedback::Step,
                format!("Median resolved via dual aggregate: {med}."),
            );
            med
        }
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "median_stream",
    description = "Use this for solving median stream problems. Trigger Keywords: median_stream, median stream, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_median_stream(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_median_stream(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        values: Vec<i32>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'values' (stream of integers).".to_string(),
    })?;

    let result = {
        let mut finder = MedianFinder::new();
        let mut medians = Vec::new();
        for &v in &req.values {
            finder.add_num(v);
            medians.push(finder.find_median());
        }
        json!({ "medians": medians, "final_median": medians.last() })
    };

    let solver = MedianStream;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["compute"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("MedianStream completed."))
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Sorting Visualizer
/// CATEGORY: sorting-searching
/// DESCRIPTION: Outputs string representations of arrays during search reductions
///              or sort partition steps for agent diagnostics.
pub struct Visualizer;

impl Complexity for Visualizer {
    fn name(&self) -> &'static str {
        "Sorting/Searching Boundary Visualizer"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Iterates the array slice strictly to stringify chunks."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Formatted string lengths."
    }

    fn description(&self) -> &'static str {
        "Provides formatters to display active search intervals within an array, e.g. `[..] <active> [..]`, or to show split partitions visually."
    }
}

impl Visualizer {
    /// Visualizes an active sliding window or a binary search interval boundary.
    /// `left` and `right` represent the `[left, right]` inclusive range that is currently active.
    pub fn view_interval<T: std::fmt::Display>(arr: &[T], left: usize, right: usize) -> String {
        if arr.is_empty() {
            return String::from("[]");
        }
        
        let safe_left = left.min(arr.len());
        let safe_right = right.min(arr.len() - 1);
        
        let mut out = String::new();
        
        if safe_left > 0 {
            out.push_str("...| ");
        }

        for i in safe_left..=safe_right {
            out.push_str(&format!("{} ", arr[i]));
        }

        if safe_right < arr.len() - 1 {
            out.push_str("|...");
        }

        AgentLogger::log(
            AgentFeedback::Step,
            format!("Viewing interval [{safe_left}, {safe_right}] within bounds (len={}).", arr.len()),
        );

        out.trim().to_string()
    }
    
    /// Provides an overview showing frequency distribution of smaller chunks,
    /// ideal for bucket sort/counting sort histogram logs.
    pub fn display_histogram(counts: &[usize], offset: i32) -> String {
        let mut out = String::from("Histogram:\n");
        let max_val = *counts.iter().max().unwrap_or(&0);
        let max_chars = 30; // Max horizontal bar width
        
        for (i, &count) in counts.iter().enumerate() {
            if count == 0 { continue; }
            let bars = if max_val > 0 { (count * max_chars) / max_val } else { 0 };
            let bar_str = "#".repeat(bars.max(1));
            
            out.push_str(&format!("{:>4} | {} ({count})\n", offset + i as i32, bar_str));
        }
        
        out
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum VisualizerRequest {
    Interval {
        nums: Vec<i32>,
        left: usize,
        right: usize,
    },
    Histogram {
        counts: Vec<usize>,
        offset: Option<i32>,
    },
}

#[macros::mcp_tool(name = "sorting_searching.visualizer", description = "Use this for solving visualizer problems. Trigger Keywords: visualizer, visualizer, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_visualizer(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_visualizer(payload: Value) -> DsaResult<ResultBox> {
    let req: VisualizerRequest = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid VisualizerRequest: {e}"),
        hint: "Use mode='interval' with nums/left/right or mode='histogram' with counts."
            .to_string(),
    })?;

    let output = match req {
        VisualizerRequest::Interval { nums, left, right } => {
            if nums.is_empty() {
                return Err(DsaError::InvalidInput {
                    message: "nums cannot be empty for interval mode.".to_string(),
                    hint: "Provide at least one value in 'nums'.".to_string(),
                });
            }
            json!({
                "mode": "interval",
                "view": Visualizer::view_interval(&nums, left, right)
            })
        }
        VisualizerRequest::Histogram { counts, offset } => json!({
            "mode": "histogram",
            "view": Visualizer::display_histogram(&counts, offset.unwrap_or(0))
        }),
    };

    let solver = Visualizer;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });

    Ok(ResultBox::success(output)
        .with_complexity(complexity)
        .with_description("Sorting/search visualizer output generated."))
}

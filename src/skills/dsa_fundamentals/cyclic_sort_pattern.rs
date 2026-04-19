use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Cyclic Sort Pattern
/// CATEGORY: dsa-fundamentals
/// DESCRIPTION: Solves problems where numbers are in a fixed range (1 to n).
pub struct CyclicSort;

impl Complexity for CyclicSort {
    fn name(&self) -> &'static str {
        "Cyclic Sort Pattern"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Every number is moved to its correct index at most once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - In-place sorting without extra arrays."
    }

    fn description(&self) -> &'static str {
        "Ideal for finding missing, duplicated, or corrupted numbers in a 1..N range."
    }
}

impl CyclicSort {
    /// Checks if a number is already at its correct index.
    /// Correct Index = Value - 1 (for 1-based ranges).
    pub fn is_in_correct_pos(val: usize, current_idx: usize) -> bool {
        val == current_idx + 1
    }

    /// Visualizes the "Swap" to place a number in its home.
    pub fn trace_swap(val: i32, from_idx: usize, to_idx: usize) {
        println!(
            "  🔄 Swapping: Value [{}] from Index [{}] to its Home Index [{}]",
            val, from_idx, to_idx
        );
        AgentLogger::log(
            AgentFeedback::Step,
            "Relocating element to mapped position.",
        );
    }

    /// Explains why this is O(n) despite nested loops.
    pub fn explain_complexity_paradox() {
        println!("[EFFICIENCY HINT]:");
        println!("   Although there is a 'while' inside a 'for', each element is swapped");
        println!("   to its correct place exactly once. Total swaps < N.");
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "dsa_fundamentals.cyclic_sort_pattern",
    description = "Use this for solving cyclic sort pattern problems. Trigger Keywords: cyclic_sort_pattern, cyclic sort pattern, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_cyclic_sort_pattern(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CyclicSortPatternRequest {
    pub values: Vec<usize>,
}

async fn handle_cyclic_sort_pattern(payload: Value) -> DsaResult<ResultBox> {
    let req: CyclicSortPatternRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid CyclicSortPatternRequest: {e}"),
            hint: "Provide 'values' as an array of usize values expected to be in the 1..=n range."
                .to_string(),
        })?;

    let mut values = req.values.clone();
    let n = values.len();
    let mut swaps = 0usize;
    let mut i = 0usize;
    while i < n {
        let value = values[i];
        if value == 0 || value > n || CyclicSort::is_in_correct_pos(value, i) {
            i += 1;
            continue;
        }
        let target = value - 1;
        if values[target] == value {
            i += 1;
            continue;
        }
        CyclicSort::trace_swap(value as i32, i, target);
        values.swap(i, target);
        swaps += 1;
    }

    let anomalies: Vec<_> = values
        .iter()
        .enumerate()
        .filter_map(|(idx, &value)| {
            if value != idx + 1 {
                Some(json!({ "index": idx, "expected": idx + 1, "actual": value }))
            } else {
                None
            }
        })
        .collect();
    let solver = CyclicSort;

    Ok(ResultBox::success(json!({
        "original": req.values,
        "reordered": values,
        "swaps": swaps,
        "anomalies": anomalies,
        "is_perfect_1_to_n": anomalies.is_empty()
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Cyclic-sort pattern analysis completed."))
}

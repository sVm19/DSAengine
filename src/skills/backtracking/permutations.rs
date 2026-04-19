use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Permutations
/// CATEGORY: backtracking
/// DESCRIPTION: Generates all permutations of a slice in-place using Heap's iterative algorithm,
///              producing O(n!) outputs with O(n) auxiliary space.
pub struct Permutations;

impl Complexity for Permutations {
    fn name(&self) -> &'static str {
        "Permutations (Heap's Iterative Algorithm)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n! * n) — Exactly n! permutations, each requiring O(n) to snapshot the current slice."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) — One counter array of size n drives the algorithm; mutates the input slice in-place."
    }

    fn description(&self) -> &'static str {
        "Uses Heap's non-recursive algorithm: a counter array selects which element to swap, generating every permutation with exactly n!-1 swaps total."
    }
}

impl Permutations {
    /// Returns every permutation of `elements` without recursion.
    ///
    /// Implements Heap's algorithm (iterative form):
    ///   - A `counter[i]` tracks how many times position i has been processed.
    ///   - When counter[i] < i: swap based on parity (even i → swap [0, i]; odd i → swap [counter[i], i]).
    ///   - When counter[i] >= i: reset counter and move to i+1.
    pub fn solve<T: Clone>(elements: &mut [T]) -> Vec<Vec<T>> {
        let n = elements.len();
        if n == 0 {
            return Vec::new();
        }

        let mut results: Vec<Vec<T>> = Vec::new();
        let mut counter = vec![0usize; n];

        // Emit the first (identity) permutation.
        results.push(elements.to_vec());
        AgentLogger::log(
            AgentFeedback::Info,
            format!("Heap's algorithm starting on {n}-element slice; will emit {n}! permutations."),
        );

        let mut i = 1usize;
        while i < n {
            if counter[i] < i {
                if i % 2 == 0 {
                    elements.swap(0, i);
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Even depth {i}: swapped positions [0, {i}]."),
                    );
                } else {
                    elements.swap(counter[i], i);
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Odd depth {i}: swapped positions [{}, {i}].", counter[i]),
                    );
                }

                results.push(elements.to_vec());
                AgentLogger::log(
                    AgentFeedback::Success,
                    format!("Emitted permutation #{}.", results.len()),
                );

                counter[i] += 1;
                i = 1; // Reset the scan to position 1.
            } else {
                counter[i] = 0;
                i += 1;
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Heap's algorithm complete — generated {} permutation(s).",
                results.len()
            ),
        );
        results
    }

    /// Returns the count of permutations without materializing them: n!
    pub fn count(n: u64) -> u64 {
        (1..=n).product()
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "permutations",
    description = "Use this for solving permutations problems. Trigger Keywords: permutations, permutations, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_permutations(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_permutations(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        elements: Vec<i32>,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'elements' (array of i32). Optional 'mode': 'generate' | 'count'."
            .to_string(),
    })?;

    let result = match req.mode.as_deref().unwrap_or("") {
        "count" => {
            let count = Permutations::count(req.elements.len() as u64);
            json!({ "mode": "count", "count": count })
        }
        _ => {
            let mut elems = req.elements.clone();
            let perms = Permutations::solve(&mut elems);
            let count = perms.len();
            json!({ "mode": "generate", "total": count, "permutations": perms })
        }
    };

    let solver = Permutations;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["generate", "count"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Permutations computed."))
}

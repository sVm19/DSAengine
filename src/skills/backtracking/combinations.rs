use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Combinations
/// CATEGORY: backtracking
/// DESCRIPTION: Generates all k-length combinations from 1..=n using iterative backtracking
///              with an explicit stack frame to avoid recursion-induced stack overflow.
pub struct Combinations;

impl Complexity for Combinations {
    fn name(&self) -> &'static str {
        "Combinations (Iterative Backtracking)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(C(n,k) * k) - Enumerating C(n,k) combinations, each taking O(k) to copy out."
    }

    fn space_complexity(&self) -> &'static str {
        "O(k) auxiliary - The current path buffer; output storage excluded."
    }

    fn description(&self) -> &'static str {
        "Iteratively drives a path buffer with an explicit (start, index) stack frame, pruning branches where remaining elements cannot fill k slots."
    }
}

/// One frame on the iterative backtracking stack.
struct Frame {
    /// Next candidate value to try at `path[depth]`.
    next_candidate: u32,
    /// Depth in the path this frame owns.
    depth: usize,
}

impl Combinations {
    /// Returns all unique combinations of k numbers chosen from 1..=n.
    ///
    /// Uses an iterative simulation of the classic backtracking recursion:
    ///   choose(start, depth) → for val in start..=n - (k - depth - 1): push, recurse, pop
    pub fn solve(n: u32, k: usize) -> Vec<Vec<u32>> {
        if k == 0 || k as u32 > n {
            return Vec::new();
        }

        let mut results: Vec<Vec<u32>> = Vec::new();
        let mut path: Vec<u32> = Vec::with_capacity(k);
        // Stack holds the next candidate to try for each depth level.
        let mut stack: Vec<Frame> = vec![Frame {
            next_candidate: 1,
            depth: 0,
        }];

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Generating C({n},{k}) combinations; pruning when remaining slots cannot be filled."),
        );

        while let Some(frame) = stack.last_mut() {
            let depth = frame.depth;
            let candidate = frame.next_candidate;

            // Pruning: remaining candidates from [candidate..=n] must be >= (k - depth) slots needed.
            let remaining_needed = (k - depth) as u32;
            if candidate + remaining_needed - 1 > n {
                // Backtrack: this depth cannot yield any more valid combinations.
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Pruning depth {} — candidate {} leaves only {} values for {} slots.",
                        depth,
                        candidate,
                        n.saturating_sub(candidate) + 1,
                        remaining_needed
                    ),
                );
                stack.pop();
                path.pop();
                continue;
            }

            // Advance this frame's candidate for next iteration.
            frame.next_candidate += 1;

            // Extend the current path.
            path.push(candidate);

            if depth + 1 == k {
                // Full combination collected.
                AgentLogger::log(
                    AgentFeedback::Success,
                    format!("Collected combination: {:?}.", path),
                );
                results.push(path.clone());
                path.pop();
            } else {
                // Go deeper: push a new frame starting from candidate+1.
                stack.push(Frame {
                    next_candidate: candidate + 1,
                    depth: depth + 1,
                });
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Enumerated {} combination(s) total.", results.len()),
        );
        results
    }

    /// Counts valid combinations without storing them — uses the multiplicative formula.
    pub fn count(n: u32, k: usize) -> u64 {
        if k as u32 > n {
            return 0;
        }
        let k = k.min((n as usize) - k); // C(n,k) == C(n, n-k)
        let mut result = 1u64;
        for i in 0..k as u32 {
            result = result * (n - i) as u64 / (i + 1) as u64;
        }
        result
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "backtracking.combinations",
    description = "Use this for solving combinations problems. Trigger Keywords: combinations, combinations, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_combinations(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CombinationsRequest {
    pub n: u32,
    pub k: usize,
    pub include_results: Option<bool>,
    pub max_results: Option<usize>,
}

async fn handle_combinations(payload: Value) -> DsaResult<ResultBox> {
    let req: CombinationsRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid CombinationsRequest: {e}"),
            hint: "Provide 'n' and 'k', with optional 'include_results' and 'max_results'."
                .to_string(),
        })?;

    let count = Combinations::count(req.n, req.k);
    let combinations = if req.include_results.unwrap_or(true) {
        let mut values = Combinations::solve(req.n, req.k);
        if let Some(limit) = req.max_results {
            values.truncate(limit);
        }
        Some(values)
    } else {
        None
    };
    let solver = Combinations;

    Ok(ResultBox::success(json!({
        "n": req.n,
        "k": req.k,
        "count": count,
        "combinations": combinations
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Combination generation completed."))
}

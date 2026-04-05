use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Subsets
/// CATEGORY: backtracking
/// DESCRIPTION: Generates the power-set of an integer slice in O(2^n) using the bit-mask
///              enumeration approach — no recursion required.
pub struct Subsets;

impl Complexity for Subsets {
    fn name(&self) -> &'static str {
        "Subsets (Bitmask Power-Set Enumeration)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(2^n * n) — Exactly 2^n subsets; each requires O(n) to collect selected elements."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) per subset — No auxiliary recursion stack; output slices reference input indices."
    }

    fn description(&self) -> &'static str {
        "Iterates over every integer mask from 0 to 2^n-1; bit i set in the mask means element i is included, producing a unique subset per mask."
    }
}

impl Subsets {
    /// Returns all 2^n subsets of `elements` via bitmask enumeration.
    ///
    /// This is iterative and avoids recursion entirely:
    ///   for mask in 0..(1 << n): include element[i] if bit i is set.
    pub fn solve(elements: &[i32]) -> Vec<Vec<i32>> {
        let n = elements.len();
        if n > 30 {
            // Guard: 2^31 subsets would exhaust memory.
            AgentLogger::log(
                AgentFeedback::Warning,
                format!("Input length {n} exceeds safe bitmask limit (30); aborting."),
            );
            return Vec::new();
        }

        let total: u32 = 1u32 << n;
        let mut results: Vec<Vec<i32>> = Vec::with_capacity(total as usize);

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Enumerating power-set of {n} element(s); producing {} subsets total.", total),
        );

        for mask in 0..total {
            let mut subset: Vec<i32> = Vec::new();
            for bit in 0..n {
                if (mask >> bit) & 1 == 1 {
                    subset.push(elements[bit]);
                }
            }

            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Mask {mask:#0width$b} → selected indices {:?} → subset {:?}.",
                    (0..n).filter(|&b| (mask >> b) & 1 == 1).collect::<Vec<_>>(),
                    subset,
                    width = n + 2,
                ),
            );

            results.push(subset);
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Power-set generation complete: {} subsets produced.", results.len()),
        );
        results
    }

    /// Returns all subsets of a given size `k` (i.e., all k-combinations).
    ///
    /// Reuses bitmask enumeration but filters to masks with exactly k bits set.
    pub fn of_size(elements: &[i32], k: usize) -> Vec<Vec<i32>> {
        let n = elements.len();
        if k > n || n > 30 {
            return Vec::new();
        }

        let total: u32 = 1u32 << n;
        let mut results: Vec<Vec<i32>> = Vec::new();

        for mask in 0..total {
            if (mask.count_ones() as usize) == k {
                let subset: Vec<i32> = (0..n)
                    .filter(|&b| (mask >> b) & 1 == 1)
                    .map(|b| elements[b])
                    .collect();

                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("k={k} subset captured: {:?}.", subset),
                );
                results.push(subset);
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Subsets of size {k}: {} found.", results.len()),
        );
        results
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "subsets", description = "Use this for solving subsets problems. Trigger Keywords: subsets, subsets, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

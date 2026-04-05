use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Add Two Numbers
/// CATEGORY: linked-lists
/// DESCRIPTION: Adds two non-negative integers stored as digit linked lists
///              (least-significant digit first), producing a new digit list.
///
/// Represented as `&[u8]` slices (each element = 0–9, index 0 = LSD).
pub struct AddTwoNumbers;

impl Complexity for AddTwoNumbers {
    fn name(&self) -> &'static str {
        "Add Two Numbers (LSD-First Digit List Addition)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(max(m, n)) — One pass through the longer of the two digit lists plus carry propagation."
    }

    fn space_complexity(&self) -> &'static str {
        "O(max(m, n) + 1) — Output digit Vec; at most one extra digit for the final carry."
    }

    fn description(&self) -> &'static str {
        "Zips through both slices simultaneously, summing corresponding digits plus carry; any remaining digits and a final carry are appended in one pass."
    }
}

impl AddTwoNumbers {
    /// Adds `a` and `b` (both LSD-first digit slices) and returns the result as a
    /// LSD-first digit `Vec<u8>`.
    pub fn solve(a: &[u8], b: &[u8]) -> Vec<u8> {
        let (m, n) = (a.len(), b.len());
        let max_len = m.max(n);

        let mut result = Vec::with_capacity(max_len + 1);
        let mut carry = 0u8;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Adding two digit lists: len_a={m}, len_b={n}."),
        );

        for i in 0..max_len {
            let da = if i < m { a[i] } else { 0 };
            let db = if i < n { b[i] } else { 0 };
            let sum = da + db + carry;
            carry = sum / 10;
            let digit = sum % 10;
            result.push(digit);

            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Position {i}: {da} + {db} + carry={} = {sum} → digit={digit}, new_carry={carry}.",
                    sum - digit - carry * 10 + carry  // carry before this step
                ),
            );
        }

        if carry > 0 {
            result.push(carry);
            AgentLogger::log(
                AgentFeedback::Step,
                format!("Final carry={carry} appended at position {}.", result.len() - 1),
            );
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Result: {} digit(s), MSD={}.", result.len(), result.last().copied().unwrap_or(0)),
        );
        result
    }

    /// Converts a digit slice (LSD-first) to its decimal `u128` value.
    pub fn to_number(digits: &[u8]) -> u128 {
        digits.iter().rev().fold(0u128, |acc, &d| acc * 10 + d as u128)
    }

    /// Converts a `u128` to an LSD-first digit `Vec<u8>`.
    pub fn from_number(mut n: u128) -> Vec<u8> {
        if n == 0 { return vec![0]; }
        let mut digits = Vec::new();
        while n > 0 {
            digits.push((n % 10) as u8);
            n /= 10;
        }
        digits
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "add_two_numbers", description = "Use this for solving add two numbers problems. Trigger Keywords: add_two_numbers, add two numbers, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

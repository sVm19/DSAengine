use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Edit Distance
/// CATEGORY: dynamic-programming
/// DESCRIPTION: Computes the minimum number of single-character insert, delete, or replace
///              operations (Levenshtein distance) to transform one string into another.
pub struct EditDistance;

impl Complexity for EditDistance {
    fn name(&self) -> &'static str {
        "Edit Distance (Levenshtein — Two-Row DP)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(m * n) — Fills an m×n DP grid where m, n are the two string lengths."
    }

    fn space_complexity(&self) -> &'static str {
        "O(min(m, n)) — Keeps only two rolling rows instead of the full m×n table."
    }

    fn description(&self) -> &'static str {
        "Uses two alternating rowsof n+1 cells; dp[i][j] = cost to align src[..i] with dst[..j] via insert/delete/replace."
    }
}

impl EditDistance {
    /// Returns the Levenshtein distance between `src` and `dst`.
    pub fn solve(src: &str, dst: &str) -> usize {
        let src = src.as_bytes();
        let dst = dst.as_bytes();

        // Always iterate over the longer string as outer loop; swap if needed.
        let (s, d) = if src.len() < dst.len() { (dst, src) } else { (src, dst) };
        let (m, n) = (s.len(), d.len());

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Edit-distance DP: src_len={m}, dst_len={n}; using two-row O(n) space."),
        );

        // prev[j] = edit_distance(s[..i-1], d[..j])
        let mut prev: Vec<usize> = (0..=n).collect();
        let mut curr = vec![0usize; n + 1];

        for i in 1..=m {
            curr[0] = i; // cost of deleting all i chars from s to reach empty d
            for j in 1..=n {
                curr[j] = if s[i - 1] == d[j - 1] {
                    // Characters match — no extra cost.
                    prev[j - 1]
                } else {
                    1 + prev[j - 1]      // Replace
                        .min(prev[j])    // Delete from s
                        .min(curr[j - 1])// Insert into s
                };
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "dp[{i}][{j}]: '{}' vs '{}' → cost={}.",
                        s[i - 1] as char, d[j - 1] as char, curr[j]
                    ),
                );
            }
            std::mem::swap(&mut prev, &mut curr);
        }

        let result = prev[n];
        AgentLogger::log(
            AgentFeedback::Success,
            format!("Levenshtein distance = {result}."),
        );
        result
    }

    /// Returns the edit distance and the operation sequence as a human-readable string.
    pub fn solve_with_trace(src: &str, dst: &str) -> (usize, Vec<String>) {
        let s = src.as_bytes();
        let d = dst.as_bytes();
        let (m, n) = (s.len(), d.len());

        // Full m×n table needed for traceback.
        let mut dp = vec![vec![0usize; n + 1]; m + 1];
        for i in 0..=m { dp[i][0] = i; }
        for j in 0..=n { dp[0][j] = j; }

        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = if s[i - 1] == d[j - 1] {
                    dp[i - 1][j - 1]
                } else {
                    1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1])
                };
            }
        }

        // Traceback.
        let mut ops: Vec<String> = Vec::new();
        let (mut i, mut j) = (m, n);
        while i > 0 || j > 0 {
            if i > 0 && j > 0 && s[i - 1] == d[j - 1] {
                ops.push(format!("Match '{}'", s[i - 1] as char));
                i -= 1; j -= 1;
            } else if i > 0 && j > 0 && dp[i][j] == dp[i - 1][j - 1] + 1 {
                ops.push(format!("Replace '{}' → '{}'", s[i - 1] as char, d[j - 1] as char));
                i -= 1; j -= 1;
            } else if i > 0 && dp[i][j] == dp[i - 1][j] + 1 {
                ops.push(format!("Delete '{}'", s[i - 1] as char));
                i -= 1;
            } else {
                ops.push(format!("Insert '{}'", d[j - 1] as char));
                j -= 1;
            }
        }
        ops.reverse();

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Traceback produced {} operation(s) with total cost {}.", ops.len(), dp[m][n]),
        );
        (dp[m][n], ops)
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "edit_distance", description = "Use this for solving edit distance problems. Trigger Keywords: edit_distance, edit distance, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

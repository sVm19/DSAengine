use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: LCS Solver (Longest Common Subsequence)
/// CATEGORY: dynamic-programming
/// DESCRIPTION: Finds the length and one actual LCS between two byte strings using
///              O(min(m,n)) space tabulation and a traceback phase.
pub struct LcsSolver;

impl Complexity for LcsSolver {
    fn name(&self) -> &'static str {
        "LCS Solver (Two-Row Space-Optimised Tabulation)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(m * n) — Fills each cell of an m×n DP grid exactly once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(min(m,n)) for length query; O(m*n) for full traceback to reconstruct the subsequence."
    }

    fn description(&self) -> &'static str {
        "dp[i][j] = length of LCS(s[..i], t[..j]); space collapses to two rows for length-only queries."
    }
}

impl LcsSolver {
    /// Returns the length of the longest common subsequence of `s` and `t`.
    pub fn solve(s: &str, t: &str) -> usize {
        let sb = s.as_bytes();
        let tb = t.as_bytes();

        // Ensure the shorter sequence drives the column allocation.
        let (a, b) = if sb.len() < tb.len() { (sb, tb) } else { (tb, sb) };
        let (m, n) = (b.len(), a.len());

        AgentLogger::log(
            AgentFeedback::Info,
            format!("LCS tabulation: outer={m} chars, inner={n} chars; two-row O(n) space."),
        );

        let mut prev = vec![0usize; n + 1];
        let mut curr = vec![0usize; n + 1];

        for i in 1..=m {
            for j in 1..=n {
                curr[j] = if b[i - 1] == a[j - 1] {
                    prev[j - 1] + 1
                } else {
                    prev[j].max(curr[j - 1])
                };
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "dp[{i}][{j}]: '{}' vs '{}' → lcs_len={}.",
                        b[i - 1] as char, a[j - 1] as char, curr[j]
                    ),
                );
            }
            std::mem::swap(&mut prev, &mut curr);
            curr.fill(0);
        }

        let result = prev[n];
        AgentLogger::log(
            AgentFeedback::Success,
            format!("LCS length = {result}."),
        );
        result
    }

    /// Returns one actual longest common subsequence as a `String`, using O(m*n) traceback.
    pub fn reconstruct(s: &str, t: &str) -> String {
        let sb = s.as_bytes();
        let tb = t.as_bytes();
        let (m, n) = (sb.len(), tb.len());

        let mut dp = vec![vec![0usize; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = if sb[i - 1] == tb[j - 1] {
                    dp[i - 1][j - 1] + 1
                } else {
                    dp[i - 1][j].max(dp[i][j - 1])
                };
            }
        }

        // Traceback from dp[m][n].
        let mut result: Vec<u8> = Vec::with_capacity(dp[m][n]);
        let (mut i, mut j) = (m, n);
        while i > 0 && j > 0 {
            if sb[i - 1] == tb[j - 1] {
                result.push(sb[i - 1]);
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("LCS char '{}' traced from dp[{i}][{j}].", sb[i - 1] as char),
                );
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] >= dp[i][j - 1] {
                i -= 1;
            } else {
                j -= 1;
            }
        }
        result.reverse();

        let lcs = String::from_utf8_lossy(&result).into_owned();
        AgentLogger::log(
            AgentFeedback::Success,
            format!("Reconstructed LCS: \"{lcs}\" (length {}).", lcs.len()),
        );
        lcs
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "lcs_solver", description = "Use this for solving lcs solver problems. Trigger Keywords: lcs_solver, lcs solver, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

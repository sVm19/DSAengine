use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Palindrome Partition
/// CATEGORY: dynamic-programming
/// DESCRIPTION: Finds the minimum number of cuts needed to partition a string such that
///              every substring is a palindrome, using O(n²) DP with Manacher-expanded centers.
pub struct PalindromePartition;

impl Complexity for PalindromePartition {
    fn name(&self) -> &'static str {
        "Palindrome Partition (Min-Cut DP)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n²) — Expanding palindrome centers fills is_palindrome[i][j] in O(n²); min-cut DP also O(n²)."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n²) — The is_palindrome table and the n-element min-cut array."
    }

    fn description(&self) -> &'static str {
        "Precomputes is_pal[i][j] via center-expansion, then fills cuts[j] = min over all i≤j where is_pal[i][j] of (cuts[i-1]+1)."
    }
}

impl PalindromePartition {
    /// Returns the minimum number of cuts to partition `s` into palindromic substrings.
    pub fn solve(s: &str) -> usize {
        let b = s.as_bytes();
        let n = b.len();
        if n <= 1 {
            return 0;
        }

        // is_pal[i][j] = true if b[i..=j] is a palindrome.
        let mut is_pal = vec![vec![false; n]; n];

        // Every single character is a palindrome.
        for i in 0..n {
            is_pal[i][i] = true;
        }

        // Expand palindromes by length.
        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                if b[i] == b[j] {
                    is_pal[i][j] = len == 2 || is_pal[i + 1][j - 1];
                }
            }
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Palindrome table built for {n}-char string; computing min-cut DP."),
        );

        // cuts[j] = minimum cuts for b[0..=j].
        let mut cuts = vec![0usize; n];

        for j in 1..n {
            if is_pal[0][j] {
                // Entire b[0..=j] is already a palindrome.
                cuts[j] = 0;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("b[0..={j}] is a palindrome → cuts[{j}] = 0."),
                );
                continue;
            }

            cuts[j] = usize::MAX;
            for i in 1..=j {
                if is_pal[i][j] {
                    let candidate = cuts[i - 1] + 1;
                    if candidate < cuts[j] {
                        cuts[j] = candidate;
                        AgentLogger::log(
                            AgentFeedback::Step,
                            format!(
                                "cuts[{j}] updated to {candidate}: cut before i={i} (b[{i}..={j}] is palindrome)."
                            ),
                        );
                    }
                }
            }
        }

        let result = cuts[n - 1];
        AgentLogger::log(
            AgentFeedback::Success,
            format!("Minimum palindrome cuts for \"{}\" = {result}.", s),
        );
        result
    }

    /// Returns one valid partition into palindromic substrings using the computed cut positions.
    pub fn partition(s: &str) -> Vec<String> {
        let b = s.as_bytes();
        let n = b.len();
        if n == 0 {
            return Vec::new();
        }

        let mut is_pal = vec![vec![false; n]; n];
        for i in 0..n { is_pal[i][i] = true; }
        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                if b[i] == b[j] {
                    is_pal[i][j] = len == 2 || is_pal[i + 1][j - 1];
                }
            }
        }

        let mut cuts = vec![0usize; n];
        let mut prev_cut = vec![0usize; n]; // tracks cut point for reconstruction

        for j in 1..n {
            if is_pal[0][j] {
                cuts[j] = 0;
                prev_cut[j] = 0;
                continue;
            }
            cuts[j] = usize::MAX;
            for i in 1..=j {
                if is_pal[i][j] {
                    let candidate = cuts[i - 1] + 1;
                    if candidate < cuts[j] {
                        cuts[j] = candidate;
                        prev_cut[j] = i;
                    }
                }
            }
        }

        // Reconstruct partitions by following prev_cut.
        let mut parts: Vec<String> = Vec::new();
        let mut end = n - 1;
        loop {
            let start = if cuts[end] == 0 { 0 } else { prev_cut[end] };
            parts.push(String::from_utf8_lossy(&b[start..=end]).into_owned());
            if start == 0 { break; }
            end = start - 1;
        }
        parts.reverse();

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Partitioned into {} palindromic piece(s): {:?}.", parts.len(), parts),
        );
        parts
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "palindrome_partition", description = "Use this for solving palindrome partition problems. Trigger Keywords: palindrome_partition, palindrome partition, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

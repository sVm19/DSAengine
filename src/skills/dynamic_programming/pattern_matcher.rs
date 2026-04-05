use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Pattern Matcher (Wildcard Matching with '?' and '*')
/// CATEGORY: dynamic-programming
/// DESCRIPTION: Matches a string against a wildcard pattern where '?' matches any single
///              character and '*' matches any sequence (including empty).  Uses O(n) space DP.
pub struct PatternMatcher;

impl Complexity for PatternMatcher {
    fn name(&self) -> &'static str {
        "Pattern Matcher (Wildcard '?' / '*' — O(n)-space DP)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(m * n) — m pattern chars × n text chars; each cell computed in O(1)."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) — Two rolling rows of length n+1 replace the full m×n table."
    }

    fn description(&self) -> &'static str {
        "dp[i][j] = true if pattern[..i] matches text[..j]; '?' matches one char, '*' may consume zero or more text chars."
    }
}

impl PatternMatcher {
    /// Returns `true` if `pattern` matches the entire `text`.
    /// Supports `?` (any single char) and `*` (any sequence, including empty).
    pub fn solve(pattern: &str, text: &str) -> bool {
        let p = pattern.as_bytes();
        let t = text.as_bytes();
        let (m, n) = (p.len(), t.len());

        // prev[j] = dp[i-1][j], curr[j] = dp[i][j]
        let mut prev = vec![false; n + 1];
        let mut curr = vec![false; n + 1];

        // Base: empty pattern matches empty text only.
        prev[0] = true;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Wildcard DP: pattern_len={m}, text_len={n}; rolling two rows."),
        );

        for i in 1..=m {
            // dp[i][0]: pattern[..i] matches empty text only if all are '*'.
            curr[0] = prev[0] && p[i - 1] == b'*';

            for j in 1..=n {
                curr[j] = if p[i - 1] == b'*' {
                    // '*' matches empty (prev[j]) or one more text char (curr[j-1]).
                    prev[j] || curr[j - 1]
                } else if p[i - 1] == b'?' || p[i - 1] == t[j - 1] {
                    prev[j - 1]
                } else {
                    false
                };

                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "dp[{i}][{j}]: p='{}' t='{}' → {}.",
                        p[i - 1] as char,
                        t[j - 1] as char,
                        curr[j]
                    ),
                );
            }

            std::mem::swap(&mut prev, &mut curr);
            curr.fill(false);
        }

        let result = prev[n];
        AgentLogger::log(
            AgentFeedback::Success,
            format!("Pattern \"{pattern}\" {} text \"{text}\".", if result { "matches" } else { "does NOT match" }),
        );
        result
    }

    /// Checks whether `text` contains any position where `pattern` matches a substring.
    pub fn contains(pattern: &str, text: &str) -> bool {
        // Wrap pattern in '*' on both sides to allow prefix/suffix anything.
        let wrapped = format!("*{pattern}*");
        Self::solve(&wrapped, text)
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "pattern_matcher", description = "Use this for solving pattern matcher problems. Trigger Keywords: pattern_matcher, pattern matcher, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

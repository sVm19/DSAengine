use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Word Break
/// CATEGORY: dynamic-programming
/// DESCRIPTION: Determines whether a string can be segmented into a space-separated sequence
///              of dictionary words using a 1-D DP reachability array.
pub struct WordBreak;

impl Complexity for WordBreak {
    fn name(&self) -> &'static str {
        "Word Break (1-D Reachability DP)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n² * k) — For each end position (n), check all start positions (n); each substring lookup is O(k) for linear scan."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) — Boolean dp array of length n+1 tracks reachable split points."
    }

    fn description(&self) -> &'static str {
        "dp[i] = true if s[..i] can be segmented; for each reachable i, scan all j>i where s[i..j] is in the dictionary."
    }
}

impl WordBreak {
    /// Returns `true` if `s` can be fully segmented using words from `dictionary`.
    pub fn solve(s: &str, dictionary: &[&str]) -> bool {
        let n = s.len();
        if n == 0 {
            return true;
        }

        let mut dp = vec![false; n + 1];
        dp[0] = true; // Empty prefix is always reachable.

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Word-break DP: \"{s}\" ({n} chars), {} dict word(s).", dictionary.len()),
        );

        for i in 0..n {
            if !dp[i] {
                continue; // Position i not reachable — skip.
            }

            for &word in dictionary {
                let end = i + word.len();
                if end <= n && &s[i..end] == word {
                    if !dp[end] {
                        dp[end] = true;
                        AgentLogger::log(
                            AgentFeedback::Step,
                            format!(
                                "Reached position {end}: matched word \"{word}\" at s[{i}..{end}]."
                            ),
                        );
                    }
                }
            }
        }

        let result = dp[n];
        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Word-break \"{s}\": {}.",
                if result { "segmentable" } else { "not segmentable" }
            ),
        );
        result
    }

    /// Returns one valid segmentation if it exists.
    pub fn reconstruct<'a>(s: &'a str, dictionary: &[&'a str]) -> Option<Vec<&'a str>> {
        let n = s.len();
        let mut dp = vec![false; n + 1];
        let mut from = vec![0usize; n + 1]; // tracks which word was used to reach position i
        let mut word_used: Vec<&str> = vec![""; n + 1];
        dp[0] = true;

        for i in 0..n {
            if !dp[i] { continue; }
            for &word in dictionary {
                let end = i + word.len();
                if end <= n && !dp[end] && &s[i..end] == word {
                    dp[end] = true;
                    from[end] = i;
                    word_used[end] = word;
                }
            }
        }

        if !dp[n] {
            return None;
        }

        // Traceback from position n.
        let mut parts: Vec<&str> = Vec::new();
        let mut pos = n;
        while pos > 0 {
            parts.push(word_used[pos]);
            pos = from[pos];
        }
        parts.reverse();

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Reconstruction: {:?}.", parts),
        );
        Some(parts)
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "word_break", description = "Use this for solving word break problems. Trigger Keywords: word_break, word break, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

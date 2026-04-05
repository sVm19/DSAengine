use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Longest Increasing Subsequence
/// CATEGORY: dynamic-programming
/// DESCRIPTION: Finds the length of the longest strictly increasing subsequence in O(n log n)
///              using a patience-sorting / binary-search tails array, with O(n²) traceback.
pub struct LongestIncreasingSub;

impl Complexity for LongestIncreasingSub {
    fn name(&self) -> &'static str {
        "Longest Increasing Subsequence (Patience Sort O(n log n))"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n log n) — Each element does one binary search on the tails array of length ≤ n."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) — The tails array holds at most n elements."
    }

    fn description(&self) -> &'static str {
        "Maintains a `tails` array where tails[k] is the smallest tail element of any IS of length k+1; binary search locates the insertion point."
    }
}

impl LongestIncreasingSub {
    /// Returns the length of the LIS using O(n log n) patience sorting.
    pub fn solve(nums: &[i32]) -> usize {
        if nums.is_empty() {
            return 0;
        }

        // `tails[k]` = smallest tail value of all IS of length k+1 seen so far.
        let mut tails: Vec<i32> = Vec::with_capacity(nums.len());

        AgentLogger::log(
            AgentFeedback::Info,
            format!("LIS patience-sort over {} element(s).", nums.len()),
        );

        for (idx, &val) in nums.iter().enumerate() {
            // Binary search for leftmost position where tails[pos] >= val.
            let pos = tails.partition_point(|&t| t < val);

            if pos == tails.len() {
                tails.push(val);
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("nums[{idx}]={val}: extended tails to length {}.", tails.len()),
                );
            } else {
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "nums[{idx}]={val}: replaced tails[{pos}]={} with {val}.",
                        tails[pos]
                    ),
                );
                tails[pos] = val;
            }
        }

        let result = tails.len();
        AgentLogger::log(
            AgentFeedback::Success,
            format!("LIS length = {result}; final tails = {tails:?}."),
        );
        result
    }

    /// Reconstructs one actual LIS using an O(n²) DP traceback.
    ///
    /// Stores `dp[i]` = LIS length ending at index i, and `parent[i]` for path recovery.
    pub fn reconstruct(nums: &[i32]) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return Vec::new();
        }

        let mut dp = vec![1usize; n];
        let mut parent = vec![usize::MAX; n];

        for i in 1..n {
            for j in 0..i {
                if nums[j] < nums[i] && dp[j] + 1 > dp[i] {
                    dp[i] = dp[j] + 1;
                    parent[i] = j;
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("dp[{i}]={} via parent index {j} (nums[{j}]={}).", dp[i], nums[j]),
                    );
                }
            }
        }

        // Find the index with the maximum LIS length.
        let best_idx = dp.iter().enumerate().max_by_key(|&(_, &v)| v).map(|(i, _)| i).unwrap();

        // Reconstruct path.
        let mut path = Vec::new();
        let mut cur = best_idx;
        while cur != usize::MAX {
            path.push(nums[cur]);
            cur = parent[cur];
        }
        path.reverse();

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Reconstructed LIS: {path:?}."),
        );
        path
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "longest_increasing_sub", description = "Use this for solving longest increasing sub problems. Trigger Keywords: longest_increasing_sub, longest increasing sub, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

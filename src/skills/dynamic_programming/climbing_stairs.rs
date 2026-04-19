use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Climbing Stairs
/// CATEGORY: dynamic-programming
/// DESCRIPTION: Counts the distinct ways to climb n steps taking 1 or 2 steps at a time.
///              Equivalent to Fibonacci(n+1); solved in O(n) time and O(1) space.
pub struct ClimbingStairs;

impl Complexity for ClimbingStairs {
    fn name(&self) -> &'static str {
        "Climbing Stairs (Fibonacci Tabulation)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) — One linear pass computing the Fibonacci recurrence dp[i] = dp[i-1] + dp[i-2]."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Only two rolling variables are maintained; no dp array allocated."
    }

    fn description(&self) -> &'static str {
        "Recognises that ways(n) = ways(n-1) + ways(n-2) and collapses the recurrence to two variables."
    }
}

impl ClimbingStairs {
    /// Returns the number of distinct ways to climb `n` steps.
    ///
    /// Recurrence: ways(0) = 1, ways(1) = 1, ways(n) = ways(n-1) + ways(n-2).
    pub fn solve(n: u64) -> u64 {
        if n == 0 {
            return 1;
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Computing climbing-stairs ways for n={n} with O(1)-space Fibonacci iteration."
            ),
        );

        let mut prev2 = 1u64; // ways(0)
        let mut prev1 = 1u64; // ways(1)

        for step in 2..=n {
            let current = prev1.saturating_add(prev2);
            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Step {step}: ways = ways({}) + ways({}) = {prev1} + {prev2} = {current}.",
                    step - 1,
                    step - 2
                ),
            );
            prev2 = prev1;
            prev1 = current;
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Total distinct ways to climb {n} step(s): {prev1}."),
        );
        prev1
    }

    /// Generalises to k-step climbs: counts ways when you can take 1..=k steps at a time.
    ///
    /// Uses a sliding-window prefix sum over the last k dp values.
    pub fn solve_k_steps(n: usize, k: usize) -> u64 {
        if n == 0 || k == 0 {
            return 1;
        }

        let mut dp = vec![0u64; n + 1];
        dp[0] = 1;

        // Maintain a running window sum of the last k entries.
        let mut window_sum = 1u64;

        for step in 1..=n {
            dp[step] = window_sum;

            AgentLogger::log(
                AgentFeedback::Step,
                format!("Step {step} (k={k}): dp[{step}] = window_sum = {window_sum}."),
            );

            window_sum = window_sum.saturating_add(dp[step]);
            if step >= k {
                window_sum = window_sum.saturating_sub(dp[step - k]);
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Evicted dp[{}] = {} from sliding window.",
                        step - k,
                        dp[step - k]
                    ),
                );
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Ways to climb {n} steps with at most {k}-step jumps: {}.",
                dp[n]
            ),
        );
        dp[n]
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "dynamic_programming.climbing_stairs",
    description = "Use this for solving climbing stairs problems. Trigger Keywords: climbing_stairs, climbing stairs, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_climbing_stairs(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ClimbingStairsRequest {
    pub n: u64,
    pub k: Option<usize>,
}

async fn handle_climbing_stairs(payload: Value) -> DsaResult<ResultBox> {
    let req: ClimbingStairsRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid ClimbingStairsRequest: {e}"),
            hint: "Provide 'n' and optional 'k' for jumps of 1..=k steps.".to_string(),
        })?;

    let ways = if let Some(k) = req.k {
        ClimbingStairs::solve_k_steps(req.n as usize, k)
    } else {
        ClimbingStairs::solve(req.n)
    };
    let solver = ClimbingStairs;

    Ok(ResultBox::success(json!({
        "n": req.n,
        "k": req.k,
        "ways": ways
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Climbing-stairs DP computation completed."))
}

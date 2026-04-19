use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Coin Change
/// CATEGORY: dynamic-programming
/// DESCRIPTION: Finds the minimum number of coins from a given denominations slice
///              that sum to `amount`, using bottom-up DP tabulation.
pub struct CoinChange;

impl Complexity for CoinChange {
    fn name(&self) -> &'static str {
        "Coin Change (Bottom-Up Tabulation)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(amount * |coins|) — For each value 1..=amount, try every coin denomination once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(amount) — One DP table of size amount+1; coin slice is read-only."
    }

    fn description(&self) -> &'static str {
        "Fills dp[v] = min(dp[v], dp[v - coin] + 1) for every coin ≤ v, guaranteeing the global optimum via overlapping subproblem reuse."
    }
}

impl CoinChange {
    /// Returns the minimum number of coins that sum to `amount`, or `None` if impossible.
    pub fn solve(coins: &[u32], amount: u32) -> Option<u32> {
        if amount == 0 {
            return Some(0);
        }

        let inf = u32::MAX;
        let mut dp = vec![inf; amount as usize + 1];
        dp[0] = 0;

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Coin-change tabulation for amount={amount} with {} denomination(s).",
                coins.len()
            ),
        );

        for value in 1..=amount as usize {
            for &coin in coins {
                let coin = coin as usize;
                if coin <= value && dp[value - coin] != inf {
                    let candidate = dp[value - coin] + 1;
                    if candidate < dp[value] {
                        dp[value] = candidate;
                        AgentLogger::log(
                            AgentFeedback::Step,
                            format!(
                                "dp[{value}] updated to {candidate} via coin {coin} (dp[{}] = {}).",
                                value - coin,
                                dp[value - coin]
                            ),
                        );
                    }
                }
            }
        }

        if dp[amount as usize] == inf {
            AgentLogger::log(
                AgentFeedback::Warning,
                format!("Amount {amount} is unreachable with the given denominations."),
            );
            None
        } else {
            let result = dp[amount as usize];
            AgentLogger::log(
                AgentFeedback::Success,
                format!("Minimum coins to make {amount}: {result}."),
            );
            Some(result)
        }
    }

    /// Counts the total number of distinct combinations that sum to `amount` (unbounded knapsack).
    pub fn count_ways(coins: &[u32], amount: u32) -> u64 {
        let mut dp = vec![0u64; amount as usize + 1];
        dp[0] = 1;

        for &coin in coins {
            for value in coin as usize..=amount as usize {
                dp[value] = dp[value].saturating_add(dp[value - coin as usize]);
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Coin {coin}: dp[{value}] += dp[{}] → now {}.",
                        value - coin as usize,
                        dp[value]
                    ),
                );
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Total ways to make amount {amount}: {}.",
                dp[amount as usize]
            ),
        );
        dp[amount as usize]
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "coin_change",
    description = "Use this for solving coin change problems. Trigger Keywords: coin_change, coin change, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_coin_change(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_coin_change(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        coins: Vec<u32>,
        amount: u32,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'coins' (array of u32) and 'amount' (u32). Optional 'mode': 'min_coins' | 'count_ways'.".to_string(),
    })?;

    let result = match req.mode.as_deref().unwrap_or("") {
        "count_ways" => {
            let ways = CoinChange::count_ways(&req.coins, req.amount);
            json!({ "mode": "count_ways", "total_ways": ways })
        }
        _ => {
            let min = CoinChange::solve(&req.coins, req.amount);
            json!({ "mode": "min_coins", "min_coins": min, "solvable": min.is_some() })
        }
    };

    let solver = CoinChange;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["min_coins", "count_ways"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Coin change computation completed."))
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: House Robber
/// CATEGORY: dynamic-programming
/// DESCRIPTION: Finds the maximum money that can be robbed from non-adjacent houses
///              in a single pass with O(1) space using two rolling variables.
pub struct HouseRobber;

impl Complexity for HouseRobber {
    fn name(&self) -> &'static str {
        "House Robber (O(1)-Space Rolling DP)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) — Single left-to-right sweep; each house evaluated exactly once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Two scalars (rob, skip) replace the full dp array."
    }

    fn description(&self) -> &'static str {
        "Tracks the max profit if the previous house was robbed vs skipped; at each house choose max(rob_prev + current, skip_prev)."
    }
}

impl HouseRobber {
    /// Returns the maximum sum of non-adjacent elements from `houses`.
    pub fn solve(houses: &[u32]) -> u32 {
        match houses.len() {
            0 => return 0,
            1 => return houses[0],
            _ => {}
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "House-robber DP over {} house(s); initialising with first two houses.",
                houses.len()
            ),
        );

        // rob  = best profit when we DID rob house i-1
        // skip = best profit when we SKIPPED house i-1
        let mut rob = houses[0];
        let mut skip = 0u32;

        for (idx, &value) in houses.iter().enumerate().skip(1) {
            let new_rob = skip + value; // Rob current → must have skipped previous
            let new_skip = rob.max(skip); // Skip current → take best of both states

            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "House[{idx}]=${value}: new_rob=skip+value={new_rob}, new_skip=max(rob,skip)={new_skip}."
                ),
            );

            rob = new_rob;
            skip = new_skip;
        }

        let result = rob.max(skip);
        AgentLogger::log(
            AgentFeedback::Success,
            format!("Maximum loot from non-adjacent houses: {result}."),
        );
        result
    }

    /// House Robber II — houses arranged in a circle; first and last cannot both be robbed.
    pub fn solve_circular(houses: &[u32]) -> u32 {
        match houses.len() {
            0 => return 0,
            1 => return houses[0],
            2 => return houses[0].max(houses[1]),
            _ => {}
        }

        AgentLogger::log(
            AgentFeedback::Info,
            "Solving circular variant: taking max of rob[0..n-1] and rob[1..n].",
        );

        // Rob from index 0 to n-2 (excluding last house)
        let pass_a = Self::solve(&houses[..houses.len() - 1]);
        // Rob from index 1 to n-1 (excluding first house)
        let pass_b = Self::solve(&houses[1..]);

        let result = pass_a.max(pass_b);
        AgentLogger::log(
            AgentFeedback::Success,
            format!("Circular house-robber result: max({pass_a}, {pass_b}) = {result}."),
        );
        result
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "house_robber",
    description = "Use this for solving house robber problems. Trigger Keywords: house_robber, house robber, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_house_robber(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_house_robber(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        houses: Vec<u32>,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'houses' (array of u32). Optional 'mode': 'linear' | 'circular'."
            .to_string(),
    })?;

    let result = match req.mode.as_deref().unwrap_or("") {
        "circular" => {
            let max = HouseRobber::solve_circular(&req.houses);
            json!({ "mode": "circular", "max_loot": max })
        }
        _ => {
            let max = HouseRobber::solve(&req.houses);
            json!({ "mode": "linear", "max_loot": max })
        }
    };

    let solver = HouseRobber;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["linear", "circular"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("House robber completed."))
}

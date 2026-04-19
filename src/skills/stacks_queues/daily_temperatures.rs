use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Daily Temperatures
/// CATEGORY: stacks-queues
/// DESCRIPTION: Calculates how many days you have to wait until a warmer temperature.
///              Implemented via a monotonically decreasing stack holding indices.
pub struct DailyTemperatures;

impl Complexity for DailyTemperatures {
    fn name(&self) -> &'static str {
        "Daily Temperatures (Monotonic Stack)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Every element is pushed and popped exactly once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Stack size bounded by the number of temperatures in worst case (decreasing sequence)."
    }

    fn description(&self) -> &'static str {
        "Scans temperatures using a stack to track indices of cold days awaiting a warmup. When a warmer day is found, pop all colder days from the stack and compute the index difference."
    }
}

impl DailyTemperatures {
    /// Returns an array where `ans[i]` is the number of days you have to wait after the `i`th day
    /// to get a warmer temperature.
    pub fn solve(temperatures: &[i32]) -> Vec<i32> {
        let n = temperatures.len();
        let mut ans = vec![0; n];
        let mut stack: Vec<usize> = Vec::with_capacity(n); // Stores indices

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Daily temperatures calculation for {} day(s).", n),
        );

        for (i, &temp) in temperatures.iter().enumerate() {
            // While the current day is warmer than days on the stack
            while let Some(&top_idx) = stack.last() {
                if temp > temperatures[top_idx] {
                    let prev_day = stack.pop().unwrap();
                    ans[prev_day] = (i - prev_day) as i32;
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Day {i} (temp {temp}) is warmer than day {prev_day} (temp {}). Wait time = {}.", temperatures[prev_day], ans[prev_day]),
                    );
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        AgentLogger::log(AgentFeedback::Success, "Daily temperatures calculated.");

        ans
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "daily_temperatures",
    description = "Use this for solving daily temperatures problems. Trigger Keywords: daily_temperatures, daily temperatures, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_daily_temperatures(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_daily_temperatures(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        temperatures: Vec<i32>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'temperatures' (array of i32).".to_string(),
    })?;

    let result = {
        let days = DailyTemperatures::solve(&req.temperatures);
        json!({ "wait_days": days })
    };

    let solver = DailyTemperatures;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["compute"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Daily temperatures computed."))
}

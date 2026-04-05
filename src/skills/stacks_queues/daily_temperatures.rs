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

        AgentLogger::log(
            AgentFeedback::Success,
            "Daily temperatures calculated.",
        );

        ans
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "daily_temperatures", description = "Use this for solving daily temperatures problems. Trigger Keywords: daily_temperatures, daily temperatures, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

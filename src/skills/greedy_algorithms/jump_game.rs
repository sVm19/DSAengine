use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Jump Game
/// CATEGORY: greedy-algorithms
/// DESCRIPTION: Determines whether the last index is reachable from index 0, and computes
///              the minimum number of jumps needed, using a single greedy pass.
pub struct JumpGame;

impl Complexity for JumpGame {
    fn name(&self) -> &'static str {
        "Jump Game (Greedy Max-Reach Scan)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) — Single left-to-right pass maintaining a running max-reachable index."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Only a single `max_reach` scalar is tracked; input slice is read-only."
    }

    fn description(&self) -> &'static str {
        "Maintains `max_reach = max(max_reach, i + nums[i])`; if ever i > max_reach the end is unreachable. Min-jumps extends this with a current-boundary and jump counter."
    }
}

impl JumpGame {
    /// Returns `true` if the last index is reachable from index 0.
    /// `nums[i]` is the maximum jump length from position i.
    pub fn can_reach(nums: &[u32]) -> bool {
        if nums.is_empty() {
            return true;
        }

        let n = nums.len();
        let mut max_reach = 0usize;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Jump-game reachability scan over {n} position(s)."),
        );

        for i in 0..n {
            if i > max_reach {
                // Current index is beyond what any prior jump can reach.
                AgentLogger::log(
                    AgentFeedback::Warning,
                    format!("Index {i} is beyond max_reach={max_reach}; end is unreachable."),
                );
                return false;
            }

            let new_reach = i + nums[i] as usize;
            if new_reach > max_reach {
                max_reach = new_reach;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Index {i}: max_reach extended to {max_reach}."),
                );
            }

            if max_reach >= n - 1 {
                AgentLogger::log(
                    AgentFeedback::Success,
                    format!(
                        "End reachable: max_reach={max_reach} covers last index {}.",
                        n - 1
                    ),
                );
                return true;
            }
        }

        let result = max_reach >= n - 1;
        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Jump-game result: {}.",
                if result { "reachable" } else { "unreachable" }
            ),
        );
        result
    }

    /// Returns the minimum number of jumps to reach the last index (Jump Game II).
    ///
    /// Uses a greedy interval approach: at each jump boundary, choose the position
    /// that extends the next reachable interval the furthest.
    pub fn min_jumps(nums: &[u32]) -> Option<usize> {
        let n = nums.len();
        if n <= 1 {
            return Some(0);
        }

        let mut jumps = 0usize;
        let mut current_end = 0usize; // boundary of the current jump's range
        let mut farthest = 0usize; // farthest position reachable in the next jump

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Min-jumps greedy scan over {n} position(s)."),
        );

        for i in 0..n - 1 {
            let reach = i + nums[i] as usize;
            if reach > farthest {
                farthest = reach;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Index {i}: farthest extended to {farthest}."),
                );
            }

            if i == current_end {
                if farthest <= current_end {
                    // Stuck — cannot advance the boundary.
                    AgentLogger::log(
                        AgentFeedback::Warning,
                        format!("Stuck at boundary {current_end}; end unreachable."),
                    );
                    return None;
                }
                jumps += 1;
                current_end = farthest;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Jump #{jumps}: boundary advanced to {current_end}."),
                );
                if current_end >= n - 1 {
                    break;
                }
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Minimum jumps to reach end: {jumps}."),
        );
        Some(jumps)
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "jump_game",
    description = "Use this for solving jump game problems. Trigger Keywords: jump_game, jump game, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_jump_game(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_jump_game(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        nums: Vec<u32>,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'nums' (array of u32). Optional 'mode': 'can_reach' | 'min_jumps'."
            .to_string(),
    })?;

    let result = match req.mode.as_deref().unwrap_or("") {
        "min_jumps" => {
            let jumps = JumpGame::min_jumps(&req.nums);
            json!({ "mode": "min_jumps", "min_jumps": jumps, "reachable": jumps.is_some() })
        }
        _ => {
            let can = JumpGame::can_reach(&req.nums);
            json!({ "mode": "can_reach", "reachable": can })
        }
    };

    let solver = JumpGame;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["can_reach", "min_jumps"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Jump game completed."))
}

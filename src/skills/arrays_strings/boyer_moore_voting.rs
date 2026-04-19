use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Boyer Moore Voting
/// CATEGORY: arrays-strings
/// DESCRIPTION: Elects and verifies a majority element using vote cancellation.
pub struct BoyerMooreVoting;

impl Complexity for BoyerMooreVoting {
    fn name(&self) -> &'static str {
        "Boyer Moore Voting"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - One pass elects a candidate and one pass verifies it."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - Only a candidate and vote counter are tracked."
    }

    fn description(&self) -> &'static str {
        "Cancels opposing votes so the majority element, if it exists, survives the scan."
    }
}

impl BoyerMooreVoting {
    pub fn solve(nums: &[i32]) -> Option<i32> {
        Self::majority_element(nums)
    }

    pub fn majority_element(nums: &[i32]) -> Option<i32> {
        if nums.is_empty() {
            return None;
        }

        let mut candidate = 0;
        let mut votes = 0;

        for (index, &value) in nums.iter().enumerate() {
            if votes == 0 {
                candidate = value;
                votes = 1;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Resetting candidate to {} at index {}.", candidate, index),
                );
                continue;
            }

            if value == candidate {
                votes += 1;
            } else {
                votes -= 1;
            }
        }

        let frequency = nums.iter().filter(|&&value| value == candidate).count();
        if frequency * 2 > nums.len() {
            AgentLogger::log(
                AgentFeedback::Success,
                format!(
                    "Verified {} as the majority element with frequency {}/{}.",
                    candidate,
                    frequency,
                    nums.len()
                ),
            );
            Some(candidate)
        } else {
            AgentLogger::log(
                AgentFeedback::Warning,
                format!(
                    "Candidate {} failed verification with frequency {}/{}.",
                    candidate,
                    frequency,
                    nums.len()
                ),
            );
            None
        }
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct BoyerMooreVotingRequest {
    pub nums: Vec<i32>,
}

#[macros::mcp_tool(
    name = "arrays_strings.boyer_moore_voting",
    description = "Use this for solving boyer moore voting problems. Trigger Keywords: boyer_moore_voting, boyer moore voting, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_boyer_moore_voting(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_boyer_moore_voting(payload: Value) -> DsaResult<ResultBox> {
    let req: BoyerMooreVotingRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid BoyerMooreVotingRequest: {e}"),
            hint: "Provide 'nums' array.".to_string(),
        })?;

    let majority = BoyerMooreVoting::solve(&req.nums);
    let solver = BoyerMooreVoting;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });

    Ok(ResultBox::success(json!({
        "majority": majority
    }))
    .with_complexity(complexity)
    .with_description("Boyer-Moore majority vote completed."))
}

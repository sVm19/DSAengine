use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Next Permutation
/// CATEGORY: arrays-strings
/// DESCRIPTION: Produces the lexicographically next permutation in place.
pub struct NextPermutation;

impl Complexity for NextPermutation {
    fn name(&self) -> &'static str {
        "Next Permutation"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Scans from the right, swaps once, then reverses the suffix."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - All work is performed in place."
    }

    fn description(&self) -> &'static str {
        "Finds the rightmost ascent, swaps with the next larger suffix element, then reverses the suffix."
    }
}

impl NextPermutation {
    pub fn solve<T: Ord>(values: &mut [T]) -> bool {
        Self::next_permutation(values)
    }

    pub fn next_permutation<T: Ord>(values: &mut [T]) -> bool {
        if values.len() < 2 {
            return false;
        }

        let pivot = (0..values.len() - 1)
            .rev()
            .find(|&index| values[index] < values[index + 1]);

        let Some(pivot_index) = pivot else {
            values.reverse();
            AgentLogger::log(
                AgentFeedback::Info,
                "No ascent found; reversed into the smallest lexicographic ordering.",
            );
            return false;
        };

        let mut successor = values.len() - 1;
        while values[successor] <= values[pivot_index] {
            successor -= 1;
        }

        values.swap(pivot_index, successor);
        AgentLogger::log(
            AgentFeedback::Step,
            format!(
                "Swapped pivot index {} with successor index {}.",
                pivot_index, successor
            ),
        );

        values[pivot_index + 1..].reverse();
        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Reversed suffix [{}..{}) to produce the next permutation.",
                pivot_index + 1,
                values.len()
            ),
        );
        true
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct NextPermutationRequest {
    pub nums: Vec<i32>,
}

#[macros::mcp_tool(name = "arrays_strings.next_permutation", description = "Use this for solving next permutation problems. Trigger Keywords: next_permutation, next permutation, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_next_permutation(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_next_permutation(payload: Value) -> DsaResult<ResultBox> {
    let req: NextPermutationRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid NextPermutationRequest: {e}"),
            hint: "Provide 'nums' array.".to_string(),
        })?;

    let mut nums = req.nums;
    let has_next = NextPermutation::solve(&mut nums);
    let solver = NextPermutation;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });

    Ok(ResultBox::success(json!({
        "has_next": has_next,
        "permutation": nums
    }))
    .with_complexity(complexity)
    .with_description("Next permutation transformation completed."))
}

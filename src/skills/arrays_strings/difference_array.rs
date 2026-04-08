use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Difference Array
/// CATEGORY: arrays-strings
/// DESCRIPTION: Applies batched inclusive range increments using a difference buffer and one prefix pass.
pub struct DifferenceArray;

impl Complexity for DifferenceArray {
    fn name(&self) -> &'static str {
        "Difference Array"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n + q) - q range updates plus one prefix reconstruction over n cells."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) - Stores a difference buffer and the rebuilt output array."
    }

    fn description(&self) -> &'static str {
        "Transforms many range writes into constant-time edge markers, then reconstructs with prefix sums."
    }
}

impl DifferenceArray {
    pub fn solve(base: &[i32], updates: &[(usize, usize, i32)]) -> Vec<i32> {
        if base.is_empty() {
            return Vec::new();
        }

        let mut diff = vec![0; base.len() + 1];
        for &(start, end, delta) in updates {
            if start >= base.len() {
                AgentLogger::log(
                    AgentFeedback::Warning,
                    format!(
                        "Skipping update [{}, {}] because start is out of bounds.",
                        start, end
                    ),
                );
                continue;
            }

            let right = end.min(base.len() - 1);
            if start > right {
                continue;
            }

            diff[start] += delta;
            diff[right + 1] -= delta;
            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Queued delta {} for inclusive range [{}, {}].",
                    delta, start, right
                ),
            );
        }

        let mut carry = 0;
        let mut rebuilt = Vec::with_capacity(base.len());
        for (index, &value) in base.iter().enumerate() {
            carry += diff[index];
            rebuilt.push(value + carry);
        }

        AgentLogger::log(
            AgentFeedback::Success,
            "Collapsed the difference buffer back into the updated array.",
        );
        rebuilt
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct DifferenceArrayUpdate {
    pub start: usize,
    pub end: usize,
    pub delta: i32,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct DifferenceArrayRequest {
    pub base: Vec<i32>,
    pub updates: Vec<DifferenceArrayUpdate>,
}

#[macros::mcp_tool(name = "difference_array", description = "Use this for solving difference array problems. Trigger Keywords: difference_array, difference array, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    let req: DifferenceArrayRequest = match serde_json::from_value(payload) {
        Ok(req) => req,
        Err(e) => {
            let err = DsaError::InvalidInput {
                message: format!("Invalid DifferenceArrayRequest: {e}"),
                hint: "Provide 'base' and 'updates'[{start,end,delta}].".to_string(),
            };
            return err.into_response();
        }
    };

    let updates = req
        .updates
        .iter()
        .map(|u| (u.start, u.end, u.delta))
        .collect::<Vec<_>>();
    let updated = DifferenceArray::solve(&req.base, &updates);

    let solver = DifferenceArray;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });

    let res = ResultBox::success(json!({
        "updated": updated
    }))
    .with_complexity(complexity)
    .with_description("Difference-array updates applied.");

    (StatusCode::OK, Json(res)).into_response()
}

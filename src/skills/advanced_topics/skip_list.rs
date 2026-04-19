use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Skip List
/// CATEGORY: advanced-topics
/// DESCRIPTION: Builds a deterministic layered search index over a sorted slice.
pub struct SkipList;
pub struct SkipListIndex<'a> {
    values: &'a [i32],
    next: Vec<Vec<usize>>,
    head: usize,
}

impl Complexity for SkipList {
    fn name(&self) -> &'static str {
        "Skip List"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n log n) build and O(log n) search using deterministic tower levels."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n log n) - Stores next pointers for every level of the layered index."
    }

    fn description(&self) -> &'static str {
        "Adds express lanes above a sorted base list so searches can descend quickly through progressively finer levels."
    }
}

impl SkipList {
    pub fn solve<'a>(sorted_values: &'a [i32]) -> Option<SkipListIndex<'a>> {
        Self::build(sorted_values)
    }

    pub fn build<'a>(sorted_values: &'a [i32]) -> Option<SkipListIndex<'a>> {
        if sorted_values.is_empty() {
            return None;
        }

        if sorted_values.windows(2).any(|window| window[0] > window[1]) {
            AgentLogger::log(
                AgentFeedback::Warning,
                "Skip-list build requires an already sorted slice.",
            );
            return None;
        }

        let len = sorted_values.len();
        let levels = ((usize::BITS - len.leading_zeros()) as usize).max(1);
        let head = len;
        let mut next = vec![vec![head; len + 1]; levels];
        let mut heads = vec![head; levels];

        for index in (0..len).rev() {
            let height = (1 + (index + 1).trailing_zeros() as usize).min(levels);
            for level in 0..height {
                next[level][index] = heads[level];
                heads[level] = index;
            }
        }

        for level in 0..levels {
            next[level][head] = heads[level];
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Deterministic skip-list index built with {} levels.",
                levels
            ),
        );

        Some(SkipListIndex {
            values: sorted_values,
            next,
            head,
        })
    }
}

impl<'a> SkipListIndex<'a> {
    pub fn search(&self, target: i32) -> Option<usize> {
        let mut current = self.head;

        for level in (0..self.next.len()).rev() {
            loop {
                let candidate = self.next[level][current];
                if candidate == self.head || self.values[candidate] > target {
                    break;
                }
                if self.values[candidate] == target {
                    AgentLogger::log(
                        AgentFeedback::Success,
                        format!(
                            "Found target {} at index {} while traversing level {}.",
                            target, candidate, level
                        ),
                    );
                    return Some(candidate);
                }

                current = candidate;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Skipped forward to index {} with value {} on level {}.",
                        current, self.values[current], level
                    ),
                );
            }
        }

        AgentLogger::log(
            AgentFeedback::Warning,
            format!("Target {} is absent from the skip-list index.", target),
        );
        None
    }

    pub fn lower_bound(&self, target: i32) -> Option<usize> {
        let mut current = self.head;

        for level in (0..self.next.len()).rev() {
            loop {
                let candidate = self.next[level][current];
                if candidate == self.head || self.values[candidate] >= target {
                    break;
                }
                current = candidate;
            }
        }

        let candidate = self.next[0][current];
        (candidate != self.head).then_some(candidate)
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "skip_list",
    description = "Use this for solving skip list problems. Trigger Keywords: skip_list, skip list, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_skip_list(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_skip_list(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        values: Vec<i32>,
        target: Option<i32>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'values' (sorted array of i32). Optional 'target' to search.".to_string(),
    })?;

    let result = {
        let index = SkipList::build(&req.values).ok_or(DsaError::InvalidInput {
            message: "Cannot build skip list from empty values.".to_string(),
            hint: "Provide at least one value.".to_string(),
        })?;
        let search_result = req.target.map(|t| index.search(t));
        json!({ "size": req.values.len(), "target": req.target, "found_index": search_result })
    };

    let solver = SkipList;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["build_and_search"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Skip list completed."))
}

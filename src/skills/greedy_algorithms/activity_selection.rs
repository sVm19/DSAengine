use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Activity Selection
/// CATEGORY: greedy-algorithms
/// DESCRIPTION: Selects the maximum number of non-overlapping intervals (activities)
///              using the classic greedy earliest-finish-time algorithm.
pub struct ActivitySelection;

impl Complexity for ActivitySelection {
    fn name(&self) -> &'static str {
        "Activity Selection (Earliest Finish Time Greedy)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n log n) — Dominated by sorting activities by finish time; the selection pass is O(n)."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) — Sorted index array; original activity slice is read-only."
    }

    fn description(&self) -> &'static str {
        "Sorts activities by finish time; greedily picks each activity whose start time ≥ the last selected finish time, maximising the count of compatible activities."
    }
}

impl ActivitySelection {
    /// Returns the indices of the maximum set of non-overlapping activities.
    ///
    /// `activities` — slice of `(start, finish)` pairs (half-open intervals: start ≤ t < finish).
    pub fn solve(activities: &[(u64, u64)]) -> Vec<usize> {
        if activities.is_empty() {
            return Vec::new();
        }

        // Build a sorted index array by finish time (preserving zero-copy on activities).
        let mut order: Vec<usize> = (0..activities.len()).collect();
        order.sort_unstable_by_key(|&i| activities[i].1);

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Activity selection: {} activity(ies), sorted by finish time.",
                activities.len()
            ),
        );

        let mut selected: Vec<usize> = Vec::new();
        let mut last_finish = 0u64;

        for idx in order {
            let (start, finish) = activities[idx];
            if start >= last_finish {
                selected.push(idx);
                last_finish = finish;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Selected activity {idx} [{start}, {finish}); last_finish updated to {finish}."
                    ),
                );
            } else {
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Skipped activity {idx} [{start}, {finish}): overlaps with last_finish={last_finish}."
                    ),
                );
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Selected {}/{} non-overlapping activity(ies): {:?}.",
                selected.len(),
                activities.len(),
                selected
            ),
        );
        selected
    }

    /// Returns the minimum number of rooms (resources) needed to host all activities simultaneously.
    ///
    /// Uses a sweep-line over start/finish events — equivalent to maximum interval overlap.
    pub fn min_rooms(activities: &[(u64, u64)]) -> usize {
        let mut events: Vec<(u64, i32)> = Vec::with_capacity(activities.len() * 2);
        for &(start, finish) in activities {
            events.push((start, 1)); // +1 when an activity starts
            events.push((finish, -1)); // -1 when it ends (assuming half-open, end is free)
        }
        // Tie-break: ends (-1) before starts (+1) so a room is freed before being reused.
        events.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

        let mut rooms = 0usize;
        let mut concurrent = 0i32;

        for (time, delta) in events {
            concurrent += delta;
            if concurrent as usize > rooms {
                rooms = concurrent as usize;
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("At t={time}: concurrent activities reached {rooms}."),
                );
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Minimum rooms required: {rooms}."),
        );
        rooms
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "activity_selection",
    description = "Use this for solving activity selection problems. Trigger Keywords: activity_selection, activity selection, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_activity_selection(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_activity_selection(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        activities: Vec<(u64, u64)>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'activities' as [(start, end)] tuples.".to_string(),
    })?;

    let result = {
        let selected = ActivitySelection::solve(&req.activities);
        json!({ "selected": selected, "count": selected.len() })
    };

    let solver = ActivitySelection;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["select"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Activity selection completed."))
}

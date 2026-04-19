use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Queue Reconstruct by Height
/// CATEGORY: greedy-algorithms
/// DESCRIPTION: Reconstructs a queue from a list of (height, k) pairs where k is the number
///              of people of equal or greater height standing in front of this person.
pub struct QueueReconstruct;

impl Complexity for QueueReconstruct {
    fn name(&self) -> &'static str {
        "Queue Reconstruct by Height (Sort + Insert)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n²) — Sorting is O(n log n); inserting each of n people at position k in a Vec costs O(n) per insert."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) — The output queue; sorting uses O(log n) stack space."
    }

    fn description(&self) -> &'static str {
        "Sorts by descending height (ties broken by ascending k); inserts each person at their k-th position. Taller people are placed first so shorter insertions never invalidate earlier placements."
    }
}

impl QueueReconstruct {
    /// Reconstructs the queue from `people` — a slice of (height, k) pairs.
    ///
    /// Returns a `Vec<(u32, usize)>` in valid queue order.
    pub fn solve(people: &[(u32, usize)]) -> Vec<(u32, usize)> {
        if people.is_empty() {
            return Vec::new();
        }

        // Sort: descending height, ascending k as tiebreaker.
        let mut sorted: Vec<(u32, usize)> = people.to_vec(); // mutable copy needed for sort
        sorted.sort_unstable_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Queue reconstruct: {} people, sorted by desc height then asc k.",
                people.len()
            ),
        );

        let mut queue: Vec<(u32, usize)> = Vec::with_capacity(people.len());

        for &(h, k) in &sorted {
            // Insert at position k — all already-placed people are ≥ h, so they count correctly.
            queue.insert(k, (h, k));
            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Inserted ({h}, {k}) at position {k}; queue length now {}.",
                    queue.len()
                ),
            );
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Queue reconstruction complete: {:?}.",
                &queue[..queue.len().min(5)]
            ),
        );
        queue
    }

    /// Verifies that the reconstructed queue satisfies all (height, k) constraints.
    pub fn verify(queue: &[(u32, usize)]) -> bool {
        for (pos, &(h, k)) in queue.iter().enumerate() {
            let count = queue[..pos].iter().filter(|&&(ph, _)| ph >= h).count();
            if count != k {
                AgentLogger::log(
                    AgentFeedback::Warning,
                    format!(
                        "Constraint violated at pos {pos}: person ({h}, {k}) has {count} taller/equal ahead."
                    ),
                );
                return false;
            }
        }
        AgentLogger::log(AgentFeedback::Success, "All queue constraints satisfied.");
        true
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "queue_reconstruct",
    description = "Use this for solving queue reconstruct problems. Trigger Keywords: queue_reconstruct, queue reconstruct, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_queue_reconstruct(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_queue_reconstruct(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        people: Vec<(u32, usize)>,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'people' as [(height, k)]. Optional 'mode': 'reconstruct' | 'verify'."
            .to_string(),
    })?;

    let result = match req.mode.as_deref().unwrap_or("") {
        "verify" => {
            let valid = QueueReconstruct::verify(&req.people);
            json!({ "mode": "verify", "valid": valid })
        }
        _ => {
            let queue = QueueReconstruct::solve(&req.people);
            json!({ "mode": "reconstruct", "queue": queue })
        }
    };

    let solver = QueueReconstruct;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["reconstruct", "verify"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Queue reconstruction completed."))
}

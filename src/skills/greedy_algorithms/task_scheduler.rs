use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Task Scheduler
/// CATEGORY: greedy-algorithms
/// DESCRIPTION: Computes the minimum CPU intervals to finish all tasks given a cooldown n
///              between identical task types, using the frequency-based greedy formula.
pub struct TaskScheduler;

impl Complexity for TaskScheduler {
    fn name(&self) -> &'static str {
        "Task Scheduler (Frequency-Bucket Greedy Formula)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(T) — T = total tasks; counting frequencies is O(T); the formula itself is O(1) after counting."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Fixed-size 26-bucket frequency array for uppercase letter tasks."
    }

    fn description(&self) -> &'static str {
        "Inserts idle slots around the most frequent task type: result = max(T, (max_freq - 1) * (n + 1) + tasks_with_max_freq). Handles arbitrary cooldowns and task alphabets."
    }
}

impl TaskScheduler {
    /// Returns the minimum number of CPU intervals to complete all tasks.
    ///
    /// `tasks` — byte slice of uppercase letters ('A'–'Z').
    /// `cooldown` — mandatory gap between two executions of the same task type.
    pub fn solve(tasks: &[u8], cooldown: usize) -> usize {
        if tasks.is_empty() {
            return 0;
        }

        // Count frequency of each task type (A='A'=65 offset).
        let mut freq = [0u32; 26];
        for &t in tasks {
            let idx = (t.to_ascii_uppercase() - b'A') as usize;
            freq[idx] += 1;
        }

        let total = tasks.len();
        let max_freq = *freq.iter().max().unwrap() as usize;

        // Number of task types that share the maximum frequency.
        let max_count = freq.iter().filter(|&&f| f as usize == max_freq).count();

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Task scheduler: {} tasks, cooldown={cooldown}, max_freq={max_freq}, max_count={max_count}.",
                total
            ),
        );

        // Greedy formula:
        //   frames = (max_freq - 1) chunks of (cooldown + 1) slots, plus the final row of max_count tasks.
        //   If tasks are dense enough, no idles are needed and result = total.
        let min_intervals = ((max_freq - 1) * (cooldown + 1) + max_count).max(total);

        let idle_slots = min_intervals.saturating_sub(total);
        AgentLogger::log(
            AgentFeedback::Step,
            format!(
                "Idle slots needed: {idle_slots}; total intervals = {min_intervals}."
            ),
        );

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Minimum CPU intervals: {min_intervals}."),
        );
        min_intervals
    }

    /// Returns the schedule as a Vec of task bytes and b'_' for idle slots.
    pub fn build_schedule(tasks: &[u8], cooldown: usize) -> Vec<u8> {
        let mut freq = [0i32; 26];
        for &t in tasks {
            freq[(t.to_ascii_uppercase() - b'A') as usize] += 1;
        }

        let total_intervals = Self::solve(tasks, cooldown);
        let mut schedule = Vec::with_capacity(total_intervals);

        for _ in 0..total_intervals {
            // Pick the task with the highest remaining frequency that is not on cooldown.
            // Simple greedy: at each slot choose max-freq available task.
            // Track last-used position per task type for cooldown enforcement.
            // (Simplified O(26 * intervals) simulation.)
            let slot_idx = schedule.len();

            let best = (0..26usize)
                .filter(|&t| {
                    if freq[t] == 0 { return false; }
                    // Check cooldown: was this task used within the last `cooldown` slots?
                    schedule[slot_idx.saturating_sub(cooldown)..slot_idx]
                        .iter()
                        .all(|&s| s != b'A' + t as u8)
                })
                .max_by_key(|&t| freq[t]);

            if let Some(t) = best {
                freq[t] -= 1;
                schedule.push(b'A' + t as u8);
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Slot {slot_idx}: scheduled '{}'.", (b'A' + t as u8) as char),
                );
            } else {
                schedule.push(b'_'); // Idle
                AgentLogger::log(AgentFeedback::Step, format!("Slot {slot_idx}: idle."));
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Schedule built: {} interval(s).", schedule.len()),
        );
        schedule
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "greedy_algorithms.task_scheduler", description = "Use this for solving task scheduler problems. Trigger Keywords: task_scheduler, task scheduler, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_task_scheduler(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

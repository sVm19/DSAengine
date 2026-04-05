use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Gas Station
/// CATEGORY: greedy-algorithms
/// DESCRIPTION: Finds the only viable starting gas station for a circular route,
///              or returns None if the circuit is impossible — solved in one linear pass.
pub struct GasStation;

impl Complexity for GasStation {
    fn name(&self) -> &'static str {
        "Gas Station (Single-Pass Greedy Circuit)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) — One pass accumulates total surplus and identifies the greedy start candidate."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) — Only three scalar accumulators; gas and cost slices are read-only."
    }

    fn description(&self) -> &'static str {
        "If total_gas ≥ total_cost the circuit is feasible. The start is reset to i+1 whenever the running tank goes negative, because no station before that can be a valid start."
    }
}

impl GasStation {
    /// Returns the 0-based index of the starting gas station, or `None` if the circuit
    /// is impossible.
    ///
    /// `gas[i]`  — fuel gained at station i.
    /// `cost[i]` — fuel consumed travelling from station i to i+1.
    pub fn solve(gas: &[i64], cost: &[i64]) -> Option<usize> {
        assert_eq!(gas.len(), cost.len(), "gas and cost slices must have equal length");
        let n = gas.len();

        let mut total_surplus = 0i64;
        let mut running_tank = 0i64;
        let mut candidate_start = 0usize;

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Gas-station circuit check over {n} station(s)."),
        );

        for i in 0..n {
            let net = gas[i] - cost[i];
            total_surplus += net;
            running_tank += net;

            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Station {i}: net={net:+}, running_tank={running_tank}, candidate_start={candidate_start}."
                ),
            );

            if running_tank < 0 {
                // Cannot reach station i+1 from `candidate_start`; reset.
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Tank went negative at station {i}; resetting candidate_start to {}.",
                        i + 1
                    ),
                );
                candidate_start = i + 1;
                running_tank = 0;
            }
        }

        if total_surplus < 0 {
            AgentLogger::log(
                AgentFeedback::Warning,
                format!("Total surplus={total_surplus} < 0; circuit is impossible."),
            );
            return None;
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Circuit feasible (total_surplus={total_surplus}); start at station {candidate_start}."
            ),
        );
        Some(candidate_start)
    }

    /// Returns `true` and the total net fuel if the circuit starting from `start` completes.
    pub fn verify(gas: &[i64], cost: &[i64], start: usize) -> bool {
        let n = gas.len();
        let mut tank = 0i64;
        for step in 0..n {
            let i = (start + step) % n;
            tank += gas[i] - cost[i];
            if tank < 0 {
                AgentLogger::log(
                    AgentFeedback::Warning,
                    format!("Verification failed: tank went negative at station {i}."),
                );
                return false;
            }
        }
        AgentLogger::log(
            AgentFeedback::Success,
            format!("Verification passed: circuit from station {start} completes with tank={tank}."),
        );
        true
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "greedy_algorithms.gas_station", description = "Use this for solving gas station problems. Trigger Keywords: gas_station, gas station, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_gas_station(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

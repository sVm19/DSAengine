use crate::utils::complexity::{benchmark, Complexity};
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Iteration vs Recursion Analyzer
/// CATEGORY: dsa-fundamentals
/// DESCRIPTION: Compares iterative and recursive approaches for the same logic (e.g., Factorial).
pub struct IterationVsRecursion;

impl Complexity for IterationVsRecursion {
    fn name(&self) -> &'static str {
        "Iteration vs Recursion Analyzer"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Both typically scale linearly for basic problems."
    }

    fn space_complexity(&self) -> &'static str {
        "Iterative: O(1) | Recursive: O(n) (Call Stack)"
    }

    fn description(&self) -> &'static str {
        "Helps the AI Agent decide between Stack safety (Iteration) and Code clarity (Recursion)."
    }
}

impl IterationVsRecursion {
    /// Compares two closures (one iterative, one recursive) and reports the performance gap.
    pub fn compare<F1, F2, T>(iterative_fn: F1, recursive_fn: F2)
    where
        F1: FnOnce() -> T,
        F2: FnOnce() -> T,
    {
        AgentLogger::log(AgentFeedback::Info, "Benchmarking Iteration...");
        let (_, iter_report) = benchmark(iterative_fn);

        AgentLogger::log(AgentFeedback::Info, "Benchmarking Recursion...");
        let (_, recur_report) = benchmark(recursive_fn);

        println!("\n📊 [COMPARISON RESULT]");
        println!("  - Iteration Time: {:?}", iter_report.execution_time);
        println!("  - Recursion Time: {:?}", recur_report.execution_time);

        if iter_report.execution_time < recur_report.execution_time {
            AgentLogger::log(
                AgentFeedback::Success,
                "Iteration is faster (No stack overhead).",
            );
        } else {
            AgentLogger::log(
                AgentFeedback::Success,
                "Recursion is comparable (Compiler may have optimized tail calls).",
            );
        }
    }

    /// Warns the agent about the Risk of Stack Overflow in deep recursion.
    pub fn warn_stack_depth(depth: usize) {
        if depth > 1000 {
            AgentLogger::log(
                AgentFeedback::Error,
                format!(
                    "Stack depth {} exceeds safety threshold! Risk of StackOverflow.",
                    depth
                ),
            );
        }
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "dsa_fundamentals.iteration_vs_recursion", description = "Use this for solving iteration vs recursion problems. Trigger Keywords: iteration_vs_recursion, iteration vs recursion, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_iteration_vs_recursion(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

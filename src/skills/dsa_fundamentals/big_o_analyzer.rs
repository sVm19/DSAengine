use crate::utils::complexity::{benchmark, Complexity, PerformanceReport};
use crate::utils::logger::{AgentFeedback, AgentLogger};
use crate::utils::{api_docs, responses::*};
use std::time::Duration;

/// SKILL: Big O Analyzer
/// CATEGORY: dsa-fundamentals
/// DESCRIPTION: Estimates the Time and Space complexity of a given operation.
pub struct BigOAnalyzer;

impl Complexity for BigOAnalyzer {
    fn name(&self) -> &'static str {
        "Big O Complexity Analyzer"
    }

    fn time_complexity(&self) -> &'static str {
        "O(1) - The analyzer itself is a constant time hook."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1)"
    }

    fn description(&self) -> &'static str {
        "Wraps a function/closure to measure execution time and provide complexity hints."
    }
}

impl BigOAnalyzer {
    /// The primary execution hook for the AI Agent.
    /// It takes a closure `f`, runs it, and returns the result along with a performance report.
    pub fn run_analysis<F, T>(f: F) -> (T, PerformanceReport)
    where
        F: FnOnce() -> T,
    {
        AgentLogger::log(AgentFeedback::Info, "Starting performance analysis...");

        let (result, report) = benchmark(f);

        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Analysis complete. Execution Time: {:?}",
                report.execution_time
            ),
        );

        (result, report)
    }

    /// Provides a heuristic hint based on execution time.
    /// This helps the AI Agent decide if it needs to optimize further.
    pub fn get_efficiency_rating(duration: Duration) -> &'static str {
        if duration.as_micros() < 100 {
            "🚀 High Efficiency (Likely O(1) or O(log n))"
        } else if duration.as_millis() < 10 {
            "⚖️ Standard Efficiency (Likely O(n))"
        } else {
            "⚠️ Potential Bottleneck (Likely O(n log n) or higher)"
        }
    }
}

// --- AXUM WEB BRIDGE ---
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "dsa_fundamentals.big_o_analyzer",
    description = "Use this for solving big o analyzer problems. Trigger Keywords: big_o_analyzer, big o analyzer, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_big_o_analyzer(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BigOAnalyzerRequest {
    pub duration_micros: Option<u64>,
    pub operation_count: Option<u64>,
}

async fn handle_big_o_analyzer(payload: Value) -> DsaResult<ResultBox> {
    let req: BigOAnalyzerRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid BigOAnalyzerRequest: {e}"),
            hint: "Provide optional 'duration_micros' and 'operation_count'.".to_string(),
        })?;

    let duration = Duration::from_micros(req.duration_micros.unwrap_or(0));
    let rating = BigOAnalyzer::get_efficiency_rating(duration);
    let solver = BigOAnalyzer;

    Ok(ResultBox::success(json!({
        "duration_micros": req.duration_micros.unwrap_or(0),
        "operation_count": req.operation_count,
        "efficiency_rating": rating,
        "guidance": "Use measured growth across input sizes to confirm the asymptotic class."
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Big-O analysis metadata generated."))
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Fibonacci Visualizer
/// CATEGORY: dynamic-programming
/// DESCRIPTION: Computes Fibonacci numbers iteratively with step-by-step tracing,
///              and provides a fast O(log n) matrix-exponentiation variant.
pub struct FibonacciViz;

impl Complexity for FibonacciViz {
    fn name(&self) -> &'static str {
        "Fibonacci Visualizer (Iterative + Matrix Exp)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) iterative / O(log n) matrix exponentiation — both avoid redundant sub-calls."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) iterative — Two rolling variables; matrix method also O(1) ignoring output."
    }

    fn description(&self) -> &'static str {
        "Demonstrates bottom-up DP tabulation with two rolling vars, and an O(log n) closed-form via 2×2 matrix fast-power."
    }
}

impl FibonacciViz {
    /// Returns F(n) using O(1)-space rolling-pair iteration with trace hooks.
    pub fn solve(n: u64) -> u64 {
        if n == 0 { return 0; }
        if n == 1 { return 1; }

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Fibonacci iteration for F({n}); initialising F(0)=0, F(1)=1."),
        );

        let mut a = 0u64;
        let mut b = 1u64;

        for i in 2..=n {
            let next = a.saturating_add(b);
            AgentLogger::log(
                AgentFeedback::Step,
                format!("F({i}) = F({}) + F({}) = {a} + {b} = {next}.", i - 2, i - 1),
            );
            a = b;
            b = next;
        }

        AgentLogger::log(AgentFeedback::Success, format!("F({n}) = {b}."));
        b
    }

    /// Returns the first `count` Fibonacci numbers as a Vec, with trace.
    pub fn sequence(count: usize) -> Vec<u64> {
        let mut seq = Vec::with_capacity(count);
        if count == 0 { return seq; }

        seq.push(0u64);
        if count == 1 { return seq; }
        seq.push(1u64);

        for i in 2..count {
            let next = seq[i - 1].saturating_add(seq[i - 2]);
            AgentLogger::log(
                AgentFeedback::Step,
                format!("Appending F({i}) = {} to sequence.", next),
            );
            seq.push(next);
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Generated Fibonacci sequence of length {count}."),
        );
        seq
    }

    /// Returns F(n) in O(log n) time using 2×2 matrix fast exponentiation.
    ///
    /// Uses the identity:
    ///   [[1,1],[1,0]]^n = [[F(n+1), F(n)], [F(n), F(n-1)]]
    pub fn fast(n: u64) -> u64 {
        if n == 0 { return 0; }

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Matrix fast-power computing F({n}) in O(log {n}) multiplications."),
        );

        // Matrix stored as [a, b, c, d] representing [[a,b],[c,d]].
        let mut result = [1u64, 0, 0, 1]; // Identity
        let mut base = [1u64, 1, 1, 0];   // Fibonacci matrix
        let mut exp = n;

        while exp > 0 {
            if exp & 1 == 1 {
                result = Self::mat_mul(result, base);
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!("Multiplied result by base at exp bit; remaining exp={}.", exp >> 1),
                );
            }
            base = Self::mat_mul(base, base);
            exp >>= 1;
        }

        AgentLogger::log(AgentFeedback::Success, format!("Matrix-exp F({n}) = {}.", result[1]));
        result[1]
    }

    /// Multiplies two 2×2 matrices stored as flat [a,b,c,d] arrays (saturating arithmetic).
    fn mat_mul(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
        [
            a[0].saturating_mul(b[0]).saturating_add(a[1].saturating_mul(b[2])),
            a[0].saturating_mul(b[1]).saturating_add(a[1].saturating_mul(b[3])),
            a[2].saturating_mul(b[0]).saturating_add(a[3].saturating_mul(b[2])),
            a[2].saturating_mul(b[1]).saturating_add(a[3].saturating_mul(b[3])),
        ]
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "dynamic_programming.fibonacci_viz", description = "Use this for solving fibonacci viz problems. Trigger Keywords: fibonacci_viz, fibonacci viz, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_fibonacci_viz(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

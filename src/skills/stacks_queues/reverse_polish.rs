use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Evaluate Reverse Polish Notation (RPN)
/// CATEGORY: stacks-queues
/// DESCRIPTION: Evaluates an arithmetic expression written in postfix notation
///              using an operand stack.
pub struct ReversePolish;

impl Complexity for ReversePolish {
    fn name(&self) -> &'static str {
        "Reverse Polish Notation (Operand Stack)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Every token is processed exactly once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — In the worst-case (many operands followed by operators), the stack size scales linearly."
    }

    fn description(&self) -> &'static str {
        "Parses tokens left-to-right. Numbers are pushed to the stack. Operators pop the top two numbers, compute the expression, and push the result back."
    }
}

impl ReversePolish {
    /// Evaluates the RPN sequence.
    pub fn solve(tokens: &[&str]) -> i32 {
        let mut stack = Vec::with_capacity(tokens.len() / 2 + 1);

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Evaluating RPN sequence of {} token(s).", tokens.len()),
        );

        for &token in tokens {
            match token {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop().expect("Invalid RPN: missing operand");
                    let a = stack.pop().expect("Invalid RPN: missing operand");
                    let res = match token {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b, // Integer division truncates towards zero safely in Rust
                        _   => unreachable!(),
                    };
                    stack.push(res);
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Evaluated {a} {token} {b} = {res}"),
                    );
                }
                _ => {
                    let num = token.parse::<i32>().expect("Invalid RPN: non-integer operand");
                    stack.push(num);
                }
            }
        }

        let result = stack.pop().unwrap_or(0);
        AgentLogger::log(
            AgentFeedback::Success,
            format!("RPN sequence evaluated to {result}."),
        );
        result
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "reverse_polish", description = "Use this for solving reverse polish problems. Trigger Keywords: reverse_polish, reverse polish, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

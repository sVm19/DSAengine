use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Valid Parentheses
/// CATEGORY: stacks-queues
/// DESCRIPTION: Determines if an input string composed of brackets `()[]{}`
///              is valid based on nested closure pairs.
pub struct ValidParentheses;

impl Complexity for ValidParentheses {
    fn name(&self) -> &'static str {
        "Valid Parentheses (Bracket Stack)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Single pass over the string characters."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Stack size maps directly to the number of unclosed scopes."
    }

    fn description(&self) -> &'static str {
        "Pushes opening brackets onto the stack. For a closing bracket, checks if it perfectly matches the top of the stack. Non-matches or leftover unclosed brackets indicate invalidity."
    }
}

impl ValidParentheses {
    /// Returns `true` if the string of brackets is validly closed and nested.
    pub fn solve(s: &str) -> bool {
        let mut stack = Vec::with_capacity(s.len());

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Validating {} parentheses constraints.", s.len()),
        );

        for c in s.chars() {
            match c {
                '(' | '{' | '[' => {
                    stack.push(c);
                }
                ')' => {
                    if stack.pop() != Some('(') {
                        AgentLogger::log(AgentFeedback::Warning, "Mismatched or unexpected ')'.");
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        AgentLogger::log(AgentFeedback::Warning, "Mismatched or unexpected '}'.");
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        AgentLogger::log(AgentFeedback::Warning, "Mismatched or unexpected ']'.");
                        return false;
                    }
                }
                _ => {
                    AgentLogger::log(
                        AgentFeedback::Warning,
                        format!("Unexpected character '{c}' in scope stream."),
                    );
                    return false;
                }
            }
        }

        let valid = stack.is_empty();

        if valid {
            AgentLogger::log(
                AgentFeedback::Success,
                "All parentheses constraints successfully validated.",
            );
        } else {
            AgentLogger::log(
                AgentFeedback::Warning,
                format!("Unclosed scope detected ({} remaining).", stack.len()),
            );
        }

        valid
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "valid_parentheses",
    description = "Use this for solving valid parentheses problems. Trigger Keywords: valid_parentheses, valid parentheses, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_valid_parentheses(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_valid_parentheses(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        s: String,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 's' (string with brackets).".to_string(),
    })?;

    let result = {
        let valid = ValidParentheses::solve(&req.s);
        json!({ "valid": valid, "input": req.s })
    };

    let solver = ValidParentheses;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["validate"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Parentheses validation completed."))
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Asteroid Collision
/// CATEGORY: stacks-queues
/// DESCRIPTION: Simulates asteroid collisions where right-moving asteroids (>0)
///              can collide with left-moving ones (<0). Uses a stack to process state.
pub struct AsteroidCollision;

impl Complexity for AsteroidCollision {
    fn name(&self) -> &'static str {
        "Asteroid Collision (Stack-Based Simulation)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Each asteroid is pushed to and popped from the stack at most once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Stack holds at most N surviving asteroids."
    }

    fn description(&self) -> &'static str {
        "Iterates over asteroids. Positive asteroids push unhindered. Negative asteroids trigger a pop loop for any smaller positive asteroids, culminating in mutual destruction or survival based on mass."
    }
}

impl AsteroidCollision {
    /// Simulates collisions and returns the slice of surviving asteroids.
    pub fn solve(asteroids: &[i32]) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::with_capacity(asteroids.len());

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Asteroid collision simulation started for {} asteroids.", asteroids.len()),
        );

        for &ast in asteroids {
            let mut alive = true;

            // Collision condition: asteroid is moving left (-), top of stack is moving right (+)
            while alive && ast < 0 && !stack.is_empty() && *stack.last().unwrap() > 0 {
                let top = *stack.last().unwrap();
                let ast_mass = ast.abs();
                
                if top < ast_mass {
                    // Top asteroid is destroyed
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Asteroid {ast} destroys top asteroid {top}."),
                    );
                    stack.pop();
                } else if top == ast_mass {
                    // Both destroyed
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Asteroid {ast} and top asteroid {top} mutually destroy each other."),
                    );
                    stack.pop();
                    alive = false;
                } else {
                    // Incoming asteroid is destroyed
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Asteroid {ast} destroyed by top asteroid {top}."),
                    );
                    alive = false;
                }
            }

            if alive {
                stack.push(ast);
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Simulation complete. {} asteroid(s) survived.", stack.len()),
        );

        stack
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "asteroid_collision", description = "Use this for solving asteroid collision problems. Trigger Keywords: asteroid_collision, asteroid collision, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "temporary logic removed from auto-refactor; endpoint not yet restored"
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

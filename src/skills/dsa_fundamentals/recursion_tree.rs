use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Recursion Tree Visualizer
/// CATEGORY: dsa-fundamentals
/// DESCRIPTION: Generates a text-based tree representation of recursive calls.
pub struct RecursionTree;

impl Complexity for RecursionTree {
    fn name(&self) -> &'static str {
        "Recursion Tree Visualizer"
    }

    fn time_complexity(&self) -> &'static str {
        "O(2^n) - Visualizes exponential branching."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) - Depth of the call stack."
    }

    fn description(&self) -> &'static str {
        "Helps the AI Agent visualize the depth and branching factor of recursive logic."
    }
}

impl RecursionTree {
    /// Prints a visual "node" of the recursion tree.
    ///
    /// ### Usage:
    /// ```rust
    /// use dsaengine::skills::dsa_fundamentals::recursion_tree::RecursionTree;
    /// 
    /// RecursionTree::trace(0, "fib(5)");
    /// RecursionTree::trace(1, "fib(4)");
    /// ```
    pub fn trace(depth: usize, call_signature: &str) {
        let indent = "  ".repeat(depth);
        let prefix = if depth == 0 {
            "📍 ROOT:"
        } else {
            "└──"
        };

        println!("{}{} {}", indent, prefix, call_signature);
    }

    /// Logs the "Base Case" hit to signify the end of a branch.
    pub fn log_base_case(depth: usize, value: &str) {
        let indent = "  ".repeat(depth + 1);
        AgentLogger::log(
            AgentFeedback::Step,
            format!("{}🛑 Base Case reached: returning {}", indent, value),
        );
    }

    /// Visualizes the "Return" path as the recursion unwinds.
    pub fn trace_return(depth: usize, value: &str) {
        let indent = "  ".repeat(depth);
        println!("{} ⬆️ Result: {}", indent, value);
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "dsa_fundamentals.recursion_tree", description = "Use this for solving recursion tree problems. Trigger Keywords: recursion_tree, recursion tree, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_recursion_tree(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

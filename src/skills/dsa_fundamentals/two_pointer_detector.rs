use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Two Pointer Pattern Detector
/// CATEGORY: dsa-fundamentals
/// DESCRIPTION: Identifies if a problem can be solved using the Two-Pointer technique (e.g., Sorted Arrays, Palindromes).
pub struct TwoPointerDetector;

impl Complexity for TwoPointerDetector {
    fn name(&self) -> &'static str {
        "Two Pointer Pattern Detector"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n) - Single pass through the data."
    }

    fn space_complexity(&self) -> &'static str {
        "O(1) - Constant space for pointers."
    }

    fn description(&self) -> &'static str {
        "Heuristic tool for AI Agents to detect Two-Pointer compatibility in sorted structures."
    }
}

impl TwoPointerDetector {
    /// Evaluates a problem's constraints to see if Two Pointers are applicable.
    pub fn evaluate_compatibility(is_sorted: bool, search_target: bool) -> bool {
        if is_sorted && search_target {
            AgentLogger::log(
                AgentFeedback::Success,
                "Two-Pointer Match: High probability (Sorted Search).",
            );
            return true;
        }

        AgentLogger::log(
            AgentFeedback::Info,
            "Two-Pointer Match: Low probability (Unsorted or non-search).",
        );
        false
    }

    /// Visualizes the pointer movement for the AI/User.
    pub fn trace_pointers(left: usize, right: usize, left_val: i32, right_val: i32) {
        println!(
            "  [L: {} (Val: {})] <---> [R: {} (Val: {})]",
            left, left_val, right, right_val
        );

        if left >= right {
            AgentLogger::log(
                AgentFeedback::Step,
                "Pointers crossed or met. Search complete.",
            );
        }
    }

    /// Explains why this pattern reduces complexity from O(n^2) to O(n).
    pub fn explain_efficiency() {
        println!("⚡ Optimization: Eliminates nested loops by moving pointers toward each other based on conditions.");
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "dsa_fundamentals.two_pointer_detector",
    description = "Use this for solving two pointer detector problems. Trigger Keywords: two_pointer_detector, two pointer detector, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_two_pointer_detector(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct TwoPointerDetectorRequest {
    pub is_sorted: bool,
    pub search_target: bool,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub left_val: Option<i32>,
    pub right_val: Option<i32>,
}

async fn handle_two_pointer_detector(payload: Value) -> DsaResult<ResultBox> {
    let req: TwoPointerDetectorRequest =
        serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
            message: format!("Invalid TwoPointerDetectorRequest: {e}"),
            hint: "Provide 'is_sorted' and 'search_target' booleans; pointer trace fields are optional.".to_string(),
        })?;

    let compatible = TwoPointerDetector::evaluate_compatibility(req.is_sorted, req.search_target);
    if let (Some(left), Some(right), Some(left_val), Some(right_val)) =
        (req.left, req.right, req.left_val, req.right_val)
    {
        TwoPointerDetector::trace_pointers(left, right, left_val, right_val);
    }

    let solver = TwoPointerDetector;
    Ok(ResultBox::success(json!({
        "two_pointer_compatible": compatible,
        "reason": if compatible { "Sorted target search can move inward from both ends." } else { "Two-pointer search is weaker when data is unsorted or no target relation exists." },
        "trace": {
            "left": req.left,
            "right": req.right,
            "left_val": req.left_val,
            "right_val": req.right_val
        }
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Two-pointer compatibility analysis completed."))
}

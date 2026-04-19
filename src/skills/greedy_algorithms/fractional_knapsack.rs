use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Fractional Knapsack
/// CATEGORY: greedy-algorithms
/// DESCRIPTION: Maximises total value by filling a capacity-limited knapsack with items
///              that can be taken in fractions, sorted greedily by value-per-unit-weight.
pub struct FractionalKnapsack;

impl Complexity for FractionalKnapsack {
    fn name(&self) -> &'static str {
        "Fractional Knapsack (Value/Weight Ratio Sort)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n log n) — Sorting by value/weight ratio dominates; the fill pass is O(n)."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) — Sorted index array; item slice is read-only."
    }

    fn description(&self) -> &'static str {
        "Sorts items by descending value-per-unit-weight; greedily takes as much of each item as fits, taking a fraction of the last item if needed."
    }
}

impl FractionalKnapsack {
    /// Returns the maximum value achievable and the fraction taken of each item.
    ///
    /// `items` — slice of `(value, weight)` pairs.
    /// `capacity` — maximum weight the knapsack can hold.
    ///
    /// Returns `(total_value, fractions)` where `fractions[i]` ∈ [0.0, 1.0].
    pub fn solve(items: &[(f64, f64)], capacity: f64) -> (f64, Vec<f64>) {
        if items.is_empty() || capacity <= 0.0 {
            return (0.0, vec![0.0; items.len()]);
        }

        // Build sorted index array by value/weight ratio (descending), zero-copy on items.
        let mut order: Vec<usize> = (0..items.len()).collect();
        order.sort_unstable_by(|&a, &b| {
            let ratio_a = items[a].0 / items[a].1;
            let ratio_b = items[b].0 / items[b].1;
            ratio_b
                .partial_cmp(&ratio_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Fractional knapsack: {} item(s), capacity={capacity:.2}.",
                items.len()
            ),
        );

        let mut fractions = vec![0.0f64; items.len()];
        let mut remaining = capacity;
        let mut total_value = 0.0f64;

        for idx in order {
            let (value, weight) = items[idx];
            if weight <= 0.0 {
                continue;
            }

            let take = weight.min(remaining);
            let fraction = take / weight;
            fractions[idx] = fraction;
            total_value += value * fraction;
            remaining -= take;

            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Item {idx} (v={value:.2}, w={weight:.2}, ratio={:.4}): took {:.2}/{:.2} → gained {:.4}; remaining capacity={remaining:.4}.",
                    value / weight, take, weight, value * fraction
                ),
            );

            if remaining <= 0.0 {
                AgentLogger::log(AgentFeedback::Step, "Knapsack full; stopping early.");
                break;
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Fractional knapsack total value: {total_value:.6}."),
        );
        (total_value, fractions)
    }

    /// Returns only the maximum achievable value (faster path, no fraction tracking).
    pub fn max_value(items: &[(f64, f64)], capacity: f64) -> f64 {
        Self::solve(items, capacity).0
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "fractional_knapsack",
    description = "Use this for optimization problems involving limited capacity, resource allocation, or 'packing' items for maximum value. Trigger Keywords: fractional_knapsack, fractional knapsack, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_fractional_knapsack(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_fractional_knapsack(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        items: Vec<(f64, f64)>,
        capacity: f64,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'items' as [(value, weight)] and 'capacity'.".to_string(),
    })?;

    let result = {
        let max_value = FractionalKnapsack::solve(&req.items, req.capacity);
        json!({ "max_value": max_value, "capacity": req.capacity })
    };

    let solver = FractionalKnapsack;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["optimize"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Fractional knapsack completed."))
}

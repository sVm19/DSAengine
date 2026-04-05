use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Graph Visualizer
/// CATEGORY: graphs
/// DESCRIPTION: Produces human-readable ASCII representations of a graph's
///              adjacency list, edge table, and degree distribution for debugging and tracing.
pub struct Visualizer;

impl Complexity for Visualizer {
    fn name(&self) -> &'static str {
        "Graph Visualizer (ASCII Adjacency + Degree Table)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(V + E) — Iterates every node and every edge once to build the display strings."
    }

    fn space_complexity(&self) -> &'static str {
        "O(V + E) — Output string proportional to total nodes and edges."
    }

    fn description(&self) -> &'static str {
        "Renders an adjacency list, a sorted edge table, and a degree-distribution summary as formatted strings for agent trace output."
    }
}

impl Visualizer {
    /// Returns a formatted adjacency-list string for the given graph.
    pub fn adjacency_list(adj: &[Vec<usize>]) -> String {
        let mut out = String::from("=== Adjacency List ===\n");
        for (u, neighbours) in adj.iter().enumerate() {
            let nb_str: Vec<String> = neighbours.iter().map(|v| v.to_string()).collect();
            out.push_str(&format!("  [{u:>3}] → [{}]\n", nb_str.join(", ")));
        }
        AgentLogger::log(AgentFeedback::Info, format!("Rendered adjacency list for {} node(s).", adj.len()));
        out
    }

    /// Returns a sorted edge table as a formatted string.
    pub fn edge_table(adj: &[Vec<usize>]) -> String {
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for (u, neighbours) in adj.iter().enumerate() {
            for &v in neighbours {
                edges.push((u, v));
            }
        }
        edges.sort_unstable();

        let mut out = String::from("=== Edge Table ===\n");
        out.push_str(&format!("  {:>6}  {:>6}\n", "From", "To"));
        out.push_str("  ------  ------\n");
        for (u, v) in &edges {
            out.push_str(&format!("  {:>6}  {:>6}\n", u, v));
        }
        out.push_str(&format!("  Total: {} edge(s)\n", edges.len()));

        AgentLogger::log(
            AgentFeedback::Step,
            format!("Rendered edge table: {} edge(s).", edges.len()),
        );
        out
    }

    /// Returns a degree-distribution summary.
    pub fn degree_stats(adj: &[Vec<usize>]) -> String {
        if adj.is_empty() {
            return String::from("Empty graph.\n");
        }

        let degrees: Vec<usize> = adj.iter().map(|nb| nb.len()).collect();
        let max_deg = *degrees.iter().max().unwrap_or(&0);
        let min_deg = *degrees.iter().min().unwrap_or(&0);
        let avg_deg = degrees.iter().sum::<usize>() as f64 / degrees.len() as f64;

        let mut out = String::from("=== Degree Distribution ===\n");
        out.push_str(&format!("  Nodes : {}\n", adj.len()));
        out.push_str(&format!("  Min   : {min_deg}\n"));
        out.push_str(&format!("  Max   : {max_deg}\n"));
        out.push_str(&format!("  Avg   : {avg_deg:.2}\n"));

        // Histogram buckets (max 10 buckets).
        let bucket_count = max_deg.min(10) + 1;
        let bucket_size = (max_deg / bucket_count).max(1);
        let mut buckets = vec![0usize; bucket_count + 1];
        for &d in &degrees {
            buckets[(d / bucket_size).min(bucket_count)] += 1;
        }

        out.push_str("  Histogram:\n");
        for (i, &count) in buckets.iter().enumerate() {
            let lo = i * bucket_size;
            let hi = lo + bucket_size - 1;
            let bar: String = "#".repeat(count);
            out.push_str(&format!("  {:>3}-{:<3} |{bar}| {count}\n", lo, hi));
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Degree stats: min={min_deg}, max={max_deg}, avg={avg_deg:.2}."),
        );
        out
    }

    /// Emits the adjacency list, edge table, and degree stats to the agent logger.
    pub fn print_all(adj: &[Vec<usize>]) {
        let output = format!(
            "{}\n{}\n{}",
            Self::adjacency_list(adj),
            Self::edge_table(adj),
            Self::degree_stats(adj)
        );
        AgentLogger::log(AgentFeedback::Info, output);
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "graphs.visualizer", description = "Use this for solving visualizer problems. Trigger Keywords: visualizer, visualizer, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_visualizer(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

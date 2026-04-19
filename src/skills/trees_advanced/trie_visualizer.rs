use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Trie Visualizer
/// CATEGORY: trees-advanced
/// DESCRIPTION: Inspects states of array-backed Trie structures outputting contiguous
///              hierarchical paths programmatically.
pub struct TrieVisualizer;

impl Complexity for TrieVisualizer {
    fn name(&self) -> &'static str {
        "Array Trie Visualizer"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N) — Steps across the unified arrays precisely once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N) — Requisite memory boundaries parsing entire trie sub-dictionaries."
    }

    fn description(&self) -> &'static str {
        "Because Tries hold indices instead of pointers, we just recursively (or using an explicit DFS stack) dump prefix states natively."
    }
}

impl TrieVisualizer {
    pub fn to_string(children: &[[usize; 26]], is_end: &[bool]) -> String {
        let mut out = String::from("\n[Trie Dictionary State]\n");
        let mut stack = vec![(0, String::new())];

        AgentLogger::log(
            AgentFeedback::Info,
            "Traversing array-backed Trie nodes to extract prefix dicts.",
        );

        while let Some((node_idx, prefix)) = stack.pop() {
            if is_end[node_idx] {
                out.push_str(&format!("* {prefix} (valid word)\n"));
            }

            // Push backwards to pop alphabetically (0 to 25 -> Z to A pushed sequentially)
            for i in (0..26).rev() {
                let next_idx = children[node_idx][i];
                if next_idx != usize::MAX {
                    let mut next_prefix = prefix.clone();
                    next_prefix.push((i as u8 + b'a') as char);
                    stack.push((next_idx, next_prefix));
                }
            }
        }

        out
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "trie_visualizer",
    description = "Use this for solving trie visualizer problems. Trigger Keywords: trie_visualizer, trie visualizer, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_trie_visualizer(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_trie_visualizer(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        words: Vec<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'words' to visualize as a trie.".to_string(),
    })?;

    let result = {
        // Build trie structure inline (same format TrieVisualizer::to_string expects)
        let mut children: Vec<[usize; 26]> = vec![[usize::MAX; 26]];
        let mut is_end: Vec<bool> = vec![false];
        for word in &req.words {
            let mut curr = 0;
            for c in word.bytes() {
                let idx = (c - b'a') as usize;
                if children[curr][idx] == usize::MAX {
                    children.push([usize::MAX; 26]);
                    is_end.push(false);
                    let new_node = children.len() - 1;
                    children[curr][idx] = new_node;
                }
                curr = children[curr][idx];
            }
            is_end[curr] = true;
        }
        let viz = TrieVisualizer::to_string(&children, &is_end);
        json!({ "visualization": viz, "word_count": req.words.len(), "node_count": children.len() })
    };

    let solver = TrieVisualizer;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["visualize"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("TrieVisualizer completed."))
}

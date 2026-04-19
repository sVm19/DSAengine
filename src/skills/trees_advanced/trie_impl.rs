use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Trie (Prefix Tree)
/// CATEGORY: trees-advanced
/// DESCRIPTION: Efficient string storage and prefix evaluation utilizing an array-backed
///              arena graph mitigating all traditional pointer/Object allocation bounds.
pub struct TrieImpl;

impl Complexity for TrieImpl {
    fn name(&self) -> &'static str {
        "Array-Backed Trie (Null-State Arena)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(W) — Time bound rigidly to Word Length (W) regardless of dictionary dimension."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N * W * 26) — Worst-case discrete tree limits before dictionary branch compaction."
    }

    fn description(&self) -> &'static str {
        "Exposes flat 1D arrays: `next[node][char] -> child_idx`. Strings iteratively tunnel through index matrices completely resolving without single instances of nested recursion."
    }
}

pub struct Trie {
    pub children: Vec<[usize; 26]>,
    pub is_end: Vec<bool>,
}

impl Trie {
    pub fn new() -> Self {
        AgentLogger::log(AgentFeedback::Info, "Instantiated Array-Backed Arena Trie.");
        Self {
            children: vec![[usize::MAX; 26]], // root node
            is_end: vec![false],
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut curr = 0; // Root is always 0
        AgentLogger::log(AgentFeedback::Step, format!("Trie Inserting: '{word}'."));

        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            if self.children[curr][idx] == usize::MAX {
                self.children.push([usize::MAX; 26]);
                self.is_end.push(false);
                let new_node = self.children.len() - 1;
                self.children[curr][idx] = new_node;
            }
            curr = self.children[curr][idx];
        }
        self.is_end[curr] = true;
    }

    pub fn search(&self, word: &str) -> bool {
        if let Some(curr) = self.find_node(word) {
            self.is_end[curr]
        } else {
            false
        }
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.find_node(prefix).is_some()
    }

    fn find_node(&self, word: &str) -> Option<usize> {
        let mut curr = 0;
        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            curr = self.children[curr][idx];
            if curr == usize::MAX {
                return None;
            }
        }
        Some(curr)
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "trie_impl",
    description = "Use this for solving trie impl problems. Trigger Keywords: trie_impl, trie impl, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_trie_impl(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_trie_impl(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        words: Vec<String>,
        search: Option<String>,
        prefix: Option<String>,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'words'. Optional 'search', 'prefix'. Mode: 'search'|'prefix'|'build'."
            .to_string(),
    })?;

    let result = {
        let mut trie = Trie::new();
        for w in &req.words {
            trie.insert(w);
        }
        match req.mode.as_deref().unwrap_or("build") {
            "search" => {
                let w = req.search.as_deref().unwrap_or("");
                json!({"mode":"search","word":w,"found":trie.search(w)})
            }
            "prefix" => {
                let p = req.prefix.as_deref().unwrap_or("");
                json!({"mode":"prefix","prefix":p,"has_prefix":trie.starts_with(p)})
            }
            _ => json!({"mode":"build","word_count":req.words.len()}),
        }
    };

    let solver = TrieImpl;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["build", "search", "prefix"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("TrieImpl completed."))
}

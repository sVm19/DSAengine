use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::BTreeMap;

/// SKILL: Trie Autocomplete
/// CATEGORY: advanced-topics
/// DESCRIPTION: Builds a trie-backed autocomplete index over borrowed strings.
pub struct TrieAutocomplete;
pub struct TrieIndex<'a> {
    words: &'a [&'a str],
    nodes: Vec<TrieNode>,
}

#[derive(Default)]
struct TrieNode {
    children: BTreeMap<u8, usize>,
    terminal_words: Vec<usize>,
}

impl Complexity for TrieAutocomplete {
    fn name(&self) -> &'static str {
        "Trie Autocomplete"
    }

    fn time_complexity(&self) -> &'static str {
        "O(total characters * log sigma) build and O(prefix + output) traversal per query."
    }

    fn space_complexity(&self) -> &'static str {
        "O(total characters) - Stores one trie node per created prefix edge."
    }

    fn description(&self) -> &'static str {
        "Traverses shared prefixes once, then walks the matching subtrie to enumerate completions."
    }
}

impl TrieAutocomplete {
    pub fn solve<'a>(words: &'a [&'a str]) -> TrieIndex<'a> {
        Self::build(words)
    }

    pub fn build<'a>(words: &'a [&'a str]) -> TrieIndex<'a> {
        let mut nodes = vec![TrieNode::default()];

        for (word_index, &word) in words.iter().enumerate() {
            let mut node = 0usize;
            for &byte in word.as_bytes() {
                let next = if let Some(&child) = nodes[node].children.get(&byte) {
                    child
                } else {
                    let child = nodes.len();
                    nodes.push(TrieNode::default());
                    nodes[node].children.insert(byte, child);
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!(
                            "Created trie edge '{}' from node {} to {}.",
                            char::from(byte),
                            node,
                            child
                        ),
                    );
                    child
                };
                node = next;
            }

            nodes[node].terminal_words.push(word_index);
            AgentLogger::log(
                AgentFeedback::Step,
                format!("Registered terminal word '{}' at node {}.", word, node),
            );
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Trie autocomplete index built with {} nodes.", nodes.len()),
        );

        TrieIndex { words, nodes }
    }
}

impl<'a> TrieIndex<'a> {
    pub fn suggest(&self, prefix: &str, limit: usize) -> Vec<&'a str> {
        if limit == 0 {
            return Vec::new();
        }

        let mut node = 0usize;
        for &byte in prefix.as_bytes() {
            let Some(&child) = self.nodes[node].children.get(&byte) else {
                AgentLogger::log(
                    AgentFeedback::Warning,
                    format!("Prefix '{}' is absent from the trie.", prefix),
                );
                return Vec::new();
            };
            node = child;
            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Followed prefix edge '{}' into node {}.",
                    char::from(byte),
                    node
                ),
            );
        }

        let mut results = Vec::new();
        let mut stack = vec![node];
        while let Some(current) = stack.pop() {
            for &word_index in &self.nodes[current].terminal_words {
                results.push(self.words[word_index]);
                if results.len() == limit {
                    AgentLogger::log(
                        AgentFeedback::Success,
                        format!(
                            "Autocomplete produced {} suggestions for prefix '{}'.",
                            results.len(),
                            prefix
                        ),
                    );
                    return results;
                }
            }

            let children = self.nodes[current]
                .children
                .values()
                .copied()
                .collect::<Vec<_>>();
            for child in children.into_iter().rev() {
                stack.push(child);
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Autocomplete exhausted the subtrie with {} suggestion(s).",
                results.len()
            ),
        );
        results
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut node = 0usize;
        for &byte in word.as_bytes() {
            let Some(&child) = self.nodes[node].children.get(&byte) else {
                return false;
            };
            node = child;
        }

        self.nodes[node]
            .terminal_words
            .iter()
            .any(|&index| self.words[index] == word)
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "trie_autocomplete",
    description = "Use this for solving trie autocomplete problems. Trigger Keywords: trie_autocomplete, trie autocomplete, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_trie_autocomplete(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_trie_autocomplete(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        words: Vec<String>,
        prefix: Option<String>,
        limit: Option<usize>,
        search: Option<String>,
        mode: Option<String>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'words'. Optional 'prefix' for suggestions, 'search' for lookup, 'limit' for max results. 'mode': 'suggest' | 'contains'.".to_string(),
    })?;

    let result = match req.mode.as_deref().unwrap_or("") {
        "contains" => {
            let word_refs: Vec<&str> = req.words.iter().map(|s| s.as_str()).collect();
            let trie = TrieAutocomplete::build(&word_refs);
            let word = req.search.as_deref().unwrap_or("");
            let found = trie.contains(word);
            json!({ "mode": "contains", "word": word, "found": found })
        }
        _ => {
            let word_refs: Vec<&str> = req.words.iter().map(|s| s.as_str()).collect();
            let trie = TrieAutocomplete::build(&word_refs);
            let prefix = req.prefix.as_deref().unwrap_or("");
            let suggestions = trie.suggest(prefix, req.limit.unwrap_or(10));
            json!({ "mode": "suggest", "prefix": prefix, "suggestions": suggestions })
        }
    };

    let solver = TrieAutocomplete;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["suggest", "contains"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Trie autocomplete completed."))
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::{HashMap, HashSet, VecDeque};

/// SKILL: Word Ladder
/// CATEGORY: graphs
/// DESCRIPTION: Finds the shortest transformation sequence from `begin` to `end`
///              where each step changes exactly one letter and every intermediate word
///              must be in the word list — solved as BFS on an implicit word graph.
pub struct WordLadder;

impl Complexity for WordLadder {
    fn name(&self) -> &'static str {
        "Word Ladder (BFS Implicit Graph / One-Letter Mutations)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(N * L²) — N words × L characters per word × L mutation positions; each word dequeued once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(N * L) — Word set lookup O(1); BFS queue and visited set hold at most N strings."
    }

    fn description(&self) -> &'static str {
        "At each BFS step, generates all L*(26-1) one-letter mutations of the current word and checks them against the word set, guaranteeing the shortest transformation chain."
    }
}

impl WordLadder {
    /// Returns the length of the shortest transformation sequence from `begin` to `end`,
    /// or 0 if no sequence exists.
    pub fn solve(begin: &str, end: &str, word_list: &[&str]) -> usize {
        if begin.is_empty() || end.is_empty() { return 0; }

        let mut word_set: HashSet<&str> = word_list.iter().copied().collect();

        if !word_set.contains(end) {
            AgentLogger::log(AgentFeedback::Warning, format!("End word \"{end}\" not in word list."));
            return 0;
        }

        // BFS on the implicit graph of one-letter mutations.
        let mut queue: VecDeque<(String, usize)> = VecDeque::new();
        queue.push_back((begin.to_string(), 1));
        word_set.remove(begin); // Mark begin as visited by removing from set.

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Word-ladder BFS: \"{begin}\" → \"{end}\"; {} word(s) in dict.", word_list.len()),
        );

        while let Some((word, steps)) = queue.pop_front() {
            let chars: Vec<u8> = word.bytes().collect();

            for pos in 0..chars.len() {
                let original = chars[pos];
                let mut mutated = chars.clone();

                for c in b'a'..=b'z' {
                    if c == original { continue; }
                    mutated[pos] = c;

                    // SAFE: all bytes are ASCII a-z.
                    let candidate = std::str::from_utf8(&mutated).unwrap();

                    if candidate == end {
                        AgentLogger::log(
                            AgentFeedback::Success,
                            format!("Reached \"{end}\" in {steps} step(s).", steps = steps + 1),
                        );
                        return steps + 1;
                    }

                    if word_set.contains(candidate) {
                        AgentLogger::log(
                            AgentFeedback::Step,
                            format!("Enqueuing mutation \"{candidate}\" at step {}.", steps + 1),
                        );
                        queue.push_back((candidate.to_string(), steps + 1));
                        word_set.remove(candidate);
                    }
                }
            }
        }

        AgentLogger::log(
            AgentFeedback::Warning,
            format!("No transformation sequence from \"{begin}\" to \"{end}\"."),
        );
        0
    }

    /// Returns ALL shortest transformation sequences (ladders) as Vec<Vec<String>>.
    pub fn find_all_ladders(
        begin: &str,
        end: &str,
        word_list: &[&str],
    ) -> Vec<Vec<String>> {
        let word_set: HashSet<&str> = word_list.iter().copied().collect();
        if !word_set.contains(end) { return Vec::new(); }

        // BFS to build a layer-by-layer parents map.
        let mut parents: HashMap<String, Vec<String>> = HashMap::new();
        let mut current_layer: HashSet<String> = [begin.to_string()].into();
        let mut visited: HashSet<String> = current_layer.clone();
        let mut found = false;

        while !current_layer.is_empty() && !found {
            let mut next_layer: HashSet<String> = HashSet::new();

            for word in &current_layer {
                let chars: Vec<u8> = word.bytes().collect();
                for pos in 0..chars.len() {
                    let original = chars[pos];
                    let mut mutated = chars.clone();
                    for c in b'a'..=b'z' {
                        if c == original { continue; }
                        mutated[pos] = c;
                        let candidate = String::from_utf8(mutated.clone()).unwrap();
                        if word_set.contains(candidate.as_str()) && !visited.contains(&candidate) {
                            next_layer.insert(candidate.clone());
                            parents.entry(candidate.clone()).or_default().push(word.clone());
                            if candidate == end { found = true; }
                        }
                    }
                }
            }
            visited.extend(next_layer.iter().cloned());
            current_layer = next_layer;
        }

        if !found { return Vec::new(); }

        // DFS traceback of parents to build all paths.
        let mut results: Vec<Vec<String>> = Vec::new();
        let mut path = vec![end.to_string()];

        fn dfs(
            node: &str, begin: &str,
            parents: &HashMap<String, Vec<String>>,
            path: &mut Vec<String>,
            results: &mut Vec<Vec<String>>,
        ) {
            if node == begin {
                let mut p = path.clone();
                p.reverse();
                results.push(p);
                return;
            }
            if let Some(pars) = parents.get(node) {
                for parent in pars {
                    path.push(parent.clone());
                    dfs(parent, begin, parents, path, results);
                    path.pop();
                }
            }
        }

        dfs(end, begin, &parents, &mut path, &mut results);

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Found {} shortest ladder(s) of length {}.", results.len(),
                results.first().map(|p| p.len()).unwrap_or(0)),
        );
        results
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "graphs.word_ladder", description = "Use this for solving word ladder problems. Trigger Keywords: graph, word_ladder, shortest path, traversal. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_word_ladder(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::{HashMap, VecDeque};

/// SKILL: Alien Dictionary
/// CATEGORY: graphs
/// DESCRIPTION: Derives the character ordering of an alien language from a sorted list
///              of words by building a directed constraint graph and topologically sorting it.
pub struct AlienDictionary;

impl Complexity for AlienDictionary {
    fn name(&self) -> &'static str {
        "Alien Dictionary (Constraint Graph + Topological Sort)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(C + N * L) — C unique chars, N words, L max word length; edge extraction is O(N*L); Kahn's BFS is O(C + edges)."
    }

    fn space_complexity(&self) -> &'static str {
        "O(C²) — At most C² directed edges between C distinct characters."
    }

    fn description(&self) -> &'static str {
        "Compares adjacent words character-by-character to extract ordering constraints; Kahn's BFS on the constraint graph produces a valid alien alphabet, or empty string if a cycle exists."
    }
}

impl AlienDictionary {
    /// Returns the alien character ordering as a `String`, or `""` if the input is inconsistent.
    pub fn solve(words: &[&str]) -> String {
        // Collect all unique characters and build adjacency + in-degree.
        let mut adj: HashMap<u8, Vec<u8>> = HashMap::new();
        let mut in_degree: HashMap<u8, usize> = HashMap::new();

        for word in words {
            for &b in word.as_bytes() {
                adj.entry(b).or_default(); // Ensure every char appears in the map.
                in_degree.entry(b).or_insert(0);
            }
        }

        let num_chars = adj.len();

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Alien dictionary: {} word(s), {} unique char(s).",
                words.len(), num_chars
            ),
        );

        // Extract ordering constraints by comparing adjacent words.
        for pair in words.windows(2) {
            let (a, b) = (pair[0].as_bytes(), pair[1].as_bytes());
            let common = a.len().min(b.len());

            // Edge case: if a is longer than b and b is a prefix of a, the list is invalid.
            if a.len() > b.len() && a[..common] == b[..common] {
                AgentLogger::log(
                    AgentFeedback::Warning,
                    format!("Invalid order: \"{}\" comes before \"{}\" but is a prefix extension.", pair[0], pair[1]),
                );
                return String::new();
            }

            for i in 0..common {
                if a[i] != b[i] {
                    // a[i] must come before b[i].
                    adj.entry(a[i]).or_default().push(b[i]);
                    *in_degree.entry(b[i]).or_insert(0) += 1;
                    AgentLogger::log(
                        AgentFeedback::Step,
                        format!("Constraint: '{}' < '{}'.", a[i] as char, b[i] as char),
                    );
                    break; // Only the first differing character matters.
                }
            }
        }

        // Kahn's BFS topological sort.
        let mut queue: VecDeque<u8> = in_degree
            .iter()
            .filter(|(_, &ind)| ind == 0)
            .map(|(&c, _)| c)
            .collect();

        // Stable sort for deterministic output.
        let mut queue_vec: Vec<u8> = queue.drain(..).collect();
        queue_vec.sort_unstable();
        let mut queue: VecDeque<u8> = queue_vec.into();

        let mut result = Vec::with_capacity(num_chars);

        while let Some(c) = queue.pop_front() {
            result.push(c);
            if let Some(neighbours) = adj.get(&c) {
                let mut nexts: Vec<u8> = Vec::new();
                for &nb in neighbours {
                    let ind = in_degree.get_mut(&nb).unwrap();
                    *ind -= 1;
                    if *ind == 0 {
                        nexts.push(nb);
                    }
                }
                nexts.sort_unstable();
                for nb in nexts {
                    queue.push_back(nb);
                }
            }
        }

        if result.len() < num_chars {
            AgentLogger::log(
                AgentFeedback::Warning,
                "Cycle detected in character constraints; no valid alien alphabet.",
            );
            return String::new();
        }

        let ordering = String::from_utf8(result).unwrap_or_default();
        AgentLogger::log(
            AgentFeedback::Success,
            format!("Alien alphabet ordering: \"{ordering}\"."),
        );
        ordering
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "graphs.alien_dictionary", description = "Use this for solving alien dictionary problems. Trigger Keywords: graph, alien_dictionary, shortest path, traversal. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_alien_dictionary(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

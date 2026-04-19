use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::BinaryHeap;

/// SKILL: Huffman Coding
/// CATEGORY: greedy-algorithms
/// DESCRIPTION: Builds an optimal prefix-free binary code for a symbol frequency table
///              using a min-heap to repeatedly merge the two lowest-frequency nodes.
pub struct HuffmanCoding;

impl Complexity for HuffmanCoding {
    fn name(&self) -> &'static str {
        "Huffman Coding (Min-Heap Symbol Merge)"
    }

    fn time_complexity(&self) -> &'static str {
        "O(n log n) — n symbols each pushed and popped from the min-heap at most once."
    }

    fn space_complexity(&self) -> &'static str {
        "O(n) — Heap holds at most n nodes; code table has n entries."
    }

    fn description(&self) -> &'static str {
        "Builds the Huffman tree bottom-up: repeatedly extract the two minimum-frequency nodes and merge them into a parent whose frequency is their sum. Traverses the final tree to assign bit strings."
    }
}

/// A node in the Huffman tree.
/// Using an arena (Vec of HuffmanNode) to avoid `Box` heap allocation.
struct HuffmanNode {
    freq: u64,
    symbol: Option<u8>,  // `Some(byte)` for leaf nodes
    left: Option<usize>, // index into the arena
    right: Option<usize>,
}

/// Heap entry for ordering by frequency (min-heap via Reverse).
#[derive(Eq, PartialEq)]
struct HeapEntry {
    freq: u64,
    node_idx: usize,
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Min-heap: lower frequency has higher priority.
        other
            .freq
            .cmp(&self.freq)
            .then(other.node_idx.cmp(&self.node_idx))
    }
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl HuffmanCoding {
    /// Builds the Huffman code table for the given symbol frequencies.
    ///
    /// `frequencies` — slice of `(symbol_byte, frequency)` pairs.
    /// Returns a `Vec<(u8, String)>` of `(symbol, code_bits)` sorted by code length then symbol.
    pub fn build_codes(frequencies: &[(u8, u64)]) -> Vec<(u8, String)> {
        let n = frequencies.len();
        if n == 0 {
            return Vec::new();
        }

        AgentLogger::log(
            AgentFeedback::Info,
            format!("Huffman coding: {n} symbol(s)."),
        );

        // Arena-allocated tree.
        let mut arena: Vec<HuffmanNode> = Vec::with_capacity(2 * n);

        // Seed the arena with leaf nodes.
        let mut heap: BinaryHeap<HeapEntry> = BinaryHeap::new();
        for &(sym, freq) in frequencies {
            let idx = arena.len();
            arena.push(HuffmanNode {
                freq,
                symbol: Some(sym),
                left: None,
                right: None,
            });
            heap.push(HeapEntry {
                freq,
                node_idx: idx,
            });
            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Leaf node: symbol='{}' (0x{sym:02X}), freq={freq}.",
                    sym as char
                ),
            );
        }

        // Special case: single symbol gets code "0".
        if n == 1 {
            return vec![(frequencies[0].0, "0".to_string())];
        }

        // Merge until one root remains.
        while heap.len() > 1 {
            let left = heap.pop().unwrap();
            let right = heap.pop().unwrap();
            let parent_freq = left.freq + right.freq;
            let parent_idx = arena.len();

            arena.push(HuffmanNode {
                freq: parent_freq,
                symbol: None,
                left: Some(left.node_idx),
                right: Some(right.node_idx),
            });

            AgentLogger::log(
                AgentFeedback::Step,
                format!(
                    "Merged nodes (freq={} + freq={}) → parent freq={parent_freq} (idx={parent_idx}).",
                    left.freq, right.freq
                ),
            );
            heap.push(HeapEntry {
                freq: parent_freq,
                node_idx: parent_idx,
            });
        }

        let root_idx = heap.pop().unwrap().node_idx;

        // Iterative traversal of the Huffman tree to assign codes.
        // Stack entries: (node_index, current_code_bits)
        let mut stack: Vec<(usize, String)> = vec![(root_idx, String::new())];
        let mut codes: Vec<(u8, String)> = Vec::with_capacity(n);

        while let Some((idx, code)) = stack.pop() {
            let node = &arena[idx];
            if let Some(sym) = node.symbol {
                // Leaf: emit code.
                let final_code = if code.is_empty() {
                    "0".to_string()
                } else {
                    code
                };
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Symbol '{}': code=\"{final_code}\" (len={}) freq={}",
                        sym as char,
                        final_code.len(),
                        node.freq
                    ),
                );
                codes.push((sym, final_code));
            } else {
                // Internal: push children with extended code.
                if let Some(r) = node.right {
                    stack.push((r, format!("{code}1")));
                }
                if let Some(l) = node.left {
                    stack.push((l, format!("{code}0")));
                }
            }
        }

        codes.sort_unstable_by(|a, b| a.1.len().cmp(&b.1.len()).then(a.0.cmp(&b.0)));

        AgentLogger::log(
            AgentFeedback::Success,
            format!("Huffman codes built for {n} symbol(s).",),
        );
        codes
    }

    /// Returns the expected bits-per-symbol for the code table given symbol frequencies.
    pub fn average_code_length(frequencies: &[(u8, u64)], codes: &[(u8, String)]) -> f64 {
        let total_freq: u64 = frequencies.iter().map(|(_, f)| f).sum();
        if total_freq == 0 {
            return 0.0;
        }

        let code_map: std::collections::HashMap<u8, usize> =
            codes.iter().map(|(sym, code)| (*sym, code.len())).collect();

        let weighted: u64 = frequencies
            .iter()
            .map(|(sym, freq)| freq * code_map.get(sym).copied().unwrap_or(0) as u64)
            .sum();

        let avg = weighted as f64 / total_freq as f64;
        AgentLogger::log(
            AgentFeedback::Success,
            format!("Average code length: {avg:.4} bits/symbol."),
        );
        avg
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "huffman_coding",
    description = "Use this for solving huffman coding problems. Trigger Keywords: huffman_coding, huffman coding, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_huffman_coding(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_huffman_coding(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        text: Option<String>,
        frequencies: Option<Vec<(u8, u64)>>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'text' (string) or 'frequencies' as [(byte, count)] pairs.".to_string(),
    })?;

    let frequencies = if let Some(freqs) = req.frequencies {
        freqs
    } else if let Some(text) = &req.text {
        let mut freq_map = std::collections::HashMap::new();
        for &b in text.as_bytes() {
            *freq_map.entry(b).or_insert(0u64) += 1;
        }
        let mut freqs: Vec<(u8, u64)> = freq_map.into_iter().collect();
        freqs.sort_by_key(|&(b, _)| b);
        freqs
    } else {
        return Err(DsaError::InvalidInput {
            message: "Missing input.".to_string(),
            hint: "Provide 'text' or 'frequencies'.".to_string(),
        });
    };

    let codes = HuffmanCoding::build_codes(&frequencies);
    let avg_len = HuffmanCoding::average_code_length(&frequencies, &codes);
    let codebook: Vec<serde_json::Value> = codes.iter().map(|(sym, code)| {
        json!({ "symbol": *sym as char, "byte": sym, "code": code, "bits": code.len() })
    }).collect();

    let result = json!({
        "symbol_count": frequencies.len(),
        "codebook": codebook,
        "avg_bits_per_symbol": avg_len,
    });

    let solver = HuffmanCoding;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["build_codes"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Huffman coding completed."))
}

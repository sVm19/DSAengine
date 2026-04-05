use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Bloom Filter
/// CATEGORY: advanced-topics
/// DESCRIPTION: Implements a probabilistic membership filter with double hashing over a compact bitset.
pub struct BloomFilter;
pub struct BloomMembership {
    bits: Vec<u64>,
    bit_count: usize,
    hash_count: u32,
}

impl Complexity for BloomFilter {
    fn name(&self) -> &'static str {
        "Bloom Filter"
    }

    fn time_complexity(&self) -> &'static str {
        "O(k) per insert/query, where k is the configured number of hash probes."
    }

    fn space_complexity(&self) -> &'static str {
        "O(m) - Stores an m-bit backing array."
    }

    fn description(&self) -> &'static str {
        "Uses multiple hash probes against a dense bitset to trade exactness for speed and memory efficiency."
    }
}

impl BloomFilter {
    pub fn solve(bit_count: usize, hash_count: u32) -> BloomMembership {
        Self::build(bit_count, hash_count)
    }

    pub fn build(bit_count: usize, hash_count: u32) -> BloomMembership {
        let bit_count = bit_count.max(64);
        let hash_count = hash_count.max(1);
        let words = bit_count.div_ceil(64);

        AgentLogger::log(
            AgentFeedback::Info,
            format!(
                "Allocating Bloom filter with {} bits and {} hash probes.",
                words * 64,
                hash_count
            ),
        );

        BloomMembership {
            bits: vec![0; words],
            bit_count: words * 64,
            hash_count,
        }
    }

    fn hash_pair(item: &[u8]) -> (u64, u64) {
        let mut first = 0xcbf2_9ce4_8422_2325u64;
        let mut second = 0x9e37_79b9_7f4a_7c15u64;

        for &byte in item {
            first ^= byte as u64;
            first = first.wrapping_mul(0x1000_0000_01b3);

            second = second
                .wrapping_add(byte as u64)
                .wrapping_add(second << 6)
                .wrapping_add(second >> 2);
        }

        (first, second | 1)
    }
}

impl BloomMembership {
    pub fn insert(&mut self, item: &[u8]) {
        let (first, second) = BloomFilter::hash_pair(item);
        for probe in 0..self.hash_count {
            let bit = (first.wrapping_add(second.wrapping_mul(probe as u64))
                % self.bit_count as u64) as usize;
            self.bits[bit / 64] |= 1u64 << (bit % 64);
            AgentLogger::log(
                AgentFeedback::Step,
                format!("Set Bloom bit {} during probe {}.", bit, probe),
            );
        }
    }

    pub fn contains(&self, item: &[u8]) -> bool {
        let (first, second) = BloomFilter::hash_pair(item);
        for probe in 0..self.hash_count {
            let bit = (first.wrapping_add(second.wrapping_mul(probe as u64))
                % self.bit_count as u64) as usize;
            let is_set = (self.bits[bit / 64] & (1u64 << (bit % 64))) != 0;
            if !is_set {
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Bloom probe {} missed bit {}. Membership rejected.",
                        probe, bit
                    ),
                );
                return false;
            }
        }

        AgentLogger::log(
            AgentFeedback::Success,
            "All Bloom probes hit set bits. Membership is possible.",
        );
        true
    }

    pub fn estimated_false_positive_rate(&self, inserted_items: usize) -> f64 {
        let m = self.bit_count as f64;
        let k = self.hash_count as f64;
        let n = inserted_items as f64;
        (1.0 - (-k * n / m).exp()).powf(k)
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[macros::mcp_tool(name = "advanced_topics.bloom_filter", description = "Use this for solving bloom filter problems. Trigger Keywords: bloom_filter, bloom filter, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(_payload): Json<Value>) -> impl IntoResponse {
    let body = json!({
        "status": "error",
        "engine": "dsaengine",
        "error": "This endpoint is temporarily disabled; under reconstruction."
    });
    (StatusCode::NOT_IMPLEMENTED, Json(body))
}

async fn handle_bloom_filter(payload: Value) -> DsaResult<ResultBox> {
    Err(DsaError::InvalidInput {
        message: "Temporary handler placeholder".to_string(),
        hint: "Endpoint currently under recovery; please try a different skill or wait until rebuild completes.".to_string(),
    })
}

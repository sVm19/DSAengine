use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};

/// SKILL: Consistent Hashing
/// CATEGORY: advanced-topics
/// DESCRIPTION: Builds a virtual-node hash ring for low-churn key placement across distributed nodes.
pub struct ConsistentHashing;
pub struct ConsistentHashRing<'a> {
    nodes: &'a [&'a str],
    ring: Vec<(u64, usize)>,
    replicas: usize,
}

impl Complexity for ConsistentHashing {
    fn name(&self) -> &'static str {
        "Consistent Hashing"
    }

    fn time_complexity(&self) -> &'static str {
        "O(v log v) build and O(log v) lookup, where v = nodes * virtual replicas."
    }

    fn space_complexity(&self) -> &'static str {
        "O(v) - Stores one ring entry per virtual node."
    }

    fn description(&self) -> &'static str {
        "Maps keys onto a sorted virtual-node ring so adding or removing servers only remaps local ranges."
    }
}

impl ConsistentHashing {
    pub fn solve<'a>(nodes: &'a [&'a str], replicas: usize) -> ConsistentHashRing<'a> {
        Self::build(nodes, replicas)
    }

    pub fn build<'a>(nodes: &'a [&'a str], replicas: usize) -> ConsistentHashRing<'a> {
        let replicas = replicas.max(1);
        let mut ring = Vec::with_capacity(nodes.len() * replicas);

        for (node_index, &node) in nodes.iter().enumerate() {
            for replica in 0..replicas {
                let hash = Self::hash_virtual_node(node.as_bytes(), replica as u64);
                ring.push((hash, node_index));
                AgentLogger::log(
                    AgentFeedback::Step,
                    format!(
                        "Placed virtual node {}#{} at ring hash {}.",
                        node, replica, hash
                    ),
                );
            }
        }

        ring.sort_unstable_by_key(|&(hash, _)| hash);
        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Consistent hash ring built with {} virtual nodes.",
                ring.len()
            ),
        );

        ConsistentHashRing {
            nodes,
            ring,
            replicas,
        }
    }

    fn hash_bytes(bytes: &[u8]) -> u64 {
        let mut state = 0xcbf2_9ce4_8422_2325u64;
        for &byte in bytes {
            state ^= byte as u64;
            state = state.wrapping_mul(0x1000_0000_01b3);
        }
        state
    }

    fn hash_virtual_node(node: &[u8], replica: u64) -> u64 {
        let mut state = Self::hash_bytes(node);
        for byte in replica.to_le_bytes() {
            state ^= byte as u64;
            state = state.wrapping_mul(0x1000_0000_01b3);
        }
        state
    }
}

impl<'a> ConsistentHashRing<'a> {
    pub fn route(&self, key: &[u8]) -> Option<&'a str> {
        if self.ring.is_empty() {
            return None;
        }

        let key_hash = ConsistentHashing::hash_bytes(key);
        let slot = self
            .ring
            .binary_search_by_key(&key_hash, |&(hash, _)| hash)
            .unwrap_or_else(|index| index);
        let ring_index = if slot == self.ring.len() { 0 } else { slot };
        let node = self.nodes[self.ring[ring_index].1];

        AgentLogger::log(
            AgentFeedback::Success,
            format!(
                "Key hash {} routed to node '{}' via ring slot {}.",
                key_hash, node, ring_index
            ),
        );
        Some(node)
    }

    pub fn route_many<'b>(&self, keys: &'b [&'b [u8]]) -> Vec<Option<&'a str>> {
        keys.iter().map(|key| self.route(key)).collect()
    }

    pub fn replicas(&self) -> usize {
        self.replicas
    }
}

// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[macros::mcp_tool(
    name = "consistent_hashing",
    description = "Use this for solving consistent hashing problems. Trigger Keywords: consistent_hashing, consistent hashing, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results."
)]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_consistent_hashing(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_consistent_hashing(payload: Value) -> DsaResult<ResultBox> {
    #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
    struct Request {
        nodes: Vec<String>,
        replicas: Option<usize>,
        keys: Option<Vec<String>>,
    }

    let req: Request = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid request: {e}"),
        hint: "Provide 'nodes' (server names). Optional 'replicas' (default 3), 'keys' (to route)."
            .to_string(),
    })?;

    let result = {
        let node_refs: Vec<&str> = req.nodes.iter().map(|s| s.as_str()).collect();
        let ring = ConsistentHashing::build(&node_refs, req.replicas.unwrap_or(3));
        let routing = if let Some(keys) = &req.keys {
            keys.iter()
                .map(|k| {
                    let dest = ring.route(k.as_bytes());
                    json!({ "key": k, "node": dest })
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        };
        json!({ "nodes": req.nodes, "replicas": ring.replicas(), "routing": routing })
    };

    let solver = ConsistentHashing;
    Ok(ResultBox::success(json!({
        "result": result,
        "available_modes": ["build_and_route"],
    }))
    .with_complexity(json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    }))
    .with_description("Consistent hashing completed."))
}

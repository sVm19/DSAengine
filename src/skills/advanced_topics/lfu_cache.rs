use crate::utils::complexity::Complexity;
use crate::utils::logger::{AgentFeedback, AgentLogger};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

/// SKILL: LFU Cache
/// CATEGORY: advanced-topics
/// DESCRIPTION: Implements an LFU cache with frequency buckets and lazy stale-entry cleanup.
pub struct LFUCache;
pub struct LfuCacheState<K, V>
where
    K: Clone + Eq + Hash + Ord,
{
    capacity: usize,
    clock: u64,
    values: HashMap<K, LfuEntry<V>>,
    frequency_queues: BTreeMap<usize, VecDeque<(K, u64)>>,
}

#[derive(Clone)]
struct LfuEntry<V> {
    value: V,
    frequency: usize,
    stamp: u64,
}

impl Complexity for LFUCache {
    fn name(&self) -> &'static str {
        "LFU Cache"
    }

    fn time_complexity(&self) -> &'static str {
        "O(log n) amortized get/put due to ordered frequency buckets and lazy cleanup."
    }

    fn space_complexity(&self) -> &'static str {
        "O(capacity) - Stores values plus per-frequency access queues."
    }

    fn description(&self) -> &'static str {
        "Evicts the least frequently used entry, breaking ties by recency within each frequency bucket."
    }
}

impl LFUCache {
    pub fn solve<K, V>(capacity: usize) -> LfuCacheState<K, V>
    where
        K: Clone + Eq + Hash + Ord + Debug,
    {
        Self::build(capacity)
    }

    pub fn build<K, V>(capacity: usize) -> LfuCacheState<K, V>
    where
        K: Clone + Eq + Hash + Ord + Debug,
    {
        AgentLogger::log(
            AgentFeedback::Info,
            format!("Creating LFU cache with capacity {}.", capacity),
        );

        LfuCacheState {
            capacity,
            clock: 0,
            values: HashMap::with_capacity(capacity),
            frequency_queues: BTreeMap::new(),
        }
    }
}

impl<K, V> LfuCacheState<K, V>
where
    K: Clone + Eq + Hash + Ord + Debug,
{
    pub fn get(&mut self, key: K) -> Option<&V> {
        if !self.values.contains_key(&key) {
            AgentLogger::log(
                AgentFeedback::Warning,
                format!("LFU miss for key {:?}.", key),
            );
            return None;
        }

        self.touch(key.clone());
        self.discard_stale_fronts();
        self.values.get(&key).map(|entry| &entry.value)
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        if self.capacity == 0 {
            AgentLogger::log(
                AgentFeedback::Warning,
                "LFU cache capacity is zero; insert is dropped immediately.",
            );
            return Some(value);
        }

        if self.values.contains_key(&key) {
            let previous = {
                let entry = self.values.get_mut(&key).expect("entry exists");
                std::mem::replace(&mut entry.value, value)
            };
            self.touch(key.clone());
            return Some(previous);
        }

        if self.values.len() == self.capacity {
            self.evict_one();
        }

        self.clock += 1;
        let stamp = self.clock;
        self.values.insert(
            key.clone(),
            LfuEntry {
                value,
                frequency: 1,
                stamp,
            },
        );
        self.frequency_queues
            .entry(1)
            .or_default()
            .push_back((key.clone(), stamp));

        AgentLogger::log(
            AgentFeedback::Step,
            format!(
                "Inserted LFU entry for key {:?} with initial frequency 1.",
                key
            ),
        );
        None
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    fn touch(&mut self, key: K) {
        self.clock += 1;
        let stamp = self.clock;
        let next_frequency = {
            let entry = self.values.get_mut(&key).expect("entry exists");
            entry.frequency += 1;
            entry.stamp = stamp;
            entry.frequency
        };

        self.frequency_queues
            .entry(next_frequency)
            .or_default()
            .push_back((key.clone(), stamp));
        AgentLogger::log(
            AgentFeedback::Step,
            format!("Raised key {:?} to LFU frequency {}.", key, next_frequency),
        );
    }

    fn evict_one(&mut self) {
        loop {
            let Some(frequency) = self.frequency_queues.keys().next().copied() else {
                return;
            };

            let mut evicted_key = None;
            let remove_bucket = {
                let queue = self
                    .frequency_queues
                    .get_mut(&frequency)
                    .expect("bucket exists");
                while let Some((key, stamp)) = queue.pop_front() {
                    let is_live = matches!(
                        self.values.get(&key),
                        Some(entry) if entry.frequency == frequency && entry.stamp == stamp
                    );
                    if is_live {
                        evicted_key = Some(key.clone());
                        break;
                    }
                }
                queue.is_empty()
            };

            if remove_bucket {
                self.frequency_queues.remove(&frequency);
            }

            if let Some(key) = evicted_key {
                self.values.remove(&key);
                AgentLogger::log(
                    AgentFeedback::Success,
                    format!(
                        "Evicted LFU key {:?} from frequency bucket {}.",
                        key, frequency
                    ),
                );
                return;
            }
        }
    }

    fn discard_stale_fronts(&mut self) {
        let frequencies = self.frequency_queues.keys().copied().collect::<Vec<_>>();
        for frequency in frequencies {
            let remove_bucket = {
                let queue = self
                    .frequency_queues
                    .get_mut(&frequency)
                    .expect("bucket exists");
                while let Some((key, stamp)) = queue.front().cloned() {
                    let is_stale = match self.values.get(&key) {
                        Some(entry) => entry.frequency != frequency || entry.stamp != stamp,
                        None => true,
                    };
                    if is_stale {
                        queue.pop_front();
                    } else {
                        break;
                    }
                }
                queue.is_empty()
            };

            if remove_bucket {
                self.frequency_queues.remove(&frequency);
            }
        }
    }
}



// --- AXUM WEB BRIDGE ---
use crate::utils::{api_docs, responses::*};
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum LfuOperation {
    Put { key: String, value: String },
    Get { key: String },
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct LfuCacheRequest {
    pub capacity: usize,
    pub operations: Vec<LfuOperation>,
}

#[macros::mcp_tool(name = "advanced_topics.lfu_cache", description = "Use this for solving lfu cache problems. Trigger Keywords: lfu_cache, lfu cache, algorithm, dsa. Input Hints: Look for input fields like nums, numbers, arr, target, edges, adj, source, capacity, weight, values in the user's text to populate task arguments.. Why: Choose this over generic fallback when the problem domain matches the algorithm's strengths for best-performance results.")]
pub async fn post(Json(payload): Json<Value>) -> impl IntoResponse {
    match handle_lfu_cache(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn handle_lfu_cache(payload: Value) -> DsaResult<ResultBox<serde_json::Value>> {
    let req: LfuCacheRequest = serde_json::from_value(payload).map_err(|e| DsaError::InvalidInput {
        message: format!("Invalid LfuCacheRequest: {e}"),
        hint: "Provide 'capacity' and an 'operations' list of {type: 'put'|'get', key, value?}.".to_string(),
    })?;

    let mut cache = LFUCache::build::<String, String>(req.capacity);
    let mut results = Vec::new();

    for op in &req.operations {
        match op {
            LfuOperation::Put { key, value } => {
                let prev = cache.put(key.clone(), value.clone());
                results.push(json!({ "op": "put", "key": key, "value": value, "previous": prev }));
            }
            LfuOperation::Get { key } => {
                let val = cache.get(key.clone()).map(|v| v.clone());
                results.push(json!({ "op": "get", "key": key, "value": val }));
            }
        }
    }

    let solver = LFUCache;
    let complexity = json!({
        "name": solver.name(),
        "time": solver.time_complexity(),
        "space": solver.space_complexity(),
        "description": solver.description(),
    });

    let res_val = json!({
        "results": results,
        "final_len": cache.len()
    });

    Ok(ResultBox::success(res_val)
        .with_complexity(complexity)
        .with_description("LFU cache operations completed."))
}

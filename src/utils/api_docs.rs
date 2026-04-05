use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, JsonSchema)]
pub struct ArrayRequest {
    pub nums: Option<Vec<i64>>,
    pub target: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, JsonSchema)]
pub struct GraphRequest {
    pub edges: Option<Vec<(usize, usize, u64)>>,
    pub adj: Option<Vec<Vec<(usize, u64)>>>,
    pub source: Option<usize>,
    pub num_nodes: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, JsonSchema)]
pub struct TreeRequest {
    pub nodes: Option<Vec<Option<i64>>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, JsonSchema)]
pub struct Complexity {
    pub name: Option<String>,
    pub time: Option<String>,
    pub space: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, JsonSchema)]
pub struct StandardResponse {
    pub status: String,
    pub engine: String,
    pub complexity: Option<Complexity>,
    pub result: Option<Value>,
    pub description: Option<String>,
    pub before_vs_after: Option<String>,
    pub correction_suggestion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, JsonSchema)]
pub struct SkillManifest {
    pub category: String,
    pub name: String,
    pub input_contract: Vec<String>,
    pub description: Option<String>,
}

pub fn suggest_correction(payload: &Value) -> Option<String> {
    let corrections = vec![
        ("numbers", "nums", "Did you mean 'nums'? Use 'nums' for array inputs."),
        ("edges_list", "edges", "Did you mean 'edges'? Use 'edges' for edge list inputs."),
        ("tree_nodes", "nodes", "Did you mean 'nodes'? Use 'nodes' for tree input."),
    ];

    if let Some(map) = payload.as_object() {
        for (bad, _, suggestion) in corrections.iter() {
            if map.contains_key(*bad) {
                return Some(suggestion.to_string());
            }
        }
    }

    None
}

pub fn normalize_input_keys(payload: &Value) -> Value {
    let mut output = payload.clone();
    if let Some(map) = output.as_object_mut() {
        if map.contains_key("numbers") && !map.contains_key("nums") {
            if let Some(v) = map.remove("numbers") {
                map.insert("nums".to_string(), v);
            }
        }
        if map.contains_key("edges_list") && !map.contains_key("edges") {
            if let Some(v) = map.remove("edges_list") {
                map.insert("edges".to_string(), v);
            }
        }
        if map.contains_key("tree_nodes") && !map.contains_key("nodes") {
            if let Some(v) = map.remove("tree_nodes") {
                map.insert("nodes".to_string(), v);
            }
        }
    }
    output
}

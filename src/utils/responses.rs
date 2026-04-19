use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Standardized result box for all DSA skill responses
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResultBox<T = serde_json::Value> {
    pub status: String,
    pub engine: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complexity: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_vs_after: Option<String>,
}

impl<T: Serialize> ResultBox<T> {
    pub fn success(result: T) -> Self {
        ResultBox {
            status: "success".to_string(),
            engine: "dsaengine".to_string(),
            error_type: None,
            message: None,
            hint: None,
            complexity: None,
            result: Some(result),
            description: None,
            before_vs_after: None,
        }
    }

    pub fn error(error_type: &str, message: &str, hint: Option<&str>) -> Self {
        ResultBox {
            status: "error".to_string(),
            engine: "dsaengine".to_string(),
            error_type: Some(error_type.to_string()),
            message: Some(message.to_string()),
            hint: hint.map(|h| h.to_string()),
            complexity: None,
            result: None,
            description: None,
            before_vs_after: None,
        }
    }

    pub fn with_complexity(mut self, complexity: serde_json::Value) -> Self {
        self.complexity = Some(complexity);
        self
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn with_before_vs_after(mut self, before_after: &str) -> Self {
        self.before_vs_after = Some(before_after.to_string());
        self
    }
}

/// Custom error type for DSA operations
#[derive(Debug)]
pub enum DsaError {
    ValidationError {
        message: String,
        hint: String,
    },
    IndexOutOfBounds {
        index: usize,
        bounds: usize,
        context: String,
    },
    InvalidInput {
        message: String,
        hint: String,
    },
    ConversionError {
        message: String,
        hint: String,
    },
    GraphError {
        message: String,
        hint: String,
    },
}

impl std::fmt::Display for DsaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DsaError::ValidationError { message, .. } => write!(f, "{}", message),
            DsaError::IndexOutOfBounds { .. } => write!(f, "Index out of bounds"),
            DsaError::InvalidInput { message, .. } => write!(f, "{}", message),
            DsaError::ConversionError { message, .. } => write!(f, "{}", message),
            DsaError::GraphError { message, .. } => write!(f, "{}", message),
        }
    }
}

impl IntoResponse for DsaError {
    fn into_response(self) -> Response {
        let (status, error_type, message, hint) = match self {
            DsaError::ValidationError { message, hint } => {
                (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", message, hint)
            }
            DsaError::IndexOutOfBounds {
                index,
                bounds,
                context,
            } => {
                let message = format!("Index {} out of bounds for {}", index, context);
                let hint = format!(
                    "You provided index {}, but valid range is 0-{}. {}",
                    index,
                    bounds - 1,
                    if bounds == 0 {
                        "The collection is empty.".to_string()
                    } else {
                        format!("Valid indices are: 0 to {}.", bounds - 1)
                    }
                );
                (
                    StatusCode::BAD_REQUEST,
                    "INDEX_OUT_OF_BOUNDS",
                    message,
                    hint,
                )
            }
            DsaError::InvalidInput { message, hint } => {
                (StatusCode::BAD_REQUEST, "INVALID_INPUT", message, hint)
            }
            DsaError::ConversionError { message, hint } => {
                (StatusCode::BAD_REQUEST, "CONVERSION_ERROR", message, hint)
            }
            DsaError::GraphError { message, hint } => {
                (StatusCode::BAD_REQUEST, "GRAPH_ERROR", message, hint)
            }
        };

        let body = json!({
            "status": "error",
            "engine": "dsaengine",
            "error_type": error_type,
            "message": message,
            "hint": hint,
        });

        (status, Json(body)).into_response()
    }
}

/// Result type alias for DSA operations
pub type DsaResult<T> = Result<T, DsaError>;

/// Helper traits and functions for pre-flight validation

pub trait GraphValidator {
    fn validate_nodes(&self, num_nodes: usize) -> DsaResult<()>;
}

impl GraphValidator for Vec<(usize, usize, u64)> {
    fn validate_nodes(&self, num_nodes: usize) -> DsaResult<()> {
        for (u, v, _) in self {
            if *u >= num_nodes {
                return Err(DsaError::IndexOutOfBounds {
                    index: *u,
                    bounds: num_nodes,
                    context: "source node in edges list".to_string(),
                });
            }
            if *v >= num_nodes {
                return Err(DsaError::IndexOutOfBounds {
                    index: *v,
                    bounds: num_nodes,
                    context: "destination node in edges list".to_string(),
                });
            }
        }
        Ok(())
    }
}

pub fn validate_source_in_bounds(source: usize, num_nodes: usize) -> DsaResult<()> {
    if source >= num_nodes {
        Err(DsaError::IndexOutOfBounds {
            index: source,
            bounds: num_nodes,
            context: "source node".to_string(),
        })
    } else {
        Ok(())
    }
}

pub fn validate_node_in_bounds(node: usize, num_nodes: usize, context: &str) -> DsaResult<()> {
    if node >= num_nodes {
        Err(DsaError::IndexOutOfBounds {
            index: node,
            bounds: num_nodes,
            context: context.to_string(),
        })
    } else {
        Ok(())
    }
}

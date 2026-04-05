/// Logger Module: The voice of the dsaengine.
/// This allows AI agents to report their progress, errors,
/// and "Aha!" moments during problem-solving.
use std::fmt::Display;

/// Different levels of feedback for the AI Agent to provide.
#[derive(Debug)]
pub enum AgentFeedback {
    Info,    // General progress (e.g., "Starting Dijkstra...")
    Step,    // Specific algorithm step (e.g., "Visiting Node A")
    Success, // Problem solved (e.g., "Shortest path found!")
    Warning, // Potential inefficiency (e.g., "O(n^2) detected")
    Error,   // Logical failure (e.g., "Graph is disconnected")
}

pub struct AgentLogger;

impl AgentLogger {
    /// Logs a message in a format that both humans and LLMs can easily parse.
    ///
    /// ### Usage:
    /// ```rust
    /// use dsaengine::utils::logger::{AgentFeedback, AgentLogger};
    ///
    /// AgentLogger::log(AgentFeedback::Step, "Checking index 5 in the array");
    /// ```
    pub fn log<T: Display>(level: AgentFeedback, message: T) {
        let prefix = match level {
            AgentFeedback::Info => "🤖 [INFO]:",
            AgentFeedback::Step => "⏭️  [STEP]:",
            AgentFeedback::Success => "✅ [SUCCESS]:",
            AgentFeedback::Warning => "⚠️  [WARNING]:",
            AgentFeedback::Error => "❌ [ERROR]:",
        };

        println!("{} {}", prefix, message);
    }

    /// A specialized log for showing code changes or suggested optimizations.
    pub fn suggest_fix(original: &str, optimized: &str) {
        println!("🛠️  [OPTIMIZATION SUGGESTED]:");
        println!("   FROM: {}", original);
        println!("   TO:   {}", optimized);
    }
}

/// A macro for quick, formatted agent logging.
#[macro_export]
macro_rules! agent_log {
    ($level:expr, $($arg:tt)*) => {
        $crate::utils::logger::AgentLogger::log($level, format!($($arg)*));
    };
}

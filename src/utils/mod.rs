/// Utils Module: The foundation of the dsaengine.
/// These tools are used by AI agents to measure and visualize
/// the performance of every DSA skill.
pub mod complexity; // Time and Space analysis logic
pub mod logger; // Specialized logging for AI feedback
pub mod pattern_matcher;
pub mod visualizer; // ASCII and Terminal rendering traits
pub mod api_docs;
pub mod responses; // Agent-oriented error handling

/// Re-exporting the benchmark function for easy access
/// Usage: dsaengine::utils::benchmark(|| my_function());
pub use complexity::benchmark;
pub use responses::{DsaError, DsaResult, ResultBox};

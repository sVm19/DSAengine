#![allow(unused_imports)]
#![allow(unused_variables)]

/// The dsaengine root crate.
/// This file exposes the core modules to the outside world,
/// allowing AI agents and developers to import specific DSA skills.
// 1. Core Utilities: Benchmarking, Logging, and Visualization
pub mod utils;

// 2. The Skill Registry: 150+ DSA implementations categorized by type
pub mod skills;

/// A helper trait that all high-performance skills in this engine
/// should implement to provide consistent feedback to the AI Agent.
pub use utils::complexity::Complexity;

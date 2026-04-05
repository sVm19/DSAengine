use std::time::{Duration, Instant};

/// The base trait that all "Solver" skills will implement.
/// This allows an AI agent to call .info() on any skill
/// to understand its efficiency before applying it to a problem.
pub trait Complexity {
    /// Returns the formal name of the DSA skill (e.g., "Dijkstra's Algorithm")
    fn name(&self) -> &'static str;

    /// Returns the Time Complexity in Big O notation (e.g., "O(n log n)")
    fn time_complexity(&self) -> &'static str;

    /// Returns the Space Complexity in Big O notation (e.g., "O(n)")
    fn space_complexity(&self) -> &'static str;

    /// Optional: Provides a human/AI-readable description of when to use this skill
    fn description(&self) -> &'static str {
        "No description provided."
    }
}

/// A standardized structure for performance reports.
/// AI agents can use this to compare two different algorithms for the same problem.
#[derive(Debug)]
pub struct PerformanceReport {
    pub execution_time: Duration,
    pub steps_estimated: u64,
}

/// The core benchmarking tool.
/// It wraps any logic (a closure) and measures exactly how long it takes to run.
///
/// ### Usage:
/// ```rust
/// use dsaengine::utils::benchmark;
///
/// let (result, report) = benchmark(|| {
///     // code to measure
/// });
/// ```
pub fn benchmark<F, T>(f: F) -> (T, PerformanceReport)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();

    let report = PerformanceReport {
        execution_time: duration,
        // In a real engine, we could track memory or CPU cycles here
        steps_estimated: 0,
    };

    (result, report)
}

/// A simple macro to help the AI Agent print complexity stats quickly.
#[macro_export]
macro_rules! log_complexity {
    ($skill:expr) => {
        println!(
            "Skill: {} | Time: {} | Space: {}",
            $skill.name(),
            $skill.time_complexity(),
            $skill.space_complexity()
        );
    };
}

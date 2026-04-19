/// Page 8: Dynamic Programming
/// This module organizes reusable state transitions, memoization, and tabulation-style optimizations.
pub mod climbing_stairs;
pub mod coin_change;
pub mod dp_on_trees;
pub mod edit_distance;
pub mod fibonacci_viz;
pub mod house_robber;
pub mod lcs_solver;
pub mod longest_increasing_sub;
pub mod palindrome_partition;
pub mod pattern_matcher;
pub mod regex_matching;
pub mod subset_sum;
pub mod wildcard_matching;
pub mod word_break;

// Re-exporting Climbing Stairs as the "Primary Skill" of this page
pub use climbing_stairs::ClimbingStairs;

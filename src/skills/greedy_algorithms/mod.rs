/// Page 9: Greedy Algorithms
/// This module focuses on local-choice heuristics, interval selection, and scheduling-style optimization.
pub mod activity_selection;
pub mod fractional_knapsack;
pub mod gas_station;
pub mod huffman_coding;
pub mod jump_game;
pub mod queue_reconstruct;
pub mod task_scheduler;

// Re-exporting Jump Game as the "Primary Skill" of this page
pub use jump_game::JumpGame;

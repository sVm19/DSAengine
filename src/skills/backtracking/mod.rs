/// Page 10: Backtracking
/// This module supports branching search, pruning, and constraint-driven enumeration.
pub mod combinations;
pub mod n_queens;
pub mod permutations;
pub mod rat_in_maze;
pub mod subsets;
pub mod sudoku_solver;
pub mod word_search;

// Re-exporting N Queens as the "Primary Skill" of this page
pub use n_queens::NQueens;

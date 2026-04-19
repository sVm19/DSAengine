/// Skills Module: The Categorized Skill Set.
/// Each category is gated by a 'feature' flag defined in Cargo.toml.
/// This allows the AI Agent to load only the "Page" it needs.

// PAGE 1: Fundamentals
#[cfg(feature = "fundamentals")]
pub mod dsa_fundamentals;

// PAGE 2: Arrays & Strings
#[cfg(feature = "arrays")]
pub mod arrays_strings;

// PAGE 3: Linked Lists
#[cfg(feature = "linked_lists")]
pub mod linked_lists;

// PAGE 4: Stacks & Queues
#[cfg(feature = "stacks")]
pub mod stacks_queues;

// PAGE 5: Binary Trees
#[cfg(feature = "trees")]
pub mod trees_binary;

// PAGE 6: Advanced Trees (Tries, Segment Trees)
#[cfg(feature = "trees_adv")]
pub mod trees_advanced;

// PAGE 7: Graphs
#[cfg(feature = "graphs")]
pub mod graphs;

// PAGE 8: Dynamic Programming
#[cfg(feature = "dp")]
pub mod dynamic_programming;

// PAGE 9: Greedy Algorithms
#[cfg(feature = "greedy")]
pub mod greedy_algorithms;

// PAGE 10: Backtracking
#[cfg(feature = "backtracking")]
pub mod backtracking;

// PAGE 11: Sorting & Searching
#[cfg(feature = "sorting")]
pub mod sorting_searching;

// PAGE 12: Bit Manipulation & Advanced Topics
#[cfg(feature = "advanced")]
pub mod advanced_topics;

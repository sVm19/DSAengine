/// Page 2: Arrays & Strings
/// This module focuses on linear data structures, search optimizations, and string-processing patterns.
pub mod anagram_detector;
pub mod array_rotation;
pub mod boyer_moore_voting;
pub mod compression;
pub mod container_water;
pub mod difference_array;
pub mod dutch_national_flag;
pub mod kadanes_algorithm;
pub mod kmp_search;
pub mod longest_substring;
pub mod manachers_algorithm;
pub mod next_permutation;
pub mod palindrome_matcher;
pub mod rabin_karp;
pub mod rainwater_trapping;
pub mod sparse_table;
pub mod string_hashing;
pub mod string_toolkit;
pub mod subarray_sum;
pub mod suffix_array_lite;
pub mod three_sum_solver;
pub mod two_sum_matcher;
pub mod z_algorithm;

// Re-exporting Kadanes Algorithm as the "Primary Skill" of this page
pub use kadanes_algorithm::KadanesAlgorithm;

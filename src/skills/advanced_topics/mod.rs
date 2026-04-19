/// Page 12: Advanced Topics
/// This module collects systems-oriented data structures and specialized indexing strategies.
pub mod bit_manipulation;
pub mod bloom_filter;
pub mod consistent_hashing;
pub mod lfu_cache;
pub mod lru_cache;
pub mod segment_tree_lazy;
pub mod skip_list;
pub mod suffix_array;
pub mod trie_autocomplete;
pub mod union_find;

// Re-exporting Union Find as the "Primary Skill" of this page
pub use union_find::UnionFind;

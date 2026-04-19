/// Page 6: Advanced Trees
/// This module covers indexed tree structures, tries, heaps, and logarithmic update/query workflows.
pub mod avl_tree;
pub mod b_tree_index;
pub mod fenwick_tree;
pub mod heap_priority_queue;
pub mod median_stream;
pub mod red_black_tree;
pub mod segment_tree_builder;
pub mod segment_tree_query;
pub mod top_k_elements;
pub mod trie_impl;
pub mod trie_visualizer;

// Re-exporting Trie Impl as the "Primary Skill" of this page
pub use trie_impl::TrieImpl;

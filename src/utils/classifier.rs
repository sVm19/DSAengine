//! Deterministic DSA Classifier
//!
//! Maps natural-language problem descriptions to optimal DSA algorithm
//! recommendations using keyword/pattern matching — ZERO AI API cost.
//! The coding agent's own LLM provides the intelligence; this is the lookup table.

/// A single algorithm recommendation with full metadata.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Recommendation {
    pub algorithm: &'static str,
    pub category: &'static str,
    pub tool_name: &'static str,
    pub time_complexity: &'static str,
    pub space_complexity: &'static str,
    pub confidence: f32,
    pub reason: &'static str,
    pub anti_pattern: &'static str,
}

/// Classifies a free-text problem description into ranked algorithm recommendations.
/// Uses deterministic multi-keyword scoring — no AI needed.
pub fn classify(description: &str) -> Vec<Recommendation> {
    let d = description.to_lowercase();
    let mut results: Vec<(f32, Recommendation)> = Vec::new();

    macro_rules! score {
        ($keywords:expr) => {{
            $keywords.iter().filter(|k| d.contains(*k)).count() as f32 / $keywords.len() as f32
        }};
    }

    // ── GRAPHS ──────────────────────────────────────────────────────────────
    {
        let s = score!(&[
            "shortest path",
            "weighted",
            "distance",
            "route",
            "navigation"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.4,
                Recommendation {
                    algorithm: "Dijkstra's Algorithm",
                    category: "graphs",
                    tool_name: "graphs.dijkstra",
                    time_complexity: "O(E log V)",
                    space_complexity: "O(V)",
                    confidence: (s + 0.4).min(1.0),
                    reason:
                        "Optimal for single-source shortest paths in non-negative weighted graphs.",
                    anti_pattern:
                        "Avoid nested loops over adjacency lists — use a min-heap priority queue.",
                },
            ));
        }
    }
    {
        let s = score!(&["negative weight", "negative edge", "bellman"]);
        if s > 0.0 {
            results.push((
                s + 0.5,
                Recommendation {
                    algorithm: "Bellman-Ford",
                    category: "graphs",
                    tool_name: "graphs.bellman_ford",
                    time_complexity: "O(V·E)",
                    space_complexity: "O(V)",
                    confidence: (s + 0.5).min(1.0),
                    reason: "Handles negative edge weights and detects negative cycles.",
                    anti_pattern:
                        "Do not use Dijkstra with negative weights — it produces wrong results.",
                },
            ));
        }
    }
    {
        let s = score!(&[
            "minimum spanning tree",
            "mst",
            "connect all nodes",
            "minimum cost network"
        ]);
        if s > 0.0 {
            results.push((s + 0.5, Recommendation {
                algorithm: "MST — Kruskal / Prim",
                category: "graphs",
                tool_name: "graphs.mst_kruskal_prim",
                time_complexity: "O(E log E)",
                space_complexity: "O(V)",
                confidence: (s + 0.5).min(1.0),
                reason: "Kruskal (sort edges + Union-Find) is optimal for sparse graphs; Prim for dense.",
                anti_pattern: "Avoid re-checking all edges for each node — use Union-Find to detect cycles in O(α(n)).",
            }));
        }
    }
    {
        let s = score!(&[
            "connected components",
            "disjoint",
            "union find",
            "merge groups",
            "friend circles",
            "network connectivity"
        ]);
        if s > 0.0 {
            results.push((s + 0.5, Recommendation {
                algorithm: "Union-Find (Disjoint Set)",
                category: "advanced_topics",
                tool_name: "advanced_topics.union_find",
                time_complexity: "O(α(n)) per operation",
                space_complexity: "O(n)",
                confidence: (s + 0.5).min(1.0),
                reason: "Near-constant time union/find with path compression + union by rank.",
                anti_pattern: "Avoid BFS/DFS for every connectivity query — Union-Find amortizes to O(α(n)).",
            }));
        }
    }
    {
        let s = score!(&[
            "topological",
            "dependency order",
            "build order",
            "course schedule",
            "task dependency",
            "dag"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.4,
                Recommendation {
                    algorithm: "Topological Sort (Kahn's BFS)",
                    category: "graphs",
                    tool_name: "graphs.topological_sort",
                    time_complexity: "O(V + E)",
                    space_complexity: "O(V)",
                    confidence: (s + 0.4).min(1.0),
                    reason: "Determines a valid execution order for tasks with dependencies.",
                    anti_pattern:
                        "Don't sort by priority alone — dependency order requires graph traversal.",
                },
            ));
        }
    }
    {
        let s = score!(&[
            "bfs",
            "breadth first",
            "level order",
            "shortest hops",
            "minimum steps"
        ]);
        if s > 0.0 {
            results.push((s + 0.3, Recommendation {
                algorithm: "BFS (Breadth-First Search)",
                category: "graphs",
                tool_name: "graphs.bfs_generator",
                time_complexity: "O(V + E)",
                space_complexity: "O(V)",
                confidence: (s + 0.3).min(1.0),
                reason: "Guarantees shortest path (by hops) in unweighted graphs.",
                anti_pattern: "Avoid DFS for shortest-path problems — DFS doesn't guarantee minimum hops.",
            }));
        }
    }
    {
        let s = score!(&[
            "dfs",
            "depth first",
            "island",
            "connected region",
            "flood fill",
            "number of islands"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.3,
                Recommendation {
                    algorithm: "DFS / Island Counter",
                    category: "graphs",
                    tool_name: "graphs.dfs_generator",
                    time_complexity: "O(V + E)",
                    space_complexity: "O(V)",
                    confidence: (s + 0.3).min(1.0),
                    reason:
                        "DFS explores all reachable nodes — ideal for connected region counting.",
                    anti_pattern: "Mark visited nodes to avoid O(V²) revisits.",
                },
            ));
        }
    }

    // ── ARRAYS & STRINGS ────────────────────────────────────────────────────
    {
        let s = score!(&[
            "subarray",
            "sliding window",
            "maximum sum",
            "minimum window",
            "contiguous"
        ]);
        if s > 0.0 {
            results.push((s + 0.4, Recommendation {
                algorithm: "Sliding Window / Kadane's",
                category: "arrays_strings",
                tool_name: "arrays_strings.kadanes_algorithm",
                time_complexity: "O(n)",
                space_complexity: "O(1)",
                confidence: (s + 0.4).min(1.0),
                reason: "Single-pass O(n) solution for contiguous subarray problems.",
                anti_pattern: "Don't use O(n²) nested loops to check every subarray — slide a window instead.",
            }));
        }
    }
    {
        let s = score!(&[
            "two sum",
            "pair sum",
            "find pair",
            "target sum",
            "two numbers"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.5,
                Recommendation {
                    algorithm: "Two-Sum (HashMap)",
                    category: "arrays_strings",
                    tool_name: "arrays_strings.two_sum_matcher",
                    time_complexity: "O(n)",
                    space_complexity: "O(n)",
                    confidence: (s + 0.5).min(1.0),
                    reason: "HashMap lookup reduces O(n²) brute-force to O(n).",
                    anti_pattern:
                        "Never use nested loops to find pairs — store complements in a HashMap.",
                },
            ));
        }
    }
    {
        let s = score!(&["anagram", "permutation check", "character frequency"]);
        if s > 0.0 {
            results.push((
                s + 0.5,
                Recommendation {
                    algorithm: "Anagram Detector (Frequency Table)",
                    category: "arrays_strings",
                    tool_name: "arrays_strings.anagram_detector",
                    time_complexity: "O(n)",
                    space_complexity: "O(1)",
                    confidence: (s + 0.5).min(1.0),
                    reason: "Fixed 26-bucket frequency balance runs in O(n) with O(1) space.",
                    anti_pattern:
                        "Don't sort both strings (O(n log n)) — frequency counting is O(n).",
                },
            ));
        }
    }
    {
        let s = score!(&[
            "pattern search",
            "substring search",
            "kmp",
            "string matching"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.4,
                Recommendation {
                    algorithm: "KMP / Rabin-Karp",
                    category: "arrays_strings",
                    tool_name: "arrays_strings.kmp_search",
                    time_complexity: "O(n + m)",
                    space_complexity: "O(m)",
                    confidence: (s + 0.4).min(1.0),
                    reason: "Failure function / rolling hash avoids O(n·m) naive search.",
                    anti_pattern:
                        "Don't slice and compare inside a loop — precompute the failure function.",
                },
            ));
        }
    }
    {
        let s = score!(&[
            "binary search",
            "sorted array",
            "search in sorted",
            "find position",
            "rotated array"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.4,
                Recommendation {
                    algorithm: "Binary Search",
                    category: "sorting_searching",
                    tool_name: "sorting_searching.binary_search_template",
                    time_complexity: "O(log n)",
                    space_complexity: "O(1)",
                    confidence: (s + 0.4).min(1.0),
                    reason: "Halves search space each step — O(log n) on any sorted array.",
                    anti_pattern:
                        "Never linear-scan a sorted array — binary search cuts it to O(log n).",
                },
            ));
        }
    }
    {
        let s = score!(&[
            "longest substring",
            "no repeat",
            "unique characters",
            "duplicate chars"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.4,
                Recommendation {
                    algorithm: "Sliding Window + HashSet",
                    category: "arrays_strings",
                    tool_name: "arrays_strings.longest_substring",
                    time_complexity: "O(n)",
                    space_complexity: "O(k)",
                    confidence: (s + 0.4).min(1.0),
                    reason: "Two-pointer sliding window with a set tracks unique chars in O(n).",
                    anti_pattern: "Avoid O(n²) by not re-checking substrings from scratch.",
                },
            ));
        }
    }

    // ── CACHING ─────────────────────────────────────────────────────────────
    {
        let s = score!(&["lru", "least recently used", "cache eviction", "cache"]);
        if s > 0.0 {
            results.push((
                s + 0.4,
                Recommendation {
                    algorithm: "LRU Cache",
                    category: "advanced_topics",
                    tool_name: "advanced_topics.lru_cache",
                    time_complexity: "O(1) get/put",
                    space_complexity: "O(capacity)",
                    confidence: (s + 0.4).min(1.0),
                    reason: "HashMap + doubly-linked list gives O(1) access and eviction.",
                    anti_pattern:
                        "Don't scan the cache linearly on every access — use a map + list.",
                },
            ));
        }
    }
    {
        let s = score!(&["lfu", "least frequently used", "frequency cache"]);
        if s > 0.0 {
            results.push((
                s + 0.5,
                Recommendation {
                    algorithm: "LFU Cache",
                    category: "advanced_topics",
                    tool_name: "advanced_topics.lfu_cache",
                    time_complexity: "O(1) amortized",
                    space_complexity: "O(capacity)",
                    confidence: (s + 0.5).min(1.0),
                    reason:
                        "Frequency buckets with lazy cleanup achieve O(1) amortized operations.",
                    anti_pattern:
                        "Avoid sorting by frequency on every access — maintain frequency buckets.",
                },
            ));
        }
    }

    // ── TREES ────────────────────────────────────────────────────────────────
    {
        let s = score!(&[
            "autocomplete",
            "prefix",
            "trie",
            "word search",
            "dictionary"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.4,
                Recommendation {
                    algorithm: "Trie (Prefix Tree)",
                    category: "advanced_topics",
                    tool_name: "advanced_topics.trie_autocomplete",
                    time_complexity: "O(L) per operation",
                    space_complexity: "O(N·L)",
                    confidence: (s + 0.4).min(1.0),
                    reason:
                        "Trie gives O(L) insert/search by word length, perfect for autocomplete.",
                    anti_pattern: "Don't linear-scan a word list for prefix matching — use a Trie.",
                },
            ));
        }
    }
    {
        let s = score!(&[
            "balanced bst",
            "avl",
            "ordered set",
            "self-balancing",
            "sorted insert"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.4,
                Recommendation {
                    algorithm: "AVL Tree",
                    category: "trees_advanced",
                    tool_name: "trees_advanced.avl_tree",
                    time_complexity: "O(log n)",
                    space_complexity: "O(n)",
                    confidence: (s + 0.4).min(1.0),
                    reason: "Height is strictly bounded; guarantees O(log n) operations.",
                    anti_pattern:
                        "Unbalanced BST degrades to O(n) on sorted input — balance is essential.",
                },
            ));
        }
    }
    {
        let s = score!(&[
            "range sum",
            "range query",
            "update range",
            "prefix sum",
            "segment"
        ]);
        if s > 0.0 {
            results.push((s + 0.4, Recommendation {
                algorithm: "Segment Tree / Fenwick Tree",
                category: "trees_advanced",
                tool_name: "trees_advanced.segment_tree_builder",
                time_complexity: "O(log n) query/update",
                space_complexity: "O(n)",
                confidence: (s + 0.4).min(1.0),
                reason: "Segment trees handle arbitrary range queries in O(log n).",
                anti_pattern: "Don't recompute prefix sums on every update — use a Fenwick tree for O(log n).",
            }));
        }
    }
    {
        let s = score!(&[
            "median",
            "running median",
            "stream median",
            "kth largest",
            "top k"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.4,
                Recommendation {
                    algorithm: "Heap — Median Stream / Top-K",
                    category: "trees_advanced",
                    tool_name: "trees_advanced.median_stream",
                    time_complexity: "O(log n) insert",
                    space_complexity: "O(n)",
                    confidence: (s + 0.4).min(1.0),
                    reason:
                        "Two heaps (max + min) maintain median in O(log n); min-heap for top-k.",
                    anti_pattern:
                        "Never sort on every insertion to find median — maintain two heaps.",
                },
            ));
        }
    }

    // ── DYNAMIC PROGRAMMING ─────────────────────────────────────────────────
    {
        let s = score!(&["knapsack", "0/1 knapsack", "include exclude", "subset sum"]);
        if s > 0.0 {
            results.push((
                s + 0.4,
                Recommendation {
                    algorithm: "DP — 0/1 Knapsack / Subset Sum",
                    category: "dynamic_programming",
                    tool_name: "dynamic_programming.subset_sum",
                    time_complexity: "O(n·W)",
                    space_complexity: "O(W)",
                    confidence: (s + 0.4).min(1.0),
                    reason: "Bottom-up DP fills a 1D capacity array, achieving O(n·W).",
                    anti_pattern:
                        "Don't use recursive backtracking without memoization — it's exponential.",
                },
            ));
        }
    }
    {
        let s = score!(&[
            "longest common subsequence",
            "lcs",
            "edit distance",
            "string similarity",
            "diff"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.4,
                Recommendation {
                    algorithm: "DP — LCS / Edit Distance",
                    category: "dynamic_programming",
                    tool_name: "dynamic_programming.edit_distance",
                    time_complexity: "O(n·m)",
                    space_complexity: "O(min(n,m))",
                    confidence: (s + 0.4).min(1.0),
                    reason: "2D DP table (space-optimized to 1 row) solves in O(n·m).",
                    anti_pattern: "Naive recursion is O(2^n) — always memoize or use bottom-up DP.",
                },
            ));
        }
    }
    {
        let s = score!(&[
            "longest increasing subsequence",
            "lis",
            "increasing sequence"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.5,
                Recommendation {
                    algorithm: "DP — LIS (Patience Sort)",
                    category: "dynamic_programming",
                    tool_name: "dynamic_programming.longest_increasing_sub",
                    time_complexity: "O(n log n)",
                    space_complexity: "O(n)",
                    confidence: (s + 0.5).min(1.0),
                    reason:
                        "Binary search on a patience-sort pile gives O(n log n) — beats O(n²) DP.",
                    anti_pattern: "The naive O(n²) DP is acceptable for n<1000 but fails at scale.",
                },
            ));
        }
    }
    {
        let s = score!(&[
            "coin change",
            "minimum coins",
            "ways to make change",
            "denomination"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.5,
                Recommendation {
                    algorithm: "DP — Coin Change",
                    category: "dynamic_programming",
                    tool_name: "dynamic_programming.coin_change",
                    time_complexity: "O(n·amount)",
                    space_complexity: "O(amount)",
                    confidence: (s + 0.5).min(1.0),
                    reason: "Bottom-up DP fills amounts 1..target, each in O(denominations).",
                    anti_pattern:
                        "Greedy does NOT always work for coin change — use DP for correctness.",
                },
            ));
        }
    }

    // ── GREEDY ───────────────────────────────────────────────────────────────
    {
        let s = score!(&[
            "scheduling",
            "activity selection",
            "interval",
            "meeting rooms",
            "non-overlapping"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.4,
                Recommendation {
                    algorithm: "Greedy — Activity Selection",
                    category: "greedy_algorithms",
                    tool_name: "greedy_algorithms.activity_selection",
                    time_complexity: "O(n log n)",
                    space_complexity: "O(1)",
                    confidence: (s + 0.4).min(1.0),
                    reason: "Sort by end time, greedily pick non-overlapping intervals.",
                    anti_pattern:
                        "Don't use DP for interval scheduling — greedy by end-time is optimal.",
                },
            ));
        }
    }
    {
        let s = score!(&["huffman", "compression", "encoding", "variable length code"]);
        if s > 0.0 {
            results.push((s + 0.5, Recommendation {
                algorithm: "Huffman Coding",
                category: "greedy_algorithms",
                tool_name: "greedy_algorithms.huffman_coding",
                time_complexity: "O(n log n)",
                space_complexity: "O(n)",
                confidence: (s + 0.5).min(1.0),
                reason: "Min-heap greedily merges lowest-frequency nodes for optimal prefix codes.",
                anti_pattern: "Fixed-width encoding wastes space — variable-length codes halve file size.",
            }));
        }
    }

    // ── SORTING ──────────────────────────────────────────────────────────────
    {
        let s = score!(&["sort", "merge sort", "divide and conquer", "stable sort"]);
        if s > 0.0 && !d.contains("binary search") {
            results.push((
                s + 0.2,
                Recommendation {
                    algorithm: "Merge Sort",
                    category: "sorting_searching",
                    tool_name: "sorting_searching.merge_sort",
                    time_complexity: "O(n log n)",
                    space_complexity: "O(n)",
                    confidence: (s + 0.2).min(1.0),
                    reason: "Stable O(n log n) sort — ideal when order preservation matters.",
                    anti_pattern: "Bubble/selection sort is O(n²) — never use for n > 1000.",
                },
            ));
        }
    }

    // ── STACKS & QUEUES ──────────────────────────────────────────────────────
    {
        let s = score!(&["valid parentheses", "bracket matching", "balanced brackets"]);
        if s > 0.0 {
            results.push((
                s + 0.6,
                Recommendation {
                    algorithm: "Stack — Parentheses Validator",
                    category: "stacks_queues",
                    tool_name: "stacks_queues.valid_parentheses",
                    time_complexity: "O(n)",
                    space_complexity: "O(n)",
                    confidence: (s + 0.6).min(1.0),
                    reason: "Push on open bracket, pop and match on close — O(n) single pass.",
                    anti_pattern:
                        "Don't count opens/closes separately — nested structures need a stack.",
                },
            ));
        }
    }
    {
        let s = score!(&[
            "next greater",
            "monotonic stack",
            "next larger element",
            "daily temperatures"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.5,
                Recommendation {
                    algorithm: "Monotonic Stack",
                    category: "stacks_queues",
                    tool_name: "stacks_queues.next_greater",
                    time_complexity: "O(n)",
                    space_complexity: "O(n)",
                    confidence: (s + 0.5).min(1.0),
                    reason: "Monotonic stack processes each element at most twice — O(n) total.",
                    anti_pattern:
                        "Avoid O(n²) nested scan for next-greater — maintain a decreasing stack.",
                },
            ));
        }
    }

    // ── HASHING ──────────────────────────────────────────────────────────────
    {
        let s = score!(&[
            "consistent hashing",
            "distributed",
            "hash ring",
            "load balancing nodes"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.5,
                Recommendation {
                    algorithm: "Consistent Hashing",
                    category: "advanced_topics",
                    tool_name: "advanced_topics.consistent_hashing",
                    time_complexity: "O(log n) lookup",
                    space_complexity: "O(n·v)",
                    confidence: (s + 0.5).min(1.0),
                    reason: "Virtual nodes + ring minimize key remapping on node add/remove.",
                    anti_pattern:
                        "Modulo hashing remaps all keys on node count change — use ring hashing.",
                },
            ));
        }
    }
    {
        let s = score!(&[
            "bloom filter",
            "membership test",
            "probabilistic",
            "false positive"
        ]);
        if s > 0.0 {
            results.push((s + 0.5, Recommendation {
                algorithm: "Bloom Filter",
                category: "advanced_topics",
                tool_name: "advanced_topics.bloom_filter",
                time_complexity: "O(k) per operation",
                space_complexity: "O(m) bits",
                confidence: (s + 0.5).min(1.0),
                reason: "Space-efficient probabilistic set with tunable false-positive rate.",
                anti_pattern: "Don't store full keys when you only need membership — bloom filters use bits.",
            }));
        }
    }

    // ── LINKED LISTS ─────────────────────────────────────────────────────────
    {
        let s = score!(&[
            "cycle detection",
            "floyd",
            "fast slow pointer",
            "linked list cycle"
        ]);
        if s > 0.0 {
            results.push((
                s + 0.5,
                Recommendation {
                    algorithm: "Floyd's Cycle Detection",
                    category: "linked_lists",
                    tool_name: "linked_lists.cycle_detection",
                    time_complexity: "O(n)",
                    space_complexity: "O(1)",
                    confidence: (s + 0.5).min(1.0),
                    reason: "Two-pointer tortoise-and-hare detects cycles in O(n) with O(1) space.",
                    anti_pattern:
                        "Don't use a HashSet to track visited nodes — Floyd's uses no extra space.",
                },
            ));
        }
    }

    // Sort by confidence descending, keep top N (configurable)
    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    const DEFAULT_MAX_RECS: usize = 3;
    let limit: usize = std::env::var("DSA_CLASSIFY_LIMIT")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(DEFAULT_MAX_RECS);
    results.truncate(limit);
    results.into_iter().map(|(_, rec)| rec).collect()
}

# Implementation Plan - Multi-Skill API Logic Injection

Resolve "dead code" warnings by implementing functional API handlers and wiring them to the web server for over 30 algorithmic skills.

## User Review Required

> [!IMPORTANT]
> This is a large-scale integration task affecting 30+ files. We will proceed module by module to ensure stability.

> [!WARNING]
> Some complex skills (e.g., LFU Cache, Segment Trees) might require specific request schemas (e.g., initial capacity, range queries) that differ from simple array/graph inputs.

## Proposed Changes

### 1. Advanced Topics Module
Implement handlers for probabilistic and cache structures.
- [MODIFY] [bloom_filter.rs](file:///c:/dsaengine/src/skills/advanced_topics/bloom_filter.rs)
- [MODIFY] [lfu_cache.rs](file:///c:/dsaengine/src/skills/advanced_topics/lfu_cache.rs)
- [MODIFY] [lru_cache.rs](file:///c:/dsaengine/src/skills/advanced_topics/lru_cache.rs)
- [MODIFY] [suffix_array.rs](file:///c:/dsaengine/src/skills/advanced_topics/suffix_array.rs)

### 2. Stacks & Queues Module
Implement handlers for data structure simulations and extensions.
- [MODIFY] [stack_via_queues.rs](file:///c:/dsaengine/src/skills/stacks_queues/stack_via_queues.rs)
- [MODIFY] [queue_via_stacks.rs](file:///c:/dsaengine/src/skills/stacks_queues/queue_via_stacks.rs)
- [MODIFY] [min_stack.rs](file:///c:/dsaengine/src/skills/stacks_queues/min_stack.rs)

### 3. Trees Advanced Module
Implement handlers for specialized tree-based queries.
- [MODIFY] [top_k_elements.rs](file:///c:/dsaengine/src/skills/trees_advanced/top_k_elements.rs)
- [MODIFY] [segment_tree_query.rs](file:///c:/dsaengine/src/skills/trees_advanced/segment_tree_query.rs)
- [MODIFY] [segment_tree_builder.rs](file:///c:/dsaengine/src/skills/trees_advanced/segment_tree_builder.rs)

### 4. Sorting & Searching Module
- [MODIFY] [visualizer.rs](file:///c:/dsaengine/src/skills/sorting_searching/visualizer.rs)
- [MODIFY] [rotated_search.rs](file:///c:/dsaengine/src/skills/sorting_searching/rotated_search.rs)

### 5. Graphs Module (Source of truth)
- [MODIFY] [dijkstra.rs](file:///c:/dsaengine/src/skills/graphs/dijkstra.rs) (Restore functional endpoint)

### 6. Arrays & Strings Module (The "Bulk" Injection)
Implement handlers for 15+ algorithmic patterns (KMP, Rabin-Karp, Kadane's, etc.).
- [MODIFY] [anagram_detector.rs](file:///c:/dsaengine/src/skills/arrays_strings/anagram_detector.rs)
- [MODIFY] [array_rotation.rs](file:///c:/dsaengine/src/skills/arrays_strings/array_rotation.rs)
- ... and others in the category.

### 7. Web Server Routing
- [MODIFY] [web_server.rs](file:///c:/dsaengine/src/web_server.rs)
    - Import and register all new `post` handlers in `api_routes`.

## Verification Plan

### Automated Tests
- Run `cargo check` after each module's injection to catch type mismatches or macro errors.
- Run `cargo test` if available for specific modules.

### Manual Verification
- Use the `curl` tool to sample 1-2 skills per module to ensure JSON serialization and algorithm logic are correctly bridged.
- Example for Bloom Filter:
    - POST `/api/v1/advanced/bloom_filter` with bits/hashes config.

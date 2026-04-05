# 🚀 dsaEngine

**dsaEngine** is a high-performance, strictly memory-safe, computationally rigorous Library of Data Structures & Algorithms built exclusively for **Agentic AI Systems**. It provides 150+ mathematically accurate algorithm templates designed to be injected dynamically into AI reasoning pipelines. 

To prevent infinite loops, stack-overflows, and massive vector allocations that crash typical containerized environments, **dsaEngine** imposes strict Zero-Copy (`&[T]`) and Zero-Recursion (Iterative `Vec / VecDeque / O(1)` Arena) constraints.

---

## 🧠 The Architecture

AI Models process tokens sequentially and frequently suffer from memory degradation when executing algorithms recursively. dsaEngine solves this via:
1. **Mathematical O(1) Limits**: We deploy algorithms like *Morris Traversal* for trees, *Floyd's Tortoise & Hare* for cycle detection, and *Bitwise Bounds* for Fenwick Trees to achieve absolute `O(1)` memory overhead where theoretically possible.
2. **Iterative Call-Stacks**: Algorithms traditionally heavily recursive (Merge Sort, Backtracking, DFS, Tree Heights, Lowest Common Ancestor) have been structurally rewritten utilizing native loop invariants, explicit DP stacks, and bounding arrays `[left, right)`.
3. **Paging System**: To minimize LLM Context Window exhaustion, the 12 categories are structured into "Pages". You boot the engine by declaring strictly what features you need inside `Cargo.toml`.

---

## 📖 The 12 Pages (Skill Categories)

You load specifically what you need by activating Cargo features (`--features "arrays graphs"`), keeping binaries microscopically tight. 

| Feature Flag | Skill Category | Core Algorithms Present |
|---|---|---|
| `fundamentals` | **Fundamentals** | Mathematics, Bitwise Manipulations, Base Conversions |
| `arrays` | **Arrays & Strings** | Two-Pointers, Sliding Windows, String Matching (KMP/Rabin-Karp) |
| `linked_lists` | **Linked Lists** | Ring Cycles, O(N) Multi-Pass Copies, Reversing, Intersections |
| `stacks` | **Stacks & Queues** | Monotonic Sequences, Asteroids, Min-Stacks, Valid Parentheses |
| `trees` | **Binary Trees** | O(1) Traversals, Iterative LCAs, Invert, Serialization |
| `trees_adv` | **Advanced Trees** | Array-Backed Segment Trees, Tries, Fenwick, Heaps |
| `graphs` | **Graphs** | Kahn's Topo Sort, Dijkstra's SSSP, Bellman-Ford, Kruskal/Prim |
| `dp` | **Dynamic Programming** | Knapsack, Climbing Stairs, Tabulation Matrices |
| `greedy` | **Greedy Algorithms**| Huffman Coding, Interval Scheduling, Jump Game |
| `backtracking` | **Backtracking** | N-Queens (Bitset Pruned), Sudoku, Array Subsets |
| `sorting` | **Sorting & Searching**| Bottom-Up Merge Sort, Segment-Aware Binary Search |
| `advanced` | **Advanced Topics** | Lazy Seg-Trees, Suffix Arrays, Skip Lists, LFU/LRU Caches |

---

## 🛠 Usage Example

Import a skill natively into your runner:

```rust
use dsaengine::skills::linked_lists::cycle_detection::CycleDetection;

fn main() {
    let arena = vec![(1, 100), (2, 200), (0, 300)]; // Circular linked list via Array Arena
    let has_cycle = CycleDetection::has_cycle(&arena, 0);
    println!("Cycle detected: {}", has_cycle);
}
```

Every module inherently integrates with `AgentLogger` and implements the runtime `Complexity` trait exposing deterministic `$O$` metric bounding!

```rust
use dsaengine::Complexity;
// -> "O(log N) — Depth is mathematically strictly bounded..."
```

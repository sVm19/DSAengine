# 🚀 dsaEngine

**dsaEngine** is a high-performance, deterministic **MCP (Model Context Protocol) server** providing **142+ production-grade DSA algorithms** for AI coding agents (Cursor, Windsurf, Claude Code, GitHub Copilot, and more).

## ⚡ Zero-Cost Algorithm Advisor

DSAEngine runs **locally** on your machine and integrates with your favorite coding agent via the **Model Context Protocol**:

- ✅ **No API calls** — 100% local, zero cost per use
- ✅ **142+ algorithms** — Optimal implementations categorized & indexed
- ✅ **Deterministic** — No randomness, repeatable recommendations
- ✅ **7 agents** — Cursor, Windsurf, Claude Code, GitHub Copilot, Kiro, Augment, Antigravity
- ✅ **MCP compatible** — JSON-RPC over stdio, zero hosting required

### How It Works

```
You ask Cursor: "Find shortest path in weighted graph with negative edges"
        ↓
Cursor sees .cursorrules → "consult DSAEngine first"
        ↓
Cursor calls: dsa_classify("Find shortest path in weighted graph with negative edges")
        ↓
DSAEngine responds: "Bellman-Ford Algorithm, O(V·E), confidence: 0.95"
        ↓
Cursor calls: graphs.bellman_ford(edges, source)
        ↓
DSAEngine returns: Rust code + pseudocode + result
        ↓
Cursor writes optimized code
```

---

## 🚀 Quick Start (30 seconds)

### 1. Install DSAEngine
```bash
cargo install dsaengine
```

### 2. Setup Your Agent
- **Cursor**: [Setup Guide](docs/setup/CURSOR_SETUP.md)
- **Windsurf**: [Setup Guide](docs/setup/WINDSURF_SETUP.md)
- **Claude**: [Setup Guide](docs/setup/CLAUDE_SETUP.md)

### 3. Start Coding
Ask your agent to write an algorithm — DSAEngine will ensure it's optimal.

---

## 📖 The 142+ Algorithm Library

Organized into **13 categories**:

---

## 📖 The 12 Pages (Skill Categories)

You load specifically what you need by activating Cargo features (`--features "arrays graphs"`), keeping binaries microscopically tight. 

| Feature Flag | Skill Category | Examples |
|---|---|---|
| `arrays` | **Arrays & Strings** | Two-Sum, KMP Search, Sliding Window, Kadane's Algorithm |
| `linked_lists` | **Linked Lists** | Cycle Detection, Reverse, Merge Sorted, Intersection |
| `stacks_queues` | **Stacks & Queues** | Min Stack, Valid Parentheses, Next Greater, Histogram |
| `trees_binary` | **Binary Trees** | Traversals, LCA, Serialize, Invert, Max Path Sum |
| `trees_advanced` | **Advanced Trees** | Trie, Segment Tree, Fenwick Tree, AVL Tree, Red-Black |
| `graphs` | **Graphs** | Dijkstra, Bellman-Ford, BFS/DFS, MST (Kruskal/Prim) |
| `dynamic_programming` | **Dynamic Programming** | Coin Change, LCS, Edit Distance, Knapsack |
| `greedy_algorithms` | **Greedy** | Activity Selection, Huffman Coding, Jump Game |
| `backtracking` | **Backtracking** | N-Queens, Sudoku, Combinations, Permutations |
| `sorting_searching` | **Sorting & Searching** | Merge Sort, Quick Sort, Binary Search |
| `dsa_fundamentals` | **Fundamentals** | Big-O Analysis, Two Pointers, Sliding Window |
| `advanced_topics` | **Advanced** | LRU/LFU Cache, Union-Find, Bloom Filter, Trie Autocomplete |

---

## 🎯 Usage

### As MCP Server (Recommended for Agents)

After setup, your agent automatically calls DSAEngine:

```
Cursor:
"Write a function to find all connected components in an undirected graph"

→ Cursor calls dsa_classify("connected components undirected graph")

DSAEngine responds:
{
  "algorithm": "Union-Find",
  "confidence": 0.98,
  "time_complexity": "O(α(n))",
  "space_complexity": "O(n)",
  "tool_name": "advanced_topics.union_find",
  "reason": "Optimal for connectivity queries with near-O(1) amortized complexity"
}

→ Cursor calls advanced_topics.union_find(...)

→ You get optimized code
```

### As Rust Library

```rust
use dsaengine::skills::graphs::dijkstra::Dijkstra;

fn main() {
    let edges = vec![(0, 1, 4), (1, 2, 2), (0, 2, 5)];
    let (distances, paths) = Dijkstra::shortest_path(&edges, 0);
    println!("Shortest distances: {:?}", distances);
}
```

### As HTTP API

```bash
# Run HTTP server (default: localhost:8000)
dsaengine

# Test:
curl -X POST http://localhost:8000/api/v1/graphs/dijkstra \
  -H "Content-Type: application/json" \
  -d '{...}'

# Swagger UI: http://localhost:8000/swagger-ui/
```

---

## 🛠 Installation Methods

### Method 1: Cargo (Easiest)
```bash
cargo install dsaengine
```

Requires: [Rust 1.70+](https://rustup.rs/)

### Method 2: Pre-built Binaries
Download from [GitHub Releases](https://github.com/yourusername/dsaengine/releases):
- `dsaengine` (macOS/Linux)
- `dsaengine.exe` (Windows)
- `dsaengine-arm64` (macOS Apple Silicon)

### Method 3: From Source
```bash
git clone https://github.com/yourusername/dsaengine
cd dsaengine
cargo build --release
# Binary: target/release/dsaengine
```

---

## 🔧 Setup Guides

- **[Cursor Setup](docs/setup/CURSOR_SETUP.md)** — Step-by-step for Cursor users
- **[Windsurf Setup](docs/setup/WINDSURF_SETUP.md)** — Step-by-step for Windsurf users
- **[Claude Code Setup](docs/setup/CLAUDE_SETUP.md)** — Step-by-step for Claude users

---

## 📝 Commands

```bash
# Run MCP stdio server (for agents)
dsaengine --mcp

# Run HTTP web server (for API/testing)
dsaengine

# Generate rules files for agents
dsaengine --install

# Show help
dsaengine --help
```

---

## 🧠 Use Cases

### ✅ Optimal Algorithm Selection
Ask Cursor/Windsurf to solve algorithmic problems — DSAEngine ensures optimal approach.

### ✅ Deterministic Recommendations
No randomness, no LLM hallucinations — pure keyword-based algorithm matching.

### ✅ O(n) → O(log n) Optimization
Catch brute-force approaches and suggest optimal alternatives automatically.

### ✅ Zero API Cost
Runs completely local — no external API calls, no rate limits.

---

## 📊 Performance

| Metric | Value |
|--------|-------|
| **Binary Size** | ~25 MB (release build) |
| **Startup Time** | <100ms |
| **Recommendation Latency** | <5ms |
| **Memory Usage** | ~50 MB at runtime |
| **Algorithms Available** | 142+ |

---

## 🤝 Contributing

DSAEngine welcomes contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

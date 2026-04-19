# MCP Integration Architecture

## Overview

DSAEngine is implemented as a **Model Context Protocol (MCP) Server** that runs as a local child process, communicating with coding agents via JSON-RPC 2.0 over stdin/stdout.

---

## Protocol Flow

### Initialization

```json
Agent → Server:
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize"
}

Server → Agent:
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2024-11-05",
    "serverInfo": {
      "name": "DSAEngine",
      "version": "0.1.0",
      "description": "142+ optimal DSA algorithm implementations"
    },
    "capabilities": {
      "tools": { "listChanged": false }
    }
  }
}
```

### Tool Discovery

```json
Agent → Server:
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list"
}

Server → Agent:
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "tools": [
      {
        "name": "dsa_classify",
        "description": "Classifies algorithm problems into optimal recommendations",
        "inputSchema": { ... }
      },
      {
        "name": "graphs.dijkstra",
        "description": "Dijkstra's shortest path algorithm",
        "inputSchema": { ... }
      },
      ... (142+ tools total)
    ]
  }
}
```

### Tool Invocation

```json
Agent → Server:
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "dsa_classify",
    "arguments": {
      "description": "Find shortest path with negative weights",
      "num_recommendations": 3
    }
  }
}

Server → Agent:
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "recommendations": [
      {
        "algorithm": "Bellman-Ford",
        "confidence": 0.95,
        "time_complexity": "O(V·E)",
        "tool_name": "graphs.bellman_ford"
      },
      ...
    ]
  }
}
```

---

## Supported Agents

| Agent | Config File | MCP Column | Stdio Support |
|-------|------------|-----------|--------------|
| **Cursor** | `~/.cursor/mcp.json` | GUI or JSON | ✅ Full |
| **Windsurf** | `~/.windsurf/mcp_config.json` | JSON | ✅ Full |
| **Claude Code** | `~/.claude/claude_desktop_config.json` | JSON | ✅ Full |
| **GitHub Copilot** | VS Code `settings.json` | GUI | ✅ Full |
| **Kiro** | `.vscode/mcp.json` | JSON | ✅ Full |
| **Augment Code** | Settings Panel | GUI | ✅ Full |
| **Google Antigravity** | Agent Manager | GUI | ✅ Full |

---

## Meta-Tools

### `dsa_classify`

**Purpose**: Classify a problem into optimal algorithm recommendations.

**Input**:
```json
{
  "description": "string — problem description",
  "num_recommendations": "number — default 3, max 10",
  "output_mode": "string — 'ranks_only' | 'brief' | 'full'"
}
```

**Output**:
```json
{
  "recommendations": [
    {
      "algorithm": "Algorithm Name",
      "category": "graphs | arrays | dp | stacks | ...",
      "tool_name": "category.algorithm_name",
      "confidence": 0.95,
      "time_complexity": "O(E log V)",
      "space_complexity": "O(V)",
      "reason": "Why this is optimal",
      "anti_pattern": "What to avoid"
    }
  ]
}
```

### `dsa_suggest`

**Purpose**: Get optimization tips for existng code.

**Input**:
```json
{
  "code": "string — code snippet",
  "language": "rust | python | javascript | java",
  "problem_domain": "string — optional problem description"
}
```

**Output**:
```json
{
  "issues": [
    {
      "type": "complexity | correctness | pattern",
      "severity": "high | medium | low",
      "message": "...",
      "suggestion": "...",
      "tool": "category.algorithm_name"
    }
  ]
}
```

---

## Rules File Injection

DSAEngine auto-generates rules files that are **prepended to every prompt** by the agent:

### `.cursorrules` (Cursor)
Instructs Cursor to consult DSAEngine before writing any algorithm code.

### `.windsurfrules` (Windsurf)
Same for Windsurf.

### `CLAUDE.md` (Claude Code)
Same for Claude.

---

## Deterministic Classification

DSAEngine uses **keyword-based pattern matching** (not AI) to classify problems:

```rust
// Simplified logic
if description.contains("shortest") && description.contains("weighted") {
    // Dijkstra or Bellman-Ford
}
if description.contains("cache") && description.contains("evict") {
    // LRU or LFU
}
if description.contains("substring") && description.contains("pattern") {
    // KMP or Rabin-Karp
}
// ... 100+ rules
```

**Why deterministic?**
- Fast (<5ms)
- Repeatable (same input = same output)
- No API calls
- No LLM hallucinations

---

## Output Modes

When calling algorithm tools, DSAEngine can return:

| Mode | Returns | Use Case |
|------|---------|----------|
| `result` | Just the answer | Quick verification |
| `code` | Rust source code | Adapt to other languages |
| `pseudocode` | Language-agnostic pseudo | Understand the approach |
| `full` | All of the above + complexity | Complete understanding |

---

## Integration Workflow

```
┌─────────────────────────────────────────────┐
│ User types in Cursor/Windsurf/Claude        │
└──────────────┬──────────────────────────────┘
               ↓
┌─────────────────────────────────────────────┐
│ Agent reads .cursorrules/.windsurfrules     │
│ "Always consult DSAEngine first"            │
└──────────────┬──────────────────────────────┘
               ↓
┌─────────────────────────────────────────────┐
│ Agent spawns: dsaengine --mcp               │
│ Communicates via JSON-RPC / stdio           │
└──────────────┬──────────────────────────────┘
               ↓
┌─────────────────────────────────────────────┐
│ Agent calls: dsa_classify(description)      │
│ DSAEngine returns: Top 3 recommendations    │
└──────────────┬──────────────────────────────┘
               ↓
┌─────────────────────────────────────────────┐
│ Agent analyzes recommendations              │
│ Selects optimal algorithm                   │
└──────────────┬──────────────────────────────┘
               ↓
┌─────────────────────────────────────────────┐
│ Agent calls: graphs.dijkstra(...)           │
│ DSAEngine returns: Code + pseudocode        │
└──────────────┬──────────────────────────────┘
               ↓
┌─────────────────────────────────────────────┐
│ Agent writes optimized code                 │
│ User gets production-ready solution         │
└─────────────────────────────────────────────┘
```

---

## Offline-First Architecture

- ✅ **No internet required** — Runs on localhost
- ✅ **No authentication** — No API keys
- ✅ **No rate limits** — Use as much as you want
- ✅ **No telemetry** — Your data stays local
- ✅ **No dependencies** — Single binary

---

## Performance Characteristics

| Operation | Latency | Notes |
|-----------|---------|-------|
| Agent initialization | ~50ms | Binary startup + handshake |
| Algorithm classification | <5ms | Keyword matching only |
| Tool listing | ~10ms | JSON serialization |
| Code generation | 50-200ms | Dependent on algorithm |

---

## Security Model

### What DSAEngine Does NOT Do
- ❌ Execute arbitrary code
- ❌ Store user data
- ❌ Make network requests
- ❌ Access filesystem (except current directory)
- ❌ Modify system state

### What DSAEngine ONLY Does
- ✅ Classify problem descriptions
- ✅ Generate algorithm code
- ✅ Suggest optimizations
- ✅ Return complexity analysis

---

## Debugging

### Enable Verbose Logging

```bash
# Run with stderr output
dsaengine --mcp 2>&1
```

### Manual Testing

```bash
# Test initialization
echo '{"jsonrpc":"2.0","id":1,"method":"initialize"}' | dsaengine --mcp

# Test tool listing
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' | dsaengine --mcp | jq '.result.tools | length'

# Test classification
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"dsa_classify","arguments":{"description":"find shortest path"}}}' | dsaengine --mcp | jq '.result.recommendations'
```

---

## Troubleshooting

### "stdio server ready" not printed
- Agent may have closed stdin immediately
- Test manually: `dsaengine --mcp`

### "Method not found" errors
- Check method name spelling
- Valid methods: `initialize`, `tools/list`, `tools/call`

### Tool recommendation not appearing in agent
- Verify `.cursorrules` / `.windsurfrules` exists
- Restart the agent
- Check agent's MCP logs

---

## Future Enhancements

- [ ] HTTP transport option (for network deployment)
- [ ] WebAssembly build (for browser-based IDEs)
- [ ] Additional languages (Python, JavaScript generators)
- [ ] Machine learning-based recommendations
- [ ] Custom algorithm contribution system

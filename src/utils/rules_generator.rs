//! Rules File Generator
//!
//! Generates agent-specific instruction files for all 7 supported coding agents.
//! Each file is injected into the agent's system prompt before every user message,
//! making the LLM consult DSAEngine automatically — zero extra AI cost.

use std::fs;
use std::path::Path;

/// All output files produced by `dsaengine --install`
pub struct InstallOutput {
    pub files_written: Vec<String>,
    pub mcp_config: String,
}

/// Core rules content injected into every agent's file.
/// Concise enough to not waste tokens, specific enough to drive behaviour.
const RULES_BODY: &str = r#"
## DSAEngine — Algorithmic Advisor (Active)

You have access to **DSAEngine MCP tools** — a library of 142 production-grade DSA
algorithm implementations. These are deterministic, O(n)–O(n log n) optimal solutions.

### Expert Engineering Mandate

You are an expert software engineer with access to a deterministic DSA Engine that
provides optimal implementations of algorithms.

[DSA ENGINE CONTEXT]

You have access to a deterministic DSA Engine with the following capabilities:
- Sliding Window (O(n)) for substring/subarray optimization
- HashMap-based lookup (O(1)) for search problems
- Dijkstra / BFS for graph traversal
- Dynamic Programming for overlapping subproblems
- Heap / Priority Queue for top-k and scheduling
- Two-pointer technique for sorted arrays
- All algorithm skills exposed from the DSAEngine skills folder

CRITICAL RULES:
1. ALWAYS check if a DSAEngine tool can solve the problem before writing code.
2. NEVER implement brute-force or suboptimal solutions if a tool exists.
3. PREFER optimal time and space complexity over simplicity.
4. DO NOT write nested loops or inefficient logic when better approaches exist.
5. USE DSAEngine tools for graph problems, array/string optimization, dynamic
   programming, tree/heap operations, searching, and sorting optimizations.

DECISION PROCESS:
1. Classify the problem type.
2. Check available DSAEngine tools.
3. Call the most optimal tool.
4. Integrate the result into code.

### Problem Analysis Checklist

Before coding, analyze the problem carefully:
1. Identify the problem category: Array/String, Graph, Dynamic Programming,
   Tree, or Greedy/Heap.
2. Detect optimization signals: "shortest", "minimum", "maximum",
   "substring", "subarray", "path", "graph", "optimize", or "efficient".
3. Match the category and signals with DSAEngine tools.
4. Prefer tools with O(n) or O(n log n) complexity.
5. Avoid nested loops, brute-force search, and repeated computation.
6. Choose the best tool and use it.

### Optimization Mode

When the user provides existing code, convert naive code into production-grade
optimized code:
1. Analyze the code before changing it.
2. Detect inefficiencies: nested loops, repeated work, and poor data structures.
3. Replace inefficiencies with optimal algorithms using DSAEngine tools.
4. Return the optimized version of the code.
5. State the time complexity improvement.
6. Explain why the new approach is better.

OUTPUT REQUIREMENTS:
- Always aim for O(n) or O(n log n) when possible.
- Clearly use optimized approaches.
- Avoid redundant computation.

STRICT INSTRUCTIONS:
- Replace brute-force approaches with optimal algorithms.
- Avoid O(n^2) solutions unless absolutely necessary.
- Prefer DSAEngine tools over manual implementation.

### MANDATORY RULE — No Brute-Force Without Consulting DSAEngine

Before writing code that involves:
- Arrays, strings, or numeric sequences
- Graphs, trees, or linked structures
- Caching, state management, or lookup tables
- Sorting, searching, or ordering
- Pattern matching or string processing

You MUST:
1. Call `dsa_classify` with the problem description
2. Review the recommended algorithm and its complexity
3. Call the specific tool (e.g. `graphs.dijkstra`) to get the optimal implementation
4. Integrate the result — never reinvent what DSAEngine already provides

### Anti-Pattern Blacklist (Never Generate These)

| BAD Pattern | WHY | USE INSTEAD |
|---|---|---|
| Nested `for` loop for pair-finding | O(n²) | `two_sum_matcher` — O(n) HashMap |
| Linear scan in sorted array | O(n) | `binary_search_template` — O(log n) |
| `Array.sort()` then find prefix | O(n log n) wasted | Sliding window — O(n) |
| Reset cache on every miss | O(n) per access | `lru_cache` / `lfu_cache` — O(1) |
| BFS/DFS for every connectivity query | O(V+E) per query | `union_find` — O(α(n)) |
| Modulo hash for distributed keys | All keys remap | `consistent_hashing` — O(log n) |

### Quick Algorithm Reference

**Graphs**: dijkstra · bellman_ford · bfs_generator · dfs_generator · topological_sort · mst_kruskal_prim · union_find  
**Arrays**: two_sum_matcher · kadanes_algorithm · sliding_window_detector · boyer_moore_voting · subarray_sum  
**Strings**: kmp_search · rabin_karp · anagram_detector · longest_substring · z_algorithm  
**Trees**: avl_tree · red_black_tree · segment_tree_builder · fenwick_tree · median_stream · trie_autocomplete  
**DP**: coin_change · edit_distance · longest_increasing_sub · lcs_solver · subset_sum · knapsack  
**Greedy**: activity_selection · huffman_coding · fractional_knapsack · task_scheduler  
**Caching**: lru_cache · lfu_cache · bloom_filter · consistent_hashing  
**Sorting**: merge_sort · quick_sort · counting_sort · binary_search_template  

### How to Use DSAEngine Tools

```
// Step 1: Classify the problem
dsa_classify({ description: "I need to find shortest path in a city map" })
// → Returns: dijkstra (O(E log V)), confidence: 0.9

// Step 2: Get the implementation
graphs.dijkstra({ edges: [...], source: 0 })
// → Returns: optimal path, distance array, complexity metadata
```
"#;

/// Generates all 7 agent rules files and the universal MCP config snippet.
pub fn install(project_dir: &Path) -> Result<InstallOutput, std::io::Error> {
    let mut files_written = Vec::new();

    // ── 1. CLAUDE.md — Claude Code + Google Antigravity ──────────────────────
    let claude_path = project_dir.join("CLAUDE.md");
    let claude_content = format!(
        "# DSAEngine Integration Rules\n\
        > Auto-generated by `dsaengine --install`. Do not delete.\n\
        {RULES_BODY}"
    );
    write_file(&claude_path, &claude_content)?;
    files_written.push(claude_path.display().to_string());

    // ── 2. .cursorrules — Cursor IDE ─────────────────────────────────────────
    let cursor_path = project_dir.join(".cursorrules");
    let cursor_content = format!(
        "# DSAEngine — Cursor Rules\n\
        # Auto-generated by `dsaengine --install`. Do not delete.\n\
        {RULES_BODY}"
    );
    write_file(&cursor_path, &cursor_content)?;
    files_written.push(cursor_path.display().to_string());

    // ── 3. .windsurfrules — Windsurf IDE ─────────────────────────────────────
    let windsurf_path = project_dir.join(".windsurfrules");
    let windsurf_content = format!(
        "# DSAEngine — Windsurf Rules\n\
        # Auto-generated by `dsaengine --install`. Do not delete.\n\
        {RULES_BODY}"
    );
    write_file(&windsurf_path, &windsurf_content)?;
    files_written.push(windsurf_path.display().to_string());

    // ── 4. .github/copilot-instructions.md — GitHub Copilot ─────────────────
    let gh_dir = project_dir.join(".github");
    fs::create_dir_all(&gh_dir)?;
    let copilot_path = gh_dir.join("copilot-instructions.md");
    let copilot_content = format!(
        "# DSAEngine — GitHub Copilot Instructions\n\
        <!-- Auto-generated by `dsaengine --install`. Do not delete. -->\n\
        {RULES_BODY}"
    );
    write_file(&copilot_path, &copilot_content)?;
    files_written.push(copilot_path.display().to_string());

    // ── 5. .kiro/steering/dsaengine.md — Kiro (AWS IDE) ──────────────────────
    let kiro_dir = project_dir.join(".kiro").join("steering");
    fs::create_dir_all(&kiro_dir)?;
    let kiro_path = kiro_dir.join("dsaengine.md");
    let kiro_content = format!(
        "# DSAEngine Steering File\n\
        <!-- Auto-generated by `dsaengine --install`. Do not delete. -->\n\
        {RULES_BODY}"
    );
    write_file(&kiro_path, &kiro_content)?;
    files_written.push(kiro_path.display().to_string());

    // ── 6. augment-guidelines.md — Augment Code ──────────────────────────────
    let augment_path = project_dir.join("augment-guidelines.md");
    let augment_content = format!(
        "# DSAEngine — Augment Code Guidelines\n\
        <!-- Auto-generated by `dsaengine --install`. Do not delete. -->\n\
        {RULES_BODY}"
    );
    write_file(&augment_path, &augment_content)?;
    files_written.push(augment_path.display().to_string());

    // ── 7. .agent/rules/dsaengine.md — Google Antigravity ────────────────────
    let agent_dir = project_dir.join(".agent").join("rules");
    fs::create_dir_all(&agent_dir)?;
    let agent_path = agent_dir.join("dsaengine.md");
    let agent_content = format!(
        "# DSAEngine — Antigravity Rules\n\
        <!-- Auto-generated by `dsaengine --install`. Do not delete. -->\n\
        {RULES_BODY}"
    );
    write_file(&agent_path, &agent_content)?;
    files_written.push(agent_path.display().to_string());

    // ── Universal MCP config snippet (saved + printed) ────────────────────────
    let binary_path = std::env::current_exe()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "dsaengine".to_string());

    let mcp_config = format!(
        r#"{{
  "mcpServers": {{
    "dsaengine": {{
      "command": "{binary_path}",
      "args": ["--mcp"],
      "description": "DSAEngine — optimal DSA algorithm advisor for vibe coders"
    }}
  }}
}}"#
    );

    // Save the config next to project dir
    let config_dir = project_dir.join(".dsaengine");
    fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("mcp-config.json");
    write_file(&config_path, &mcp_config)?;
    files_written.push(config_path.display().to_string());

    Ok(InstallOutput {
        files_written,
        mcp_config,
    })
}

fn write_file(path: &Path, content: &str) -> Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn install_writes_dsa_first_mandate_to_agent_rules() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("dsaengine-rules-test-{unique}"));

        let output = install(&dir).expect("install rules into temp dir");

        let claude = fs::read_to_string(dir.join("CLAUDE.md")).expect("read generated CLAUDE.md");
        let cursor =
            fs::read_to_string(dir.join(".cursorrules")).expect("read generated cursor rules");
        let windsurf =
            fs::read_to_string(dir.join(".windsurfrules")).expect("read generated windsurf rules");

        for content in [claude, cursor, windsurf] {
            assert!(content.contains("ALWAYS check if a DSAEngine tool can solve the problem"));
            assert!(content.contains("[DSA ENGINE CONTEXT]"));
            assert!(content.contains("Sliding Window (O(n))"));
            assert!(
                content.contains("All algorithm skills exposed from the DSAEngine skills folder")
            );
            assert!(content.contains("Avoid O(n^2) solutions unless absolutely necessary"));
            assert!(content.contains("Problem Analysis Checklist"));
            assert!(content.contains("Array/String, Graph, Dynamic Programming"));
            assert!(content.contains("\"shortest\", \"minimum\", \"maximum\""));
            assert!(content
                .contains("Avoid nested loops, brute-force search, and repeated computation"));
            assert!(content.contains("Choose the best tool and use it"));
            assert!(content.contains("Optimization Mode"));
            assert!(content.contains("nested loops, repeated work, and poor data structures"));
            assert!(content.contains("Return the optimized version of the code"));
            assert!(content.contains("time complexity improvement"));
            assert!(content.contains("Call `dsa_classify`"));
            assert!(content.contains("Always aim for O(n) or O(n log n)"));
        }
        assert!(output.mcp_config.contains("\"args\": [\"--mcp\"]"));
        assert!(output.files_written.len() >= 8);

        let _ = fs::remove_dir_all(dir);
    }
}

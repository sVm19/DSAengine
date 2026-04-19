use reqwest::Client;
use serde_json::{json, Value};
use std::net::TcpListener;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

struct TestServer {
    child: Child,
    base_url: String,
    api_key: String,
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral test port");
    listener
        .local_addr()
        .expect("read ephemeral test port")
        .port()
}

async fn start_server() -> TestServer {
    let port = free_port();
    let api_key = format!("test-key-{port}");
    let binary = env!("CARGO_BIN_EXE_dsaengine");

    let child = Command::new(binary)
        .env("PORT", port.to_string())
        .env("MASTER_API_2026", &api_key)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn dsaengine test server");

    let base_url = format!("http://127.0.0.1:{port}");
    let client = Client::new();
    let health_url = format!("{base_url}/health");

    for _ in 0..80 {
        match client.get(&health_url).send().await {
            Ok(response) if response.status().is_success() => {
                let body: Value = response.json().await.expect("parse health JSON");
                if body.get("status").and_then(Value::as_str) == Some("up") {
                    return TestServer {
                        child,
                        base_url,
                        api_key,
                    };
                }
            }
            _ => {}
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    panic!("dsaengine test server did not become ready on port {port}");
}

async fn post_tool(client: &Client, server: &TestServer, path: &str, payload: Value) -> Value {
    let url = format!("{}/api/v1/{path}", server.base_url);
    let response = client
        .post(&url)
        .header("X-API-KEY", &server.api_key)
        .json(&payload)
        .send()
        .await
        .unwrap_or_else(|e| panic!("POST {path} failed: {e}"));

    let status = response.status();
    let body: Value = response
        .json()
        .await
        .unwrap_or_else(|e| panic!("POST {path} returned non-JSON body: {e}"));

    assert!(
        status.is_success(),
        "POST {path} returned HTTP {status}: {body}"
    );
    assert_eq!(
        body.get("status").and_then(Value::as_str),
        Some("success"),
        "POST {path} returned non-success body: {body}"
    );

    body
}

async fn call_mcp_tool(
    client: &Client,
    server: &TestServer,
    name: &str,
    arguments: Value,
) -> Value {
    let url = format!("{}/mcp", server.base_url);
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": name,
                "arguments": arguments
            }
        }))
        .send()
        .await
        .unwrap_or_else(|e| panic!("MCP tools/call {name} failed: {e}"));

    let status = response.status();
    let body: Value = response
        .json()
        .await
        .unwrap_or_else(|e| panic!("MCP tools/call {name} returned non-JSON body: {e}"));

    assert!(
        status.is_success(),
        "MCP tools/call {name} returned HTTP {status}: {body}"
    );
    assert!(
        body.get("error").is_none(),
        "MCP tools/call {name} returned JSON-RPC error: {body}"
    );

    body
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn recovered_phase1_endpoints_return_success() {
    let server = start_server().await;
    let client = Client::new();

    let classified = call_mcp_tool(
        &client,
        &server,
        "dsa_classify",
        json!({
            "description": "Find shortest path in a weighted city route graph with distances"
        }),
    )
    .await;
    let first_tool = classified["result"]["structuredContent"]["recommendations"][0]["tool_name"]
        .as_str()
        .expect("classifier returns top tool");
    assert_eq!(first_tool, "graphs.dijkstra");

    let explore_url = format!("{}/api/v1/explore", server.base_url);
    let explore: Value = client
        .get(explore_url)
        .header("X-API-KEY", &server.api_key)
        .send()
        .await
        .expect("GET /api/v1/explore")
        .json()
        .await
        .expect("parse explore JSON");
    let skill_count = explore
        .get("skill_count")
        .and_then(Value::as_u64)
        .expect("explore includes skill_count");
    assert!(
        skill_count >= 100,
        "expected broad route registry, got {skill_count}"
    );

    let cases = [
        (
            "dsa_fundamentals/big_o_analyzer",
            json!({"duration_micros": 50, "operation_count": 1}),
        ),
        (
            "dsa_fundamentals/cyclic_sort_pattern",
            json!({"values": [3, 1, 2]}),
        ),
        (
            "dsa_fundamentals/fast_slow_pointer",
            json!({"next": [1, 2, 0], "start": 0}),
        ),
        (
            "dsa_fundamentals/in_place_reversal",
            json!({"values": [1, 2, 3]}),
        ),
        ("dsa_fundamentals/iteration_vs_recursion", json!({"n": 5})),
        (
            "dsa_fundamentals/memory_layout",
            json!({"values": [1, 2], "layout": "contiguous"}),
        ),
        (
            "dsa_fundamentals/merge_intervals",
            json!({"intervals": [[1, 3], [2, 4]]}),
        ),
        (
            "dsa_fundamentals/recursion_tree",
            json!({"root": "fib", "depth": 2, "branching_factor": 2}),
        ),
        (
            "dsa_fundamentals/sliding_window_detector",
            json!({"is_contiguous": true, "is_linear": true}),
        ),
        (
            "dsa_fundamentals/space_calculator",
            json!({"count": 10, "type_name": "u32"}),
        ),
        (
            "dsa_fundamentals/two_pointer_detector",
            json!({"is_sorted": true, "search_target": true}),
        ),
        (
            "arrays_strings/sparse_table",
            json!({"values": [4, 2, 7], "queries": [[0, 2], [1, 1]]}),
        ),
        (
            "arrays_strings/string_hashing",
            json!({"text": "banana", "ranges": [[1, 3]], "equals": [[1, 3, 3, 5]]}),
        ),
        (
            "arrays_strings/string_toolkit",
            json!({"words": ["flower", "flow", "flight"]}),
        ),
        (
            "arrays_strings/suffix_array_lite",
            json!({"text": "banana", "pattern": "ana"}),
        ),
        ("graphs/visualizer", json!({"adj": [[1], [2], []]})),
        (
            "graphs/word_ladder",
            json!({
                "begin": "hit",
                "end": "cog",
                "word_list": ["hot", "dot", "dog", "lot", "log", "cog"]
            }),
        ),
        ("dynamic_programming/climbing_stairs", json!({"n": 5})),
        (
            "dynamic_programming/fibonacci_viz",
            json!({"n": 7, "mode": "fast"}),
        ),
        (
            "backtracking/combinations",
            json!({"n": 4, "k": 2, "max_results": 3}),
        ),
        (
            "sorting_searching/binary_search_template",
            json!({"values": [1, 3, 5], "target": 3}),
        ),
        (
            "sorting_searching/counting_sort",
            json!({"values": [3, 1, 2]}),
        ),
        (
            "sorting_searching/matrix_search",
            json!({"matrix": [[1, 4], [2, 5]], "target": 5}),
        ),
        ("sorting_searching/merge_sort", json!({"values": [5, 1, 3]})),
        ("sorting_searching/peak_finder", json!({"nums": [1, 3, 2]})),
        ("sorting_searching/quick_sort", json!({"values": [5, 1, 3]})),
        (
            "greedy_algorithms/gas_station",
            json!({"gas": [1, 2, 3, 4, 5], "cost": [3, 4, 5, 1, 2]}),
        ),
        (
            "greedy_algorithms/task_scheduler",
            json!({"tasks": ["A", "A", "A", "B", "B", "B"], "cooldown": 2}),
        ),
    ];

    for (path, payload) in cases {
        post_tool(&client, &server, path, payload).await;
    }
}

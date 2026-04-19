use colored::*;
mod mcp_stdio;
mod web_server;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    // ── MCP stdio mode (zero-cost local agent integration) ─────────────────
    if args.contains(&"--mcp".to_string()) {
        mcp_stdio::run().await;
        return;
    }

    // ── Install mode (generate rules files for all 7 coding agents) ────────
    if args.contains(&"--install".to_string()) {
        let cwd = std::env::current_dir().expect("Cannot read current directory");
        println!("\n{}\n", "🔧 DSAEngine Install".bold().cyan());
        println!(
            "Generating rules files for 7 AI coding agents in: {}\n",
            cwd.display()
        );
        match dsaengine::utils::rules_generator::install(&cwd) {
            Ok(out) => {
                for f in &out.files_written {
                    println!("  {} {}", "✔".green(), f);
                }
                println!(
                    "\n{}\n\nPaste into your agent's MCP settings:\n\n{}",
                    "✅ Done! Add DSAEngine to your coding agent:"
                        .green()
                        .bold(),
                    out.mcp_config
                );
            }
            Err(e) => eprintln!("{} {e}", "❌ Install failed:".red().bold()),
        }
        return;
    }

    // ── Default: HTTP web server mode ───────────────────────────────────────
    println!(
        "\n{}\n",
        "🚀 dsaEngine Boot Sequence Initiated".bold().blue()
    );
    println!(
        "  Tip: Run with {} for local MCP stdio mode (zero-cost agent integration)",
        "--mcp".bold()
    );
    println!(
        "  Tip: Run with {} to generate agent rules files\n",
        "--install".bold()
    );

    println!("{} PAGE REGISTRY STATUS:", "⚙".yellow().bold());
    println!("--------------------------------------------------");

    let features = [
        (
            "fundamentals",
            "Page  1: Fundamentals       ",
            cfg!(feature = "fundamentals"),
        ),
        (
            "arrays",
            "Page  2: Arrays & Strings   ",
            cfg!(feature = "arrays"),
        ),
        (
            "linked_lists",
            "Page  3: Linked Lists       ",
            cfg!(feature = "linked_lists"),
        ),
        (
            "stacks",
            "Page  4: Stacks & Queues    ",
            cfg!(feature = "stacks"),
        ),
        (
            "trees",
            "Page  5: Binary Trees       ",
            cfg!(feature = "trees"),
        ),
        (
            "trees_adv",
            "Page  6: Advanced Trees     ",
            cfg!(feature = "trees_adv"),
        ),
        (
            "graphs",
            "Page  7: Graphs             ",
            cfg!(feature = "graphs"),
        ),
        ("dp", "Page  8: Dynamic Programming", cfg!(feature = "dp")),
        (
            "greedy",
            "Page  9: Greedy Algorithms  ",
            cfg!(feature = "greedy"),
        ),
        (
            "backtracking",
            "Page 10: Backtracking       ",
            cfg!(feature = "backtracking"),
        ),
        (
            "sorting",
            "Page 11: Sorting & Searching",
            cfg!(feature = "sorting"),
        ),
        (
            "advanced",
            "Page 12: Advanced Topics    ",
            cfg!(feature = "advanced"),
        ),
    ];

    let mut loaded_count = 0;

    for (flag, label, is_active) in features.iter() {
        if *is_active {
            println!("  ✔ {} [{}]", label.green(), flag.bold());
            loaded_count += 1;
        } else {
            println!("  ✖ {} [{}]", label.dimmed(), flag.dimmed());
        }
    }

    println!("--------------------------------------------------");

    if loaded_count == 0 {
        println!(
            "{}",
            "WARNING: Engine is in Ultra-Low Token mode. 0 pages loaded."
                .red()
                .bold()
        );
        println!(
            "Add features to Cargo.toml to bind skills (e.g., features = [\"graphs\", \"dp\"])."
        );
    } else if loaded_count == 12 {
        println!(
            "{}",
            "SUCCESS: Engine is running in FULL ADMIN mode. All pages loaded."
                .magenta()
                .bold()
        );
    } else {
        println!(
            "{}",
            format!(
                "SUCCESS: Engine running securely with {} active page(s).",
                loaded_count
            )
            .cyan()
            .bold()
        );
    }

    println!("\n{}\n", "Engine ready for deep logic operations.".blue());

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    println!(" DSAengine live on http://{}", addr);

    web_server::run_server().await;
}

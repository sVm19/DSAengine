use colored::*;
mod web_server;

#[tokio::main]
async fn main() {
    println!("\n{}\n", "🚀 dsaEngine Boot Sequence Initiated".bold().blue());
    println!("{} Loading memory-safe iterative algorithm limits...\n", "▶".green());

    println!("{} PAGE REGISTRY STATUS:", "⚙".yellow().bold());
    println!("--------------------------------------------------");

    let features = [
        ("fundamentals", "Page  1: Fundamentals       ", cfg!(feature = "fundamentals")),
        ("arrays",       "Page  2: Arrays & Strings   ", cfg!(feature = "arrays")),
        ("linked_lists", "Page  3: Linked Lists       ", cfg!(feature = "linked_lists")),
        ("stacks",       "Page  4: Stacks & Queues    ", cfg!(feature = "stacks")),
        ("trees",        "Page  5: Binary Trees       ", cfg!(feature = "trees")),
        ("trees_adv",    "Page  6: Advanced Trees     ", cfg!(feature = "trees_adv")),
        ("graphs",       "Page  7: Graphs             ", cfg!(feature = "graphs")),
        ("dp",           "Page  8: Dynamic Programming", cfg!(feature = "dp")),
        ("greedy",       "Page  9: Greedy Algorithms  ", cfg!(feature = "greedy")),
        ("backtracking", "Page 10: Backtracking       ", cfg!(feature = "backtracking")),
        ("sorting",      "Page 11: Sorting & Searching", cfg!(feature = "sorting")),
        ("advanced",     "Page 12: Advanced Topics    ", cfg!(feature = "advanced")),
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
        println!("{}", "WARNING: Engine is in Ultra-Low Token mode. 0 pages loaded.".red().bold());
        println!("Add features to Cargo.toml to bind skills (e.g., features = [\"graphs\", \"dp\"]).");
    } else if loaded_count == 12 {
        println!("{}", "SUCCESS: Engine is running in FULL ADMIN mode. All pages loaded.".magenta().bold());
    } else {
        println!("{}", format!("SUCCESS: Engine running securely with {} active page(s).", loaded_count).cyan().bold());
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

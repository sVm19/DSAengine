use clap::Parser;
use colored::*;

mod mcp_stdio;
mod web_server;

enum Mode {
    MCP,
    HTTP,
    Setup,
    Help,
}

#[derive(Parser, Debug)]
#[command(disable_help_flag = true, disable_version_flag = true)]
struct Cli {
    #[arg(long)]
    mcp: bool,

    #[arg(long)]
    http: bool,

    #[arg(long)]
    install: bool,

    #[arg(short = 'h', long)]
    help: bool,

    positional: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = match Cli::try_parse() {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Error: Invalid arguments");
            std::process::exit(1);
        }
    };

    let is_setup_positional = cli.positional.as_deref() == Some("setup-mcp");

    // Guard: Prevent multiple modes
    let mode_score = cli.mcp as u8 + cli.http as u8 + cli.install as u8 + is_setup_positional as u8;

    if mode_score > 1 {
        eprintln!("Error: Multiple modes detected. Please choose exactly one.");
        std::process::exit(1);
    }

    let mode = if cli.mcp {
        Mode::MCP
    } else if cli.http {
        Mode::HTTP
    } else if cli.install || is_setup_positional {
        Mode::Setup
    } else {
        Mode::Help
    };

    match mode {
        Mode::MCP => {
            mcp_stdio::run().await;
        }
        Mode::HTTP => {
            run_http_server().await;
        }
        Mode::Setup => {
            run_setup();
        }
        Mode::Help => {
            print_help();
        }
    }
}

async fn run_http_server() {
    println!("\n{}\n", " DSAEngine Boot Sequence Initiated".bold().blue());

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

    web_server::run_server().await;
}

fn run_setup() {
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
            println!("\nMCP setup completed successfully.");
        }
        Err(e) => {
            eprintln!("{} {e}", "❌ Install failed:".red().bold());
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("Usage:");
    println!("  dsaengine --mcp        Run in MCP mode (recommended)");
    println!("  dsaengine --http       Run HTTP server");
    println!("  dsaengine setup-mcp    Setup MCP integration");
}

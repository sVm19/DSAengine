#[cfg(feature = "fundamentals")]
use dsaengine::skills::dsa_fundamentals::BigOAnalyzer;

#[cfg(feature = "fundamentals")]
fn main() {
    let (_, report) = BigOAnalyzer::run_analysis(|| (1..=1000).sum::<i32>());
    println!(
        "Repository analysis sample completed in {:?}.",
        report.execution_time
    );
}

#[cfg(not(feature = "fundamentals"))]
fn main() {
    println!("Enable the `fundamentals` feature to run this example.");
}

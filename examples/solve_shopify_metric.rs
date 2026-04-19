#[cfg(feature = "arrays")]
use dsaengine::skills::arrays_strings::KadanesAlgorithm;

#[cfg(feature = "arrays")]
fn main() {
    let best = KadanesAlgorithm::solve_with_visual(&[4, -1, 2, 1, -5, 4]);
    println!("Sample metric run produced max subarray sum: {best}");
}

#[cfg(not(feature = "arrays"))]
fn main() {
    println!("Enable the `arrays` feature to run this example.");
}

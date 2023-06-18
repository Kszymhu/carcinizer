mod cli;
mod distance;

use cli::Args;
use clap::Parser;

use distance::get_string_distance;
fn main() {
    let args = Args::parse();

    let target: &str = &args.input;
    let pattern: &str = "crab";

    println!("Distance: {}", get_string_distance(pattern, target));
}

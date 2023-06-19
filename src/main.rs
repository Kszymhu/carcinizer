mod cli;
mod distance;
mod mutations;

use cli::Args;
use clap::Parser;

use distance::get_string_distance;
use mutations::{substitute_char, choose_substitution};

fn main() {
    let args = Args::parse();

    let text: &str = &args.input;
    let pattern: &str = "crab";

    let string_distance_result: (i32, usize) = get_string_distance(pattern, text);

    let distance: i32 = string_distance_result.0;
    let best_match_start: usize = string_distance_result.1;
    let best_match_end: usize = best_match_start + pattern.len();

    let best_match: &str = &text[best_match_start ..= best_match_end];

    println!("Best match: {0}. Distance: {1}.", best_match, distance);

    let text2 = substitute_char(2, text, 'A');
    println!("Original text: {text}");
    println!("Substituted text: {text2}");

    println!("Substitution for E: {}", choose_substitution('E'));

}

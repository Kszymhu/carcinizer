mod cli;

use cli::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("input: {}", args.input);
    println!("output file: {:?}", args.filename);
    println!("thread count: {}", args.threads);
    println!("generation size: {}", args.gen_size);
}

use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(name = "Carcinizer")]
#[command(about = "Uses an evolutionary algorithm to change given string into \"crab\".")]
#[command(long_about = "Treats given text (which has to consist only of ASCII characters, at the moment) similar to a nucleotide sequence, and performs \
substitution mutations to bring the text closer to the word \"crab\".")]
pub struct Args {
    #[arg(short = 'i')]
    #[arg(long = "input")]
    pub input: String,

    #[arg(short = 'o')]
    #[arg(long = "output_file")]
    pub filename: PathBuf,

    #[arg(short = 't')]
    #[arg(long = "threads")]
    #[arg(required = false)]
    #[arg(default_value_t = 1)]
    pub threads: usize,

    #[arg(short = 'g')]
    #[arg(long = "gen_size")]
    #[arg(required = false)]
    #[arg(default_value_t = 100)]
    pub gen_size: usize
}
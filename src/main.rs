pub mod rbuster;

use clap::Parser;
use crate::rbuster::{Rbuster};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    wordlist: String,

    #[arg(short, long, default_value = "10")]
    rps: usize,

    #[arg(long)]
    recursive: bool,

    #[arg(short, long, default_value = "3")]
    depth: usize,

    #[arg(short, long)]
    endings: Vec<String>,

    #[arg(short, long, default_value = "5")]
    timeout: u64,
}

const BANNER: &str = r#"
Rbuster v0.1.0
By: @0xleft

A simple rust based directory bruteforcer
"#;

fn main() {
    let args = Args::parse();
    println!("{}", BANNER);

    if args.verbose {
        println!("Arguments: {:#?}", args);
    }

    let rbuster = Rbuster::new(args.url, args.verbose, args.wordlist, args.rps, args.recursive, args.depth, args.endings, args.timeout);
    tokio::runtime::Runtime::new().unwrap().block_on(rbuster.run());
}
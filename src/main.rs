pub mod rbuster;

use clap::Parser;
use crate::rbuster::Rbuster;

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
    threads: usize,

    #[arg(long)]
    recursive: bool,

    #[arg(short, long, default_value = "3")]
    depth: usize,

    #[arg(short, long)]
    endings: String,

    #[arg(long, default_value = "10")]
    timeout: u64,
}

const BANNER: &str = r#"
Rbuster v0.1.0
By: @0xleft

A simple rust based directory bruteforcer
"#;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("{}", BANNER);

    if args.verbose {
        println!("Arguments: {:#?}", args);
    }

    let rbuster = Rbuster::new(args.url, args.wordlist, args.threads, args.endings, args.timeout);
    let _ = rbuster.run().await;
}
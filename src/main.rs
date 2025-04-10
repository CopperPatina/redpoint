use climbing::library::io::{log_index};
use climbing::library::summary::{print_summary};
use climbing::library::sync::{sync};

use clap::Parser;
use tokio;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    index: bool,
    #[arg(short, long)]
    climb: Option<String>,
    #[arg(short, long)]
    workout: Option<String>,
    #[arg(long)]
    summary: bool, 
    #[arg(long)]
    sync: bool, 
}
fn main() {

    let cli = Cli::parse();

    if cli.index {
        match log_index() {
            Ok(paths) => {
                for path in paths {
                    if let Some(filename) = path.file_name().and_then(|f| f.to_str()){
                        println!("{}", filename);
                    }
                }
            }
            Err(e) => println!("error {e} getting paths"),
        }
    }

    if cli.summary {
        print_summary();
    }

    if cli.sync {
        match tokio::runtime::Runtime::new() {
            Ok(rt) => rt.block_on(sync()),
            Err(e) => eprintln!("Failed to start async runtime: {}", e),
        }
    }
}

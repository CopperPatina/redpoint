use climbing::climblib::io::{print_log_index};
use climbing::climblib::summary::{print_summary};
use climbing::climblib::sync::{aws_entrypoint, AwsActions};

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
    #[arg(long)]
    pull: bool,
    #[arg(long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let bucket = "my-climblog-bucket".to_string();
    let cli = Cli::parse();

    if cli.index {
        print_log_index();
    } else if cli.summary {
        print_summary();
    } else if cli.sync {
        aws_entrypoint(AwsActions::Sync, &bucket, cli.dry_run).await;
    } else if cli.pull {
        aws_entrypoint(AwsActions::Pull, &bucket, cli.dry_run).await;
    }
}

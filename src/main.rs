use redpoint::climblib::io::{print_log_index};
use redpoint::climblib::summary::{print_summary};
use redpoint::climblib::sync::{aws_entrypoint, AwsActions};
use redpoint::api::server::{start_server};

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
    let cli = Cli::parse();
    let bucket = std::env::var("BUCKET_NAME").expect("BUCKET_NAME must be set");
    let db_connection_str = std::env::var("DATABASE_URL")
    .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    start_server(&db_connection_str).await;

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

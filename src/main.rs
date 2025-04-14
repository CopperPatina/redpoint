use climbing::climblib::io::{log_index};
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

    let bucket = "my-climblog-bucket".to_string();

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
        aws_entrypoint(AwsActions::Sync, &bucket, cli.dry_run).await;
    }

    if cli.pull {
        aws_entrypoint(AwsActions::Pull, &bucket, cli.dry_run).await;
    }
}

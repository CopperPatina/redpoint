use climbing::library::models::{ClimbingSession, WorkoutSession, ClimbEntry, ClimbMetricsEntry, ExerciseEntry, ClimbStyle};
use climbing::library::io::{save_log, load_log, log_index};

use std::fs;
use std::io::Result;
use clap::Parser;

fn session_to_json(session: &ClimbingSession, file: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(&session)?;
    fs::write(file, json)?;
    Ok(())
}

fn json_to_session(file: &str) -> Result<ClimbingSession> {
    let contents = fs::read_to_string(file)?;
    let session: ClimbingSession = serde_json::from_str(&contents)?;
    Ok(session)
}

fn print_sent_climbs(session: &ClimbingSession){
    for climb in session.climbs.iter().filter(|c| c.sent) {
        println!(
            "- Grade: {}, Attempts: {}, Rests: {:?}",
            climb.grade, climb.attempts, climb.rests.unwrap_or(0)
        );
    }
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    index: bool,
    #[arg(short, long)]
    climb: Option<String>,
    #[arg(short, long)]
    workout: Option<String>,
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

    let mut climb: Vec<ClimbEntry> = Vec::<ClimbEntry>::new();
    let route = ClimbEntry {
        name: None,
        grade: "5.11a".to_string(),
        attempts: 1,
        sent: true,
        reached_top: true,
        lead: true,
        rests: Some(2),
    };
    climb.push(route);

    let route = ClimbEntry {
        name: None,
        grade: "5.9".to_string(),
        attempts: 1,
        sent: true,
        reached_top: true,
        lead: true,
        rests: Some(0),
    };
    climb.push(route);

    let route = ClimbEntry {
        name: None,
        grade: "5.12a".to_string(),
        attempts: 1,
        sent: false,
        reached_top: false,
        lead: true,
        rests: Some(5),
    };
    climb.push(route);

    let sesh = ClimbingSession {
        date: "04-06-2025".to_string(),
        location: "Movement".to_string(),
        style: ClimbStyle::Rope,
        notes: Some("pumped!!!".to_string()),
        climbs: climb,

    };

    print_sent_climbs(&sesh);

    println!("{:?}", sesh);
    match session_to_json(&sesh, "session.json") {
        Ok(_) => println!("Session written to file."),
        Err(e) => eprintln!("Failed to write session: {}", e),
    }

    match json_to_session("session.json") {
        Ok(session) => {
            println!("Read from file:");
            println!("{:#?}", session);
        }
        Err(e) => eprintln!("Failed to read session: {}", e),
    }
}

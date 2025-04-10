use super::io::{load_log, log_index};
use super::models::{ClimbingSession, WorkoutSession, ClimbMetricsEntry};

pub fn print_summary() {
    let mut num_climbs = 0;
    let mut num_workouts = 0;
    let mut num_metrics = 0;
    match log_index() {
        Ok(paths) => {
            for path in paths {
                if let Some(filename) = path.file_name().and_then(|f| f.to_str()){
                    let full_path = format!("logs/{}", filename);
                    if filename.contains("climb") {
                        match load_log::<ClimbingSession>(&full_path){
                            Ok(c) => {
                                num_climbs += c.climbs.len();
                            }
                            Err(e) => eprintln!("error {e} loading file {filename}"),
                        }
                    }
                    else if filename.contains("workout") {
                        match load_log::<WorkoutSession>(&full_path) {
                        Ok(_w) => {
                            num_workouts += 1;
                        }
                        Err(e) => eprintln!("error {e} loading file {filename}"),
                        }
                    }
                    else if filename.contains("metrics") {
                        match load_log::<ClimbMetricsEntry>(&full_path) {
                        Ok(_m) => {
                            num_metrics += 1;
                        }
                        Err(e) => eprintln!("error {e} loading file {filename}"),
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("error {e} getting paths"),
    }
    println!("Number of climbs: {num_climbs}");
    println!("Number of workouts: {num_workouts}");
    println!("Number of metrics: {num_metrics}");
}

pub fn print_sent_climbs(session: &ClimbingSession){
    for climb in session.climbs.iter().filter(|c| c.sent) {
        println!(
            "- Grade: {}, Attempts: {}, Rests: {:?}",
            climb.grade, climb.attempts, climb.rests.unwrap_or(0)
        );
    }
}
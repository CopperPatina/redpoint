use super::io::{load_log, log_index};
use super::models::{ClimbingSession, WorkoutSession};

pub fn print_summary() {
    let mut num_climbs = 0;
    let mut num_workouts = 0;
    match log_index() {
        Ok(paths) => {
            for path in paths {
                if let Some(filename) = path.file_name().and_then(|f| f.to_str()){
                    let full_path = format!("logs/{}", filename);
                    if filename.contains("climbing") {
                        match load_log::<ClimbingSession>(&full_path){
                            Ok(c) => {
                                num_climbs += c.climbs.len();
                            }
                            Err(e) => eprintln!("error {e} loading file {filename}"),
                        }
                    }
                    if filename.contains("workout") {
                        match load_log::<WorkoutSession>(&full_path) {
                        Ok(w) => {
                            num_workouts += 1;
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
}
use super::io::{load_log, log_index};
use super::models::{ClimbingSession, WorkoutSession, ClimbMetricsEntry};
use super::utils::{is_climb, is_workout, is_metrics};
use tracing::{info, error};

pub fn print_summary() {
    let mut num_climbs = 0;
    let mut num_workouts = 0;
    let mut num_metrics = 0;
    match log_index() {
        Ok(paths) => {
            for path in paths {
                if is_climb(&path) {
                    match load_log::<ClimbingSession>(&path){
                        Ok(c) => {
                            num_climbs += c.climbs.len();
                        }
                        Err(e) => error!("error {e} loading file {:?}", path),
                    }
                }
                else if is_workout(&path) {
                    match load_log::<WorkoutSession>(&path) {
                    Ok(_w) => {
                        num_workouts += 1;
                    }
                    Err(e) => error!("error {e} loading file {:?}", path),
                    }
                }
                else if is_metrics(&path) {
                    match load_log::<ClimbMetricsEntry>(&path) {
                    Ok(_m) => {
                        num_metrics += 1;
                    }
                    Err(e) => error!("error {e} loading file {:?}", path),
                    }
                }
            }
        }
        Err(e) => error!("error {e} getting paths"),
    }
    info!("Number of climbs: {num_climbs}");
    info!("Number of workouts: {num_workouts}");
    info!("Number of metrics: {num_metrics}");
}

pub fn print_sent_climbs(session: &ClimbingSession){
    for climb in session.climbs.iter().filter(|c| c.sent) {
        info!(
            "- Grade: {:?}, Attempts: {}, Rests: {:?}",
            climb.grade, climb.attempts, climb.rests.unwrap_or(0)
        );
    }
}
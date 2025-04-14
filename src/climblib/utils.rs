use std::path::PathBuf;

pub fn is_climb(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|f| f.to_str())
        .map_or(false, |name| name.contains("climb"))
}

pub fn is_workout(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|f| f.to_str())
        .map_or(false, |name| name.contains("workout"))
}

pub fn is_metrics(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|f| f.to_str())
        .map_or(false, |name| name.contains("metrics"))
}

pub fn infer_log_type(path: &PathBuf) -> Option<&'static str> {
    if is_climb(path) {
        Some("climb")
    } else if is_workout(path) {
        Some("workout")
    } else if is_metrics(path) {
        Some("metrics")
    } else {
        None
    }
}
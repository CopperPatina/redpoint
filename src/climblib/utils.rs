use std::path::Path;

pub fn is_climb(path: &Path) -> bool {
    path.file_name()
        .and_then(|f| f.to_str())
        .map_or(false, |name| name.contains("climb"))
}

pub fn is_workout(path: &Path) -> bool {
    path.file_name()
        .and_then(|f| f.to_str())
        .map_or(false, |name| name.contains("workout"))
}

pub fn is_metrics(path: &Path) -> bool {
    path.file_name()
        .and_then(|f| f.to_str())
        .map_or(false, |name| name.contains("metrics"))
}

pub fn infer_log_type(path: &Path) -> Option<&'static str> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn path(filename: &str) -> PathBuf {
        PathBuf::from(format!("logs/{}", filename))
    }

    #[test]
    fn test_is_climb() {
        assert!(is_climb(&path("2024-04-01_climb.json")));
        assert!(!is_climb(&path("2024-04-01_workout.json")));
    }

    #[test]
    fn test_is_workout() {
        assert!(is_workout(&path("2024-04-01_workout.json")));
        assert!(!is_workout(&path("2024-04-01_climb.json")));
    }

    #[test]
    fn test_is_metrics() {
        assert!(is_metrics(&path("2024-04-01_metrics.json")));
        assert!(!is_metrics(&path("2024-04-01_climb.json")));
    }

    #[test]
    fn test_infer_log_type() {
        assert_eq!(infer_log_type(&path("2024-04-01_climb.json")), Some("climb"));
        assert_eq!(infer_log_type(&path("2024-04-01_workout.json")), Some("workout"));
        assert_eq!(infer_log_type(&path("2024-04-01_metrics.json")), Some("metrics"));
        assert_eq!(infer_log_type(&path("2024-04-01_unknown.json")), None);
    }
}
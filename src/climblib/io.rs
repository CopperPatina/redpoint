use std::fs::{self, File};
use std::io::{self, Write, Result};
use std::path::{PathBuf, Path};
use serde_json::{to_string_pretty, from_str};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{info, error, warn};

pub fn save_log<T: Serialize>(data: &T, filename: &str) -> Result<()> {
    let mut path = PathBuf::from("logs");
    path.push(filename);
    let json = to_string_pretty(&data)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let mut file = File::create(&path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_log<T: DeserializeOwned>(path: &Path) -> io::Result<T> {
    let json = fs::read_to_string(path)?;
    let session = from_str(&json)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(session)
}

pub fn log_index() -> Result<Vec<PathBuf>> {
    let log_dir = "logs";

    // Ensure the directory exists
    if !std::path::Path::new(log_dir).exists() {
        warn!("Log directory not found: {log_dir}");
        return Ok(vec![]); // Return an empty list instead of panicking
    }

    let entries = fs::read_dir(log_dir)?;
    let mut files = Vec::new();

    for entry in entries {
        let path = entry?.path();
        let filename = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
    
        if filename.contains("workout") || filename.contains("climb") || filename.contains("metrics") {
            files.push(path);
        }
    }
    
    Ok(files)
}

pub fn print_log_index() {
    match log_index() {
        Ok(paths) => {
            for path in paths {
                if let Some(filename) = path.file_name().and_then(|f| f.to_str()){
                    info!("{}", filename);
                }
            }
        }
        Err(e) => error!("error {e} getting paths"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::path::PathBuf;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct DummyWorkout {
        name: String,
        sets: u8,
        reps: u8,
    }
    
    fn test_save_and_load_log_roundtrip() {
        let temp = tempdir().unwrap();
        let filename = "test_workout.json";
        let mut path = PathBuf::from(temp.path());
        path.push(filename);

        let workout = DummyWorkout {
            name: "Deadlift".into(),
            sets: 3,
            reps: 5,
        };

        // Write log
        save_log(&workout, path.to_str().unwrap()).expect("Failed to save");

        // Read log back
        let mut loaded: DummyWorkout = DummyWorkout {name: "".to_string(),sets: 0,reps: 0};
        match load_log(&path) {
            Ok(workout) => loaded = workout,
            Err(e) => error!("Failed to load {}", e)
        };

        assert_eq!(workout, loaded);
    }
}
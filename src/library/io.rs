use std::fs::{self, File};
use std::io::{Write, Result};
use std::path::{PathBuf};
use serde_json::{to_string_pretty, from_str};
use serde::{Serialize, de::DeserializeOwned};

pub fn save_log<T: Serialize>(data: &T, filename: &str) -> Result<()> {
    let path = format!("../logs/{}", filename);
    let json = to_string_pretty(&data).unwrap();
    let mut file = File::create(&path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_log<T: DeserializeOwned>(path: &str) -> Result<T> {
    let json = fs::read_to_string(path)?;
    let session: T = from_str(&json)?;
    
    Ok(session)
}

pub fn log_index() -> Result<Vec<PathBuf>> {
    let log_dir = "logs";

    // Ensure the directory exists
    if !std::path::Path::new(log_dir).exists() {
        println!("⚠️ Log directory not found: {log_dir}");
        return Ok(vec![]); // Return an empty list instead of panicking
    }

    let entries = fs::read_dir(log_dir)?;
    let mut files = Vec::new();

    for entry in entries {
        let path = entry?.path();
        let filename = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
    
        if filename.contains("workout") || filename.contains("climb") {
            files.push(path);
        }
    }
    
    Ok(files)
}
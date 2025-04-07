use serde;
use std::fs;
use std::io::Result;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum ClimbStyle {
    Boulder,
    Rope,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ClimbEntry {
    name: Option<String>,
    grade: String,
    attempts: u8,
    sent: bool,
    reached_top: bool,
    lead: bool,
    rests: Option<u8>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ClimbMetrics {
    finger_strength_percent_bw: Option<f32>,
    max_pullup_percent_bw: Option<f32>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ClimbMetricsEntry {
    date: String,
    finger_strength_percent_bw: Option<f32>,
    max_pullup_percent_bw: Option<f32>,
    notes: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ClimbingSession {
    date: String,
    location: String,
    style: ClimbStyle,
    notes: Option<String>,
    climbs: Vec<ClimbEntry>,
}

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

fn main() {

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

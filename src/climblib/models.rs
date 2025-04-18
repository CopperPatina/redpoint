use serde;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClimbStyle {
    Boulder,
    Rope,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClimbEntry {
    pub name: Option<String>,
    pub grade: String,
    pub attempts: u8,
    pub sent: bool,
    pub reached_top: bool,
    pub lead: bool,
    pub rests: Option<u8>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClimbMetricsEntry {
    pub date: String,
    pub finger_strength_percent_bw: Option<f32>,
    pub max_pullup_percent_bw: Option<f32>,
    pub notes: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename = "camelCaseName")]
pub struct ClimbingSession {
    pub date: String,
    pub location: String,
    pub style: ClimbStyle,
    pub notes: Option<String>,
    pub climbs: Vec<ClimbEntry>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseEntry {
    pub name: String,
    pub sets: u8,
    pub reps: u8,
    pub weight_lb: i32,
    pub rpe: Option<u8>,
    pub is_main_lift: Option<bool>,
  }

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename = "camelCaseName")]
pub struct WorkoutSession {
    pub date: String,
    pub notes: Option<String>,
    pub exercises: Vec<ExerciseEntry>,
  }

pub enum LogEntry {
    Climbing(ClimbingSession),
    Workout(WorkoutSession),
    Metrics(ClimbMetricsEntry),
}

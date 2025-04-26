use serde::{Serialize, Deserialize};
use validator::{Validate};
use std::fmt;
use super::utils::{validate_date_format};


#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClimbStyle {
    Boulder,
    Rope,
}

impl fmt::Display for ClimbStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ClimbStyle::Boulder => "Boulder",
            ClimbStyle::Rope => "Rope",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RopeGrade {
    #[serde(rename = "5.intro")] FiveIntro,
    #[serde(rename = "5.6")] FiveSix,
    #[serde(rename = "5.7")] FiveSeven,
    #[serde(rename = "5.8")] FiveEight,
    #[serde(rename = "5.9")] FiveNine,
    #[serde(rename = "5.10a")] FiveTenA,
    #[serde(rename = "5.10b")] FiveTenB,
    #[serde(rename = "5.10c")] FiveTenC,
    #[serde(rename = "5.10d")] FiveTenD,
    #[serde(rename = "5.11a")] FiveElevenA,
    #[serde(rename = "5.11b")] FiveElevenB,
    #[serde(rename = "5.11c")] FiveElevenC,
    #[serde(rename = "5.11d")] FiveElevenD,
    #[serde(rename = "5.12a")] FiveTwelveA,
    #[serde(rename = "5.12b")] FiveTwelveB,
    #[serde(rename = "5.12c")] FiveTwelveC,
    #[serde(rename = "5.12d")] FiveTwelveD,
    #[serde(rename = "5.13a")] FiveThirteenA,
    #[serde(rename = "5.13b")] FiveThirteenB,
    #[serde(rename = "5.13c")] FiveThirteenC,
    #[serde(rename = "5.13d")] FiveThirteenD,
    #[serde(rename = "5.14a")] FiveFourteenA,
    #[serde(rename = "5.14b")] FiveFourteenB,
    #[serde(rename = "5.14c")] FiveFourteenC,
    #[serde(rename = "5.14d")] FiveFourteenD,
    #[serde(rename = "5.15a")] FiveFifteenA,
    #[serde(rename = "5.15b")] FiveFifteenB,
    #[serde(rename = "5.15c")] FiveFifteenC,
    #[serde(rename = "5.15d")] FiveFifteenD,
}

impl fmt::Display for RopeGrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RopeGrade::FiveIntro => "5.intro",
            RopeGrade::FiveSix => "5.6",
            RopeGrade::FiveSeven => "5.7",
            RopeGrade::FiveEight => "5.8",
            RopeGrade::FiveNine => "5.9",
            RopeGrade::FiveTenA => "5.10a",
            RopeGrade::FiveTenB => "5.10b",
            RopeGrade::FiveTenC => "5.10c",
            RopeGrade::FiveTenD => "5.10d",
            RopeGrade::FiveElevenA => "5.11a",
            RopeGrade::FiveElevenB => "5.11b",
            RopeGrade::FiveElevenC => "5.11c",
            RopeGrade::FiveElevenD => "5.11d",
            RopeGrade::FiveTwelveA => "5.12a",
            RopeGrade::FiveTwelveB => "5.12b",
            RopeGrade::FiveTwelveC => "5.12c",
            RopeGrade::FiveTwelveD => "5.12d",
            RopeGrade::FiveThirteenA => "5.13a",
            RopeGrade::FiveThirteenB => "5.13b",
            RopeGrade::FiveThirteenC => "5.13c",
            RopeGrade::FiveThirteenD => "5.13d",
            RopeGrade::FiveFourteenA => "5.14a",
            RopeGrade::FiveFourteenB => "5.14b",
            RopeGrade::FiveFourteenC => "5.14c",
            RopeGrade::FiveFourteenD => "5.14d",
            RopeGrade::FiveFifteenA => "5.15a",
            RopeGrade::FiveFifteenB => "5.15b",
            RopeGrade::FiveFifteenC => "5.15c",
            RopeGrade::FiveFifteenD => "5.15d",
        };
        write!(f, "{}", s)
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BoulderGrade {
    #[serde(rename = "vintro")] VIntro,
    #[serde(rename = "v0")] V0,
    #[serde(rename = "v1")] V1,
    #[serde(rename = "v2")] V2,
    #[serde(rename = "v3")] V3,
    #[serde(rename = "v4")] V4,
    #[serde(rename = "v5")] V5,
    #[serde(rename = "v6")] V6,
    #[serde(rename = "v7")] V7,
    #[serde(rename = "v8")] V8,
    #[serde(rename = "v9")] V9,
    #[serde(rename = "v10")] V10,
    #[serde(rename = "v11")] V11,
    #[serde(rename = "v12")] V12,
    #[serde(rename = "v13")] V13,
    #[serde(rename = "v14")] V14,
    #[serde(rename = "v15")] V15,
    #[serde(rename = "v16")] V16,
    #[serde(rename = "v17")] V17,
}

impl fmt::Display for BoulderGrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BoulderGrade::VIntro => "vintro",
            BoulderGrade::V0 => "v0",
            BoulderGrade::V1 => "v1",
            BoulderGrade::V2 => "v2",
            BoulderGrade::V3 => "v3",
            BoulderGrade::V4 => "v4",
            BoulderGrade::V5 => "v5",
            BoulderGrade::V6 => "v6",
            BoulderGrade::V7 => "v7",
            BoulderGrade::V8 => "v8",
            BoulderGrade::V9 => "v9",
            BoulderGrade::V10 => "v10",
            BoulderGrade::V11 => "v11",
            BoulderGrade::V12 => "v12",
            BoulderGrade::V13 => "v13",
            BoulderGrade::V14 => "v14",
            BoulderGrade::V15 => "v15",
            BoulderGrade::V16 => "v16",
            BoulderGrade::V17 => "v17",
        };
        write!(f, "{}", s)
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Grade {
    Rope(RopeGrade),
    Boulder(BoulderGrade),
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClimbEntry {
    #[validate(length(min = 0, max = 100))]
    pub name: Option<String>,
    pub grade: Grade,
    #[validate(range(min = 0, max = 100))]
    pub attempts: u8,
    pub sent: bool,
    pub reached_top: bool,
    pub lead: bool,
    #[validate(range(min = 0, max = 100))]
    pub rests: Option<u8>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClimbMetricsEntry {
    #[validate(length(min = 1), custom(function = "validate_date_format"))]
    pub date: String,
    #[validate(range(min = 100, max = 300))]
    pub finger_strength_percent_bw: Option<f32>,
    #[validate(range(min = 100, max = 300))]
    pub max_pullup_percent_bw: Option<f32>,
    #[validate(length(min = 0, max = 300))]
    pub notes: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Validate)]
#[serde(rename = "camelCaseName")]
pub struct ClimbingSession {
    #[validate(length(min = 1), custom(function = "validate_date_format"))]
    pub date: String,
    #[validate(length(min = 0, max = 100))]
    pub location: String,
    pub style: ClimbStyle,
    #[validate(length(min = 0, max = 300))]
    pub notes: Option<String>,
    pub climbs: Vec<ClimbEntry>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseEntry {
    #[validate(length(min = 0, max = 100))]
    pub name: String,
    #[validate(range(min = 0, max = 100))]
    pub sets: u8,
    #[validate(range(min = 0, max = 100))]
    pub reps: u8,
    #[validate(range(min = -100, max = 1000))]
    pub weight_lb: i32,
    #[validate(range(min = 0, max = 12))]
    pub rpe: Option<u8>,
    pub is_main_lift: Option<bool>,
  }

#[derive(Debug, serde::Serialize, serde::Deserialize, Validate)]
#[serde(rename = "camelCaseName")]
pub struct WorkoutSession {
    #[validate(length(min = 1), custom(function = "validate_date_format"))]
    pub date: String,
    #[validate(length(min = 0, max = 300))]
    pub notes: Option<String>,
    pub exercises: Vec<ExerciseEntry>,
  }

pub enum LogEntry {
    Climbing(ClimbingSession),
    Workout(WorkoutSession),
    Metrics(ClimbMetricsEntry),
}

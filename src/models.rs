use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local, Utc};

#[derive(Debug)]
pub struct File {
    pub filename: String,
}

impl File {
    pub fn new(filename: String) -> Self {
        Self {
            filename: filename
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Activity {
    pub sport: String,

    pub timestamp: DateTime<Local>,

    // in seconds
    pub duration: f64,

    // in meters
    pub distance: Option<f64>,

    // in kcal
    pub calories: f64,
}

impl Activity {
    pub fn new() -> Self {
        Self {
            sport: "Unknown".to_owned(),
            timestamp: Local::now(),
            duration: 0.0,
            distance: None,
            calories: 0.0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.sport == "Unknown"
    }
}

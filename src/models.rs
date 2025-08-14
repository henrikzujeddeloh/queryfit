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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    pub product: String,

    pub timestamp: DateTime<Local>,

    // in V
    pub battery: Option<f64>,

}

impl Device {
    pub fn new() -> Self {
        Self {
            product: "Unknown".to_owned(),
            timestamp: Local::now(),
            battery: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.product == "Unknown"
    }
}

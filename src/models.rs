use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct File {
    pub filename: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Activity {
    pub id: i64,
    pub sport_type: String,
    // in seconds
    pub duration: f64,
}

impl Activity {
    pub fn new() -> Self {
        Self {
            id: 0,
            sport_type: "Unknown".to_owned(),
            duration: 0.0,
        }
    }
}

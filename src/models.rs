use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct File {
    pub filename: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Activity {
    pub sport: String,
    // in seconds
    pub duration: f64,
}

impl Activity {
    pub fn new() -> Self {
        Self {
            sport: "Unknown".to_owned(),
            duration: 0.0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.sport == "Unknown"
    }
}

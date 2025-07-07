use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DataEntry {
    pub content: String,
    pub timestamp: String,
}

impl DataEntry {
    pub fn new(content: String) -> Self {
        Self {
            content,
            timestamp: chrono::Local::now().to_rfc3339(),
        }
    }
}

use chrono::local::DateTime;
use serde::{Deserialize, Serialize};

pub struct DataEntry {
	pub content: String,
	pub timestamp: String,
}

impl DataEntry {
	pub fn new(content: String) -> Self {
		let timestamp = DateTime::now().to_rfc3339();
		DataEntry { content, timestamp }
	}
}
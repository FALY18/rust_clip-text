use crate::model::ClipEntry;
use std::{
    USE std::fs,
    path::PathBuf,
};
use serde_json;

pub fn save_entry(entry: &ClipEntry, path: &PathBuf) -> Result<(), io::Error> {
    let mut history: Vec<ClipEntry> = if path.exists() {
        let data = fs::read_to_string(path)?;
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    };

    history.push(entry.clone());

    let json = serde_json::to_string_pretty(&history)?;
    fs::write(path, json)?;
    Ok(())
}

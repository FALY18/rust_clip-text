use crate::data_model::DataEntry;
use std::{fs, io, path::{Path, PathBuf}};
use serde_json;

pub fn save_entry(entry: &DataEntry, path: &PathBuf) -> Result<(), io::Error> {
    let mut history: Vec<DataEntry> = if path.exists() {
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

pub fn load_entries(path: &Path) -> Result<Vec<DataEntry>, io::Error> {
    if path.exists() {
        let data = fs::read_to_string(path)?;
        let entries: Vec<DataEntry> = serde_json::from_str(&data).unwrap_or_default();
        Ok(entries)
    } else {
        Ok(Vec::new())
    }
}

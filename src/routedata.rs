use std::{fs, path::PathBuf};
use dirs;

pub fn get_history_path() -> Result<PathBuf, std::io::Error> {
    let mut dir = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push("clipboard_time_machine");

    fs::create_dir_all(&dir)?;
    dir.push("history.json");
    Ok(dir)
}

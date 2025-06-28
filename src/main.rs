mod clipbord;
mod data_model;
mod savedata;
mod routedata;

use data_model::DataEntry;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let content = clipbord::gettxtcopy()?;
    let entry = DataEntry::new(content);
    let path = routedata::get_history_path()?;
    savedata::save_entry(&entry, &path)?;

    println!("Copie sauvegardée à {}", entry.timestamp);
    Ok(())
}

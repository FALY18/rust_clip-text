use clap::{Parser, Subcommand};
use std::error::Error;

mod clipbord;
mod data_model;
mod savedata;
mod routedata;

use savedata::{save_entry, load_entries};
use data_model::DataEntry;
use std::path::PathBuf;
	
/**
 * gestionnaire de copier-coller
 */
 
#[derive(Parser)]
#[command(name = "Clipboard History")]
#[command(version = "1.0")]
#[command(about = "Sauvegarde et affiche l’historique du presse-papiers")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /**
     * Sauvegarde la dernière copie du presse-papiers
     */
    Save,
    /**
     * Affiche tout l'historique
     */
    List,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let path: PathBuf = routedata::get_history_path()?;

    match &cli.command {
        Commands::Save => {
            let content = clipbord::gettxtcopy()?;
            let entry = DataEntry::new(content);
            save_entry(&entry, &path)?; 
            println!("Copie sauvegardée à {}", entry.timestamp);
        }
        Commands::List => {
            show_history(&path); 
        }
    }

    Ok(())
}

fn show_history(path: &std::path::Path) {
    match load_entries(path) { 
        Ok(entries) => {
            if entries.is_empty() {
                println!("Aucune entrée trouvée.");
            } else {
                for entry in entries {
                    println!("[{}]\n{}\n", entry.timestamp, entry.content);
                }
            }
        }
        Err(e) => eprintln!("Erreur de chargement de l'historique: {}", e),
    }
}

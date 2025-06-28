use arboard::Clipboard;
use std::error::Error;

pub fn gettxtcopy() -> Result<String, Box<dyn Error>> {
	let mut clipboard = Clipboard::new()?;
	let text = clipboard.get_text()?;
	Ok(text)
}
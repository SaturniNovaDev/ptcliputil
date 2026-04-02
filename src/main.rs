use arboard::Clipboard;
use std::env;
use std::fs;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut content = String::new();
    let args: Vec<String> = env::args().collect();

    // Check if data is being piped in (stdin is not a terminal)
    if !atty::is(atty::Stream::Stdin) {
        io::stdin().read_to_string(&mut content)?;
    }
    // Otherwise, check for a filename argument
    else if args.len() > 1 {
        content = fs::read_to_string(&args[1])?;
    } else {
        eprintln!("Usage: \n  list | ptcliputil\n  ptcliputil file.txt");
        std::process::exit(1);
    }

    // Initialize clipboard and copy
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(content.trim().to_string())?;

    println!("✓ Content copied to clipboard.");
    Ok(())
}

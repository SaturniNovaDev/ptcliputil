use arboard::Clipboard;
use clap::Parser;
use std::io::Read;
use std::{fs, io, process, thread, time::Duration};

/// CLI tool to copy text in the terminal, either passed from pipes or read from files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The file to read (optional if piping data)
    file: Option<String>,

    /// Show the copied text (forced false if multi-line)
    #[arg(short, long)]
    verbose: bool,

    /// Clear the clipboard after N seconds
    #[arg(short, long, value_name = "SECONDS")]
    clear: Option<u64>,
}

fn main() {
    let args: Args = Args::parse();

    // Check if program is piped
    let is_piped: bool = !io::IsTerminal::is_terminal(&io::stdin());

    if args.file.is_none() && !is_piped {
        // Use the Command interface from clap to print help manually
        use clap::CommandFactory;
        let mut cmd: clap::Command = Args::command();
        cmd.print_help().unwrap();
        println!();
        process::exit(0);
    }

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut content: String = String::new();

    // 1. Check from the check in main() wether the program is piped or reading from file
    if let Some(file_path) = &args.file {
        content = fs::read_to_string(file_path)?;
    } else {
        io::stdin().read_to_string(&mut content)?;
    }

    let content_trimmed: &str = content.trim();
    if content_trimmed.is_empty() {
        return Err("Nothing to copy (content is empty)".into());
    }

    // 2. Copy to clipboard
    let mut clipboard: Clipboard = Clipboard::new()?;
    clipboard.set_text(content_trimmed.to_string())?;

    // 3. Verbose output
    let line_count: usize = content_trimmed.lines().count();
    let character_count: usize = content_trimmed.chars().count();
    let should_show_content: bool = args.verbose && line_count <= 1;

    if should_show_content {
        println!("✓ Copied: \"{}\"", content_trimmed);
    } else {
        println!(
            "✓ Content copied to clipboard ({} lines, {} characters).",
            line_count, character_count
        );
    }

    // 4. Delayed Clear Logic
    if let Some(seconds_until_clear) = args.clear {
        println!(
            "INFO: The `--clear` flag was detected. Clipboard will be cleared in {} seconds.",
            seconds_until_clear
        );

        // Create a separate thread to clear the clipboard after N seconds.
        thread::sleep(Duration::from_secs(seconds_until_clear));

        let mut clipboard: Clipboard = Clipboard::new()?;
        clipboard.set_text("".to_string())?;
        println!("Clipboard cleared.");
    }

    Ok(())
}

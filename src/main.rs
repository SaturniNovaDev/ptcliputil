use arboard::Clipboard;
use clap::Parser;
use std::io::{self, IsTerminal, Read, Write};
use std::path::Path;
use std::{fs, process, thread, time::Duration};

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

    /// Save current clipboard content to a file
    #[arg(short, long, value_name = "OUT_FILE")]
    save: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    if let Some(ref path) = args.save {
        if let Err(e) = save_clipboard_to_file(path) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
        process::exit(0);
    }

    let is_piped: bool = !io::stdin().is_terminal();
    if args.file.is_none() && !is_piped {
        use clap::CommandFactory;
        Args::command().print_help().unwrap();
        println!();
        process::exit(0);
    }

    if let Err(e) = run_copy(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run_copy(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut content: String = String::new();

    if let Some(file_path) = &args.file {
        content = fs::read_to_string(file_path)?;
    } else {
        io::stdin().read_to_string(&mut content)?;
    }

    let content_trimmed: &str = content.trim();
    if content_trimmed.is_empty() {
        return Err("Nothing to copy (content is empty)".into());
    }

    let mut clipboard: Clipboard = Clipboard::new()?;
    clipboard.set_text(content_trimmed.to_string())?;

    let line_count: usize = content_trimmed.lines().count();
    let char_count: usize = content_trimmed.chars().count();

    if args.verbose && line_count <= 1 {
        println!("✓ Copied: \"{}\"", content_trimmed);
    } else {
        println!(
            "✓ Content copied to clipboard ({} lines, {} chars).",
            line_count, char_count
        );
    }

    // Delayed Clear Logic
    if let Some(seconds_until_clear) = args.clear {
        println!(
            "INFO: The `--clear` flag was detected. Clipboard will be cleared in {} seconds.",
            seconds_until_clear
        );

        // Clear the clipboard after N seconds.
        thread::sleep(Duration::from_secs(seconds_until_clear));

        let mut clipboard: Clipboard = Clipboard::new()?;
        clipboard.set_text("".to_string())?;
        println!("Clipboard cleared.");
    }

    Ok(())
}

fn save_clipboard_to_file(path_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path: &Path = Path::new(path_str);

    if path.exists() {
        print!("File '{}' already exists. Overwrite? (y/N): ", path_str);
        io::stdout().flush()?;

        let mut input: String = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() != "y" {
            println!("Aborted.");
            return Ok(());
        }
    }

    let mut clipboard: Clipboard = Clipboard::new()?;
    let content: String = clipboard.get_text()?;

    fs::write(path, content)?;
    println!("✓ Clipboard saved to '{}'", path_str);
    Ok(())
}

//! CLI entry point: read Markdown from a file or stdin, emit HTML to stdout.
use clap::Parser;
use std::io::Read;

#[derive(Parser)]
#[command(name = "markdown-to-html", about = "Convert Markdown to HTML")]
struct Cli {
    /// Input file (reads stdin if omitted)
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let input = match cli.file {
        Some(path) => std::fs::read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("Error reading {}: {}", path, e);
            std::process::exit(1);
        }),
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf).unwrap();
            buf
        }
    };
    print!("{}", markdown_to_html::convert(&input));
}

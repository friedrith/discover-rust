use anyhow::Result;
use clap::Parser;

mod file_scanner;
mod models;
mod printer;
mod readme_parser;

use file_scanner::list_files_recursive;
use printer::print_features;

/// Recursively list all files in a directory.
#[derive(Parser)]
struct Cli {
    /// The path to the directory to list
    path: std::path::PathBuf,

    /// Output features as JSON
    #[arg(long)]
    json: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let features = list_files_recursive(&args.path)?;

    if args.json {
        let json = serde_json::to_string_pretty(&features)?;
        println!("{}", json);
    } else {
        println!("Features found in {}:", args.path.display());
        if features.is_empty() {
            println!("No features found.");
        } else {
            print_features(&features, 0);
        }
    }

    Ok(())
}

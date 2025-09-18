use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::Path;

/// Recursively list all files in a directory.
#[derive(Parser)]
struct Cli {
    /// The path to the directory to list
    path: std::path::PathBuf,
}

struct Feature {
    name: String,
    description: String,
    path: String,
}

fn list_files_recursive(dir: &Path) -> Result<Vec<Feature>> {
    let entries = fs::read_dir(dir)
        .with_context(|| format!("could not read directory `{}`", dir.display()))?;

    let mut features: Vec<Feature> = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();

        if path.is_dir() {
            if dir.ends_with("features") {
                features.push(Feature {
                    name: name.to_string(),
                    description: "Source code".to_string(),
                    path: path.to_string_lossy().to_string(),
                });
            }

            let new_features = list_files_recursive(&path);
            features.extend(new_features?);
        }
    }

    return Ok(features);
}

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("{}/", args.path.display());
    let features = list_files_recursive(&args.path)?;

    println!("Features:");
    for feature in features {
        println!(
            "{} - {} -> {}",
            feature.name, feature.path, feature.description
        );
    }

    Ok(())
}

use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Recursively list all files in a directory.
#[derive(Parser)]
struct Cli {
    /// The path to the directory to list
    path: std::path::PathBuf,
}

#[derive(Debug, Deserialize)]
struct FrontMatter {
    owner: Option<String>,
}

struct Feature {
    name: String,
    description: String,
    owner: String,
    path: String,
}

fn read_readme_info(readme_path: &Path) -> Result<(String, String)> {
    if !readme_path.exists() {
        return Ok((
            "Unknown".to_string(),
            "No description available".to_string(),
        ));
    }

    let content = fs::read_to_string(readme_path)
        .with_context(|| format!("could not read README.md at `{}`", readme_path.display()))?;

    let mut owner = "Unknown".to_string();
    let mut description = "No description available".to_string();

    // Check if content starts with YAML front matter (---)
    if content.starts_with("---\n") {
        if let Some(end_pos) = content[4..].find("\n---\n") {
            let yaml_content = &content[4..end_pos + 4];
            let markdown_content = &content[end_pos + 8..];

            // Parse YAML front matter
            if let Ok(front_matter) = serde_yaml::from_str::<FrontMatter>(yaml_content) {
                if let Some(owner_value) = front_matter.owner {
                    owner = owner_value;
                }
            }

            // Use markdown content as description (first non-empty line)
            for line in markdown_content.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    description = trimmed.to_string();
                    break;
                }
            }
        }
    } else {
        // No YAML front matter, use first non-empty line as description
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                description = trimmed.to_string();
                break;
            }
        }
    }

    Ok((owner, description))
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
                let readme_path = path.join("README.md");
                let (owner, description) = read_readme_info(&readme_path)?;

                features.push(Feature {
                    name: name.to_string(),
                    description,
                    owner,
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
            feature.name, feature.path, feature.owner, feature.description
        );
    }

    Ok(())
}

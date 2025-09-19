use crate::models::Feature;
use colored::*;

pub fn print_features(features: &[Feature], indent: usize, show_description: bool) {
    let prefix = "  ".repeat(indent);

    for feature in features {
        println!(
            "{}{} {} -> {}",
            prefix,
            feature.name,
            format!("[{}]", feature.owner).blue(),
            feature.path.dimmed()
        );
        if show_description {
            println!("{}Description: {}", prefix, feature.description);
        }

        // Recursively print nested features
        if !feature.features.is_empty() {
            print_features(&feature.features, indent + 1, show_description);
        }
    }
}

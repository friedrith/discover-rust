use crate::models::Feature;

pub fn print_features(features: &[Feature], indent: usize) {
    let prefix = "  ".repeat(indent);

    for feature in features {
        println!(
            "{}{} -> {} ({})",
            prefix, feature.name, feature.description, feature.owner
        );
        println!("{}  Path: {}", prefix, feature.path);

        // Recursively print nested features
        if !feature.features.is_empty() {
            print_features(&feature.features, indent + 1);
        }
    }
}

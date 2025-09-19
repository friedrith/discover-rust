use crate::models::Feature;

pub fn print_features(features: &[Feature], indent: usize, show_description: bool) {
    let prefix = "  ".repeat(indent);

    for feature in features {
        println!(
            "{}{} [{}] -> {}",
            prefix, feature.name, feature.owner, feature.path
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

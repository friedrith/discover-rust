use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FrontMatter {
    pub owner: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct Feature {
    pub name: String,
    pub description: String,
    pub owner: String,
    pub path: String,
    pub features: Vec<Feature>,
}

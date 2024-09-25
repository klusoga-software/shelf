use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    pub vers: String,
    pub deps: Vec<Dependency>,
    pub features: HashMap<String, Vec<String>>,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub homepage: Option<String>,
    pub readme: Option<String>,
    pub readme_file: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub repository: Option<String>,
    pub links: Option<String>,
    pub rust_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version_req: String,
    pub features: Vec<String>,
    pub optional: bool,
    pub default_features: bool,
    pub target: Option<String>,
    pub kind: String,
    pub registry: Option<String>,
    pub explicit_name_in_toml: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    pub extras: Vec<String>,
}

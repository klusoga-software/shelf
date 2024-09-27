use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexConfig {
    pub dl: String,
    pub api: String,

    #[serde(rename = "auth-required")]
    pub auth_required: bool,
}

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dependency {
    pub name: String,
    pub version_req: String,
    pub features: Vec<String>,
    pub optional: bool,
    pub default_features: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_name_in_toml: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexDependency {
    pub name: String,
    pub req: String,
    pub features: Vec<String>,
    pub optional: bool,
    pub default_features: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrateIndex {
    pub name: String,
    pub vers: String,
    pub deps: Vec<IndexDependency>,
    pub cksum: String,
    pub features: HashMap<String, Vec<String>>,
    pub yanked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rust_version: Option<String>,
}

impl From<Dependency> for IndexDependency {
    fn from(value: Dependency) -> Self {
        Self {
            package: None,
            req: value.version_req,
            target: value.target,
            registry: value.registry,
            kind: value.kind,
            name: value.name,
            optional: value.optional,
            features: value.features,
            default_features: value.default_features,
        }
    }
}

impl CrateIndex {
    pub fn new_from_metadata(metadata: &Metadata, checksum: String) -> Self {
        let mut deps: Vec<IndexDependency> = vec![];

        for d in metadata.deps.iter() {
            deps.push(d.clone().into());
        }

        Self {
            name: metadata.name.clone(),
            yanked: false,
            deps,
            rust_version: metadata.rust_version.clone(),
            links: metadata.links.clone(),
            vers: metadata.vers.clone(),
            features: metadata.features.clone(),
            cksum: checksum,
        }
    }
}

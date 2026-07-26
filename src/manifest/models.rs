use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractSpec {
    pub id: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DependencySpec {
    pub contract: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactPaths {
    pub guide: Option<String>,
    pub examples: Option<String>,
    pub tests: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(default)]
    pub contracts: Vec<ContractSpec>,
    #[serde(default)]
    pub dependencies: Vec<DependencySpec>,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub artifacts: Option<ArtifactPaths>,
}

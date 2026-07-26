use std::path::{Path, PathBuf};

use crate::manifest::models::{ArtifactPaths, CapabilityManifest};
use crate::manifest::parser::{JsonManifestParser, ManifestParser};
use crate::utils::paths::{MANIFEST_FILENAME, manifest_path_for};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestValidationReport {
    pub path: PathBuf,
    pub valid: bool,
    pub manifest: Option<CapabilityManifest>,
    pub issues: Vec<ValidationIssue>,
}

impl ManifestValidationReport {
    pub fn from_manifest(
        path: PathBuf,
        manifest: CapabilityManifest,
        issues: Vec<ValidationIssue>,
    ) -> Self {
        let valid = issues.is_empty();
        Self {
            path,
            valid,
            manifest: Some(manifest),
            issues,
        }
    }

    pub fn invalid(path: PathBuf, issues: Vec<ValidationIssue>) -> Self {
        Self {
            path,
            valid: false,
            manifest: None,
            issues,
        }
    }
}

pub struct ManifestValidator {
    parser: Box<dyn ManifestParser>,
}

impl Default for ManifestValidator {
    fn default() -> Self {
        Self {
            parser: Box::new(JsonManifestParser),
        }
    }
}

impl ManifestValidator {
    pub fn new(parser: Box<dyn ManifestParser>) -> Self {
        Self { parser }
    }

    pub fn validate_path<P: AsRef<Path>>(&self, path: P) -> ManifestValidationReport {
        let path = path.as_ref();
        let manifest_path = if path.is_dir() {
            manifest_path_for(path)
        } else {
            path.to_path_buf()
        };

        match self.parser.parse_path(&manifest_path) {
            Ok(manifest) => {
                let issues = validate_manifest(&manifest);
                ManifestValidationReport::from_manifest(manifest_path, manifest, issues)
            }
            Err(error) => ManifestValidationReport::invalid(
                manifest_path,
                vec![ValidationIssue {
                    field: MANIFEST_FILENAME.to_string(),
                    message: error.to_string(),
                }],
            ),
        }
    }

    pub fn validate_data(&self, data: &str) -> ManifestValidationReport {
        match self.parser.parse_str(data) {
            Ok(manifest) => {
                let issues = validate_manifest(&manifest);
                ManifestValidationReport::from_manifest(PathBuf::from("<memory>"), manifest, issues)
            }
            Err(error) => ManifestValidationReport::invalid(
                PathBuf::from("<memory>"),
                vec![ValidationIssue {
                    field: "manifest".to_string(),
                    message: error.to_string(),
                }],
            ),
        }
    }
}

fn validate_manifest(manifest: &CapabilityManifest) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    if manifest.name.trim().is_empty() {
        issues.push(ValidationIssue {
            field: "name".to_string(),
            message: "must not be blank".to_string(),
        });
    }

    if manifest.version.trim().is_empty() {
        issues.push(ValidationIssue {
            field: "version".to_string(),
            message: "must not be blank".to_string(),
        });
    }

    issues.extend(validate_contracts(&manifest.contracts));
    issues.extend(validate_dependencies(&manifest.dependencies));
    issues.extend(validate_text_list("keywords", &manifest.keywords));

    if let Some(artifacts) = &manifest.artifacts {
        issues.extend(validate_artifacts(artifacts));
    }

    issues
}

fn validate_contracts(contracts: &[crate::manifest::models::ContractSpec]) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    for (index, contract) in contracts.iter().enumerate() {
        if contract.id.trim().is_empty() {
            issues.push(ValidationIssue {
                field: format!("contracts[{index}].id"),
                message: "must not be blank".to_string(),
            });
        }
        if contract.version.trim().is_empty() {
            issues.push(ValidationIssue {
                field: format!("contracts[{index}].version"),
                message: "must not be blank".to_string(),
            });
        }
    }

    issues
}

fn validate_dependencies(
    dependencies: &[crate::manifest::models::DependencySpec],
) -> Vec<ValidationIssue> {
    dependencies
        .iter()
        .enumerate()
        .filter_map(|(index, dependency)| {
            if dependency.contract.trim().is_empty() {
                Some(ValidationIssue {
                    field: format!("dependencies[{index}].contract"),
                    message: "must not be blank".to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

fn validate_text_list(field: &str, values: &[String]) -> Vec<ValidationIssue> {
    values
        .iter()
        .enumerate()
        .filter_map(|(index, value)| {
            if value.trim().is_empty() {
                Some(ValidationIssue {
                    field: format!("{field}[{index}]"),
                    message: "must not be blank".to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

fn validate_artifacts(artifacts: &ArtifactPaths) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    for (field, value) in [
        ("artifacts.guide", &artifacts.guide),
        ("artifacts.examples", &artifacts.examples),
        ("artifacts.tests", &artifacts.tests),
    ] {
        if value.as_deref().is_some_and(|text| text.trim().is_empty()) {
            issues.push(ValidationIssue {
                field: field.to_string(),
                message: "must not be blank".to_string(),
            });
        }
    }

    issues
}

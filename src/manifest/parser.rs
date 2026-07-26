use std::fs;
use std::path::Path;

use crate::error::Result;
use crate::manifest::models::CapabilityManifest;

pub trait ManifestParser: Send + Sync {
    fn parse_str(&self, source: &str) -> Result<CapabilityManifest>;

    fn parse_path(&self, path: &Path) -> Result<CapabilityManifest>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct JsonManifestParser;

impl ManifestParser for JsonManifestParser {
    fn parse_str(&self, source: &str) -> Result<CapabilityManifest> {
        Ok(serde_json::from_str(source)?)
    }

    fn parse_path(&self, path: &Path) -> Result<CapabilityManifest> {
        let source = fs::read_to_string(path)?;
        self.parse_str(&source)
    }
}

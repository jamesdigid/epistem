use std::path::{Path, PathBuf};

pub const MANIFEST_FILENAME: &str = "epistem.json";

pub fn manifest_path_for(capability_root: &Path) -> PathBuf {
    capability_root.join(MANIFEST_FILENAME)
}

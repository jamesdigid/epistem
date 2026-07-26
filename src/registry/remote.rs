use crate::registry::traits::RegistryProvider;
use crate::storage::capability::FilesystemCapability;

#[derive(Debug, Default)]
pub struct RemoteRegistry;

impl RegistryProvider for RemoteRegistry {
    fn register(&mut self, _capability: FilesystemCapability) {}

    fn capabilities(&self) -> &[FilesystemCapability] {
        &[]
    }

    fn find_by_capability(&self, _contract: &str) -> Vec<FilesystemCapability> {
        Vec::new()
    }
}

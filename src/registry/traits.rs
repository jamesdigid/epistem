use crate::storage::capability::FilesystemCapability;

pub trait RegistryProvider {
    fn register(&mut self, capability: FilesystemCapability);

    fn capabilities(&self) -> &[FilesystemCapability];

    fn find_by_capability(&self, capability: &str) -> Vec<FilesystemCapability>;
}

use crate::error::Result;
use crate::manifest::models::CapabilityManifest;
use crate::resolver::traits::DependencyGraph;
use crate::search::traits::SearchResult;
use crate::storage::capability::FilesystemCapability;

pub trait CatalogProvider {
    fn lookup(&self, contract: &str) -> Vec<FilesystemCapability>;

    fn search(&self, query: &str) -> Vec<SearchResult>;

    fn dependency_graph(
        &self,
        manifests: &[CapabilityManifest],
    ) -> Result<Box<dyn DependencyGraph>>;
}

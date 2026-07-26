use crate::catalog::traits::CatalogProvider;
use crate::error::Result;
use crate::manifest::models::CapabilityManifest;
use crate::registry::local::LocalRegistry;
use crate::registry::traits::RegistryProvider;
use crate::resolver::petgraph_resolver::PetgraphDependencyResolver;
use crate::resolver::traits::{DependencyGraph, DependencyResolver};
use crate::search::local::LocalSearch;
use crate::search::traits::{SearchProvider, SearchResult};
use crate::storage::capability::FilesystemCapability;

#[derive(Default)]
pub struct LocalCatalog {
    registry: LocalRegistry,
    search: LocalSearch,
    resolver: PetgraphDependencyResolver,
}

impl LocalCatalog {
    pub fn new(
        registry: LocalRegistry,
        search: LocalSearch,
        resolver: PetgraphDependencyResolver,
    ) -> Self {
        Self {
            registry,
            search,
            resolver,
        }
    }
}

impl CatalogProvider for LocalCatalog {
    fn lookup(&self, contract: &str) -> Vec<FilesystemCapability> {
        self.registry.find_by_capability(contract)
    }

    fn search(&self, query: &str) -> Vec<SearchResult> {
        self.search.search(query)
    }

    fn dependency_graph(
        &self,
        manifests: &[CapabilityManifest],
    ) -> Result<Box<dyn DependencyGraph>> {
        self.resolver.build(manifests)
    }
}

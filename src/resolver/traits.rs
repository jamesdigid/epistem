use crate::error::Result;
use crate::manifest::models::CapabilityManifest;
use crate::models::DependencyEdge;

pub trait DependencyGraph {
    fn nodes(&self) -> Vec<String>;

    fn edges(&self) -> Vec<DependencyEdge>;

    fn topological_order(&self) -> Result<Vec<String>>;

    fn unresolved_requirements(&self) -> Vec<String>;
}

pub trait DependencyResolver {
    fn build(&self, manifests: &[CapabilityManifest]) -> Result<Box<dyn DependencyGraph>>;
}

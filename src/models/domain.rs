#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyEdge {
    pub provider: String,
    pub dependent: String,
    pub contract: String,
}

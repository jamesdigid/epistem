#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    pub name: String,
    pub score: f32,
    pub description: Option<String>,
}

pub trait SearchProvider {
    fn search(&self, query: &str) -> Vec<SearchResult>;
}

use crate::search::traits::{SearchProvider, SearchResult};

#[derive(Debug, Default, Clone, Copy)]
pub struct LocalSearch;

impl SearchProvider for LocalSearch {
    fn search(&self, _query: &str) -> Vec<SearchResult> {
        Vec::new()
    }
}

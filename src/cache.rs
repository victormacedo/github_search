use std::collections::HashMap;
use std::sync::Mutex;

use crate::models::{CodeSearchResponse, SearchResponse}; // Import your SearchResponse struct

#[derive(Clone, Debug)]
pub enum CachedResponse {
    Search(SearchResponse), // For `search_repositories`
    Code(CodeSearchResponse), // For `search_code`
}

pub struct Cache {
    data: Mutex<HashMap<String, CachedResponse>>, // A thread-safe cache
}

impl Cache {
    // Initialize a new cache
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }

    // Check the cache for a query
    pub fn get(&self, query: &str) -> Option<CachedResponse> {
        let cache = self.data.lock().unwrap(); // Access the cache
        cache.get(query).cloned() // Clone the value if it exists (to avoid borrowing issues)
    }

    // Insert a result into the cache
    pub fn insert(&self, query: &str, response: CachedResponse) {
        let mut cache = self.data.lock().unwrap(); // Access the cache
        cache.insert(query.to_string(), response); // Insert the query and its response
    }
}
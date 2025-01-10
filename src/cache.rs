use std::collections::HashMap;
use std::sync::Mutex;

use crate::models::SearchResponse; // Import your SearchResponse struct

pub struct Cache {
    data: Mutex<HashMap<String, SearchResponse>>, // A thread-safe cache
}

impl Cache {
    // Initialize a new cache
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }

    // Check the cache for a query
    pub fn get(&self, query: &str) -> Option<SearchResponse> {
        let cache = self.data.lock().unwrap(); // Access the cache
        cache.get(query).cloned() // Clone the value if it exists (to avoid borrowing issues)
    }

    // Insert a result into the cache
    pub fn insert(&self, query: &str, response: SearchResponse) {
        let mut cache = self.data.lock().unwrap(); // Access the cache
        cache.insert(query.to_string(), response); // Insert the query and its response
    }
}
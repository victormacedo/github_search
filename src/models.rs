use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Repo {
    pub full_name: String,         // e.g., "rust-lang/rust"
    pub description: Option<String>, // Optional: Not all repos have a description
    pub stargazers_count: u32,     // Number of stars
    pub language: Option<String>, // Primary language
    pub html_url: String,          // Link to repo
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResponse {
    pub total_count: u32,          // Total number of matching repositories
    pub incomplete_results: bool, // If not all results are complete
    pub items: Vec<Repo>,         // A list of repositories
}

#[derive(serde::Deserialize, Debug)]
pub struct RateLimit {
    pub rate: RateLimitInfo, // General API rate limit info
}

#[derive(serde::Deserialize, Debug)]
pub struct RateLimitInfo {
    pub limit: u32,        // Total allowable requests per interval
    pub remaining: u32,    // Remaining requests for the interval
    pub reset: u64,        // Time at which the limit resets (Unix timestamp)
}
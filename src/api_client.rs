use anyhow::anyhow;
use reqwest::Client;
use crate::cache::{Cache, CachedResponse};
use crate::models::{CodeSearchResponse, RateLimit, SearchResponse};

pub async fn search_code(
    client: &Client,
    cache: &Cache,            // Add cache for code search as well
    query: &str,
    filename: Option<&str>,   // Allow limiting search by specific filenames
    per_page: Option<&u32>    // Number of results per page
) -> Result<CodeSearchResponse, anyhow::Error> {

    // Build the full query with optional filename filtering
    let mut full_query = query.to_string();
    if let Some(fname) = filename {
        full_query.push_str(&format!(" filename:{}", fname));
    }

    // Use per_page parameter, defaulting to 10
    let pp = per_page.unwrap_or(&10);

    // Use the full query (query + filters) as the cache key
    let cache_key = format!("code-{}-{}", full_query, pp);

    // Check the cache for this specific query
    if let Some(CachedResponse::Code(cached_response)) = cache.get(&cache_key) {
        println!("Cache hit for code search query: {}", cache_key);
        return Ok(cached_response);
    }

    println!("Cache miss for code search query: {}", cache_key);

    // Query the GitHub Search API (code search endpoint)
    let response = client
        .get("https://api.github.com/search/code")
        .query(&[("q", &full_query)]) // Add query parameters, such as `q=<search_phrase>`
        .query(&[("per_page", pp)])   // Limit results per page
        .header("User-Agent", "github_search_tool")
        .send()
        .await?;

    let status_code = response.status();
    let raw_body = response.text().await?;

    if status_code.eq(&422) {
        return Err(anyhow!("Invalid query syntax: {}", raw_body));
    } else if status_code.eq(&401) {
        return Err(anyhow!("Invalid token: {}", raw_body));
    } else if status_code.eq(&403) {
        return Err(anyhow!("Permission denied: {}", raw_body));
    } else if status_code.is_client_error() {
        return Err(anyhow!("Unexpected client error: {}", raw_body));
    } else if status_code.is_server_error() {
        return Err(anyhow!("Unexpected server error: {}", raw_body));
    }

    // Deserialize the response as `CodeSearchResponse`
    let result: CodeSearchResponse = serde_json::from_str(&raw_body).unwrap();

    // Insert the new result into the cache
    cache.insert(&cache_key, CachedResponse::Code(result.clone()));

    Ok(result)
}

pub async fn search_repositories(
    client: &Client,
    cache: &Cache,            // Add cache as a parameter
    query: &str,
    per_page: Option<&u32>
) -> Result<SearchResponse, anyhow::Error> {

    let pp = per_page.unwrap_or(&10);
    let cache_key = format!("{}-{}", query, pp);

    // Check if the query result is in the cache
    if let Some(CachedResponse::Search(cached_response)) = cache.get(&cache_key) {
        println!("Cache hit for query: {}", cache_key);
        return Ok(cached_response); // Return the cached response
    }

    println!("Cache miss for query: {}", query);

    let response = client
        .get("https://api.github.com/search/repositories")
        .query(&[("q", query)]) // Add the query as a GET parameter
        .query(&[("per_page", pp)]) // Add per_page as a GET parameter
        .send()
        .await?;

    let status_code = response.status();
    let raw_body = response.text().await?;

    if status_code.eq(&422) {
        return Err(anyhow!("Invalid query syntax: {}", raw_body));
    } else if status_code.eq(&401) {
        return Err(anyhow!("Invalid token: {}", raw_body));
    } else if status_code.eq(&403) {
        return Err(anyhow!("Permission denied: {}", raw_body));
    } else if status_code.is_client_error() {
        return Err(anyhow!("Unexpected client error: {}", raw_body));
    } else if status_code.is_server_error() {
        return Err(anyhow!("Unexpected server error: {}", raw_body));
    }

    let result: SearchResponse = serde_json::from_str(&raw_body).unwrap();

    // Insert the new result into the cache
    cache.insert(&cache_key, CachedResponse::Search(result.clone()));

    Ok(result)
}

pub async fn check_rate_limit(client: &Client) -> Result<RateLimit, anyhow::Error> {
    // Make the request to the rate limit endpoint
    let response = client
        .get("https://api.github.com/rate_limit")
        .send()
        .await?
        .json::<RateLimit>() // Deserialize JSON into `RateLimit`
        .await?;

    if response.rate.remaining < 1 {
        return Err(anyhow!(
            "{} requests remaining (out of {}). Limit resets at {}.",
                response.rate.remaining,
                response.rate.limit,
                chrono::NaiveDateTime::from_timestamp(response.rate.reset as i64, 0)
                    .format("%Y-%m-%d %H:%M:%S")
        ));
    }

    Ok(response)
}
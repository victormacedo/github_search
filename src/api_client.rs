use anyhow::anyhow;
use reqwest::Client;
use crate::models::{RateLimit, SearchResponse};

pub async fn search_repositories(client: &Client, query: &str, per_page: Option<&u32>) -> Result<SearchResponse, anyhow::Error> {
    let response = client
        .get("https://api.github.com/search/repositories")
        .query(&[("q", query)]) // Add the query as a GET parameter
        .query(&[("per_page", per_page.unwrap_or(&10))]) // Add per_page as a GET parameter
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
mod models;
mod search_query;
mod api_client;
mod errors;

use dotenv::dotenv;
use std::env;
use reqwest::Client;
use api_client::search_repositories;
use crate::api_client::check_rate_limit;
use crate::search_query::GithubSearchQuery;

#[tokio::main] // Marks the main function as asynchronous
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token = env::var("GITHUB_TOKEN").expect("Expected a GITHUB_TOKEN in the environment");

    // Create a new HTTP client with the Authorization header
    let client = Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", token).parse().unwrap(),
            );
            headers.insert(
                reqwest::header::USER_AGENT,
                "LeapTheory-Test-App/1.0".parse().unwrap(),
            );
            headers
        })
        .build()?;

    match check_rate_limit(&client).await {
        Ok(limit) => {
            println!("{} requests remaining", limit.rate.remaining);
        },
        Err(err) => {
            println!("Rate limit error: {}", err.to_string());
            std::process::exit(1);
        }
    }

    let query = GithubSearchQuery::new("rust async")
        .language("rust")
        .min_stars("5000")
        .to_query_string();

    // Send the search request
    match search_repositories(&client, &query, Some(&10)).await {
        Ok(response) => {
            println!("Found {} repositories:", response.total_count);
            for repo in response.items {
                println!("- {} ({} stars)", repo.full_name, repo.stargazers_count);
            }
        },
        Err(err) => {
            eprintln!("Error while searching: {}", err);
        },
    }

    Ok(())
}

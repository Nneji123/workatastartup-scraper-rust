#![allow(dead_code)]
#[allow(clippy::redundant_pattern_matching)]
mod config;
mod models;
mod scraper;
mod selectors;
mod utils;
use log::info;
#[tokio::main]
async fn main() {
    // Create a new instance of Scraper
    let dets: config::Config = config::Config::from_env();
    let username: String = dets.login_username;
    let password: String = dets.login_password;
    // Perform login
    if let Ok(_) = scraper::login(&username, &password).await {
        info!("Login successful");
    } else {
        info!("Login failed");
    }
    let company_url: &str = "https://workatastartup.com/companies/vocode";
    if let Ok(founders) = scraper::scrape_founders_data(company_url).await {
        info!("Scraping founders successful");
        println!("{:?}", founders);
    } else {
        info!("Login failed");
    }

    let job_url: &str = "https://www.workatastartup.com/jobs/64333";
    if let Ok(jobs) = scraper::scrape_job_data(job_url).await {
        info!("Scraping jobs successful!");
        println!("{:?}", jobs);
    } else {
        info!("Login failed");
    }
}

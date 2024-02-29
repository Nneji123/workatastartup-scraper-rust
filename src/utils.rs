#![allow(dead_code)]

use regex::Regex;

use soup::prelude::*;

// Define a custom error type for invalid URLs
#[derive(Debug)]
pub struct InvalidURLException(String);

impl std::fmt::Display for InvalidURLException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid URL: {}", self.0)
    }
}

/// Function to validate the job URL
pub fn validate_job_url(input_url: &str) -> Result<(), InvalidURLException> {
    let url_pattern = Regex::new(r"https://www.workatastartup.com/jobs").unwrap();

    if !url_pattern.is_match(input_url) {
        return Err(InvalidURLException(input_url.to_string()));
    }

    Ok(())
}

/// Function to validate company url
pub fn validate_company_url(input_url: &str) -> Result<(), InvalidURLException> {
    let url_pattern = Regex::new(r"https://www.workatastartup.com/companies").unwrap();

    if !url_pattern.is_match(input_url) {
        return Err(InvalidURLException(input_url.to_string()));
    }
    Ok(())
}

/// Function to strip HTML tags from the input HTML content
pub fn strip_html_tags(html_content: &str) -> String {
    let soup = Soup::new(html_content);
    let text_content: String = soup.text();
    text_content
}

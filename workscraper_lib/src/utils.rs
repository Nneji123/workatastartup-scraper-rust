#![allow(dead_code)]

use regex::Regex;
use soup::prelude::*;
use thirtyfour::error::WebDriverError;

/// Function to validate the job URL
pub fn validate_job_url(input_url: &str) -> Result<(), WebDriverError> {
    let url_pattern = Regex::new(r"https://www.workatastartup.com/jobs").unwrap();

    if !url_pattern.is_match(input_url) {
        return Err(WebDriverError::CustomError(format!(
            "Invalid job URL: {}",
            input_url
        )));
    }

    Ok(())
}

/// Function to validate company url
pub fn validate_company_url(input_url: &str) -> Result<(), WebDriverError> {
    let url_pattern = Regex::new(r"https://www.workatastartup.com/companies").unwrap();

    if !url_pattern.is_match(input_url) {
        return Err(WebDriverError::CustomError(format!(
            "Invalid job URL: {}",
            input_url
        )));
    }

    Ok(())
}

/// Function to strip HTML tags from the input HTML content
pub fn strip_html_tags(html_content: &str) -> String {
    let soup = Soup::new(html_content);
    let text_content: String = soup.text();
    text_content
}

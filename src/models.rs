#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FounderData {
    pub founder_name: String,
    pub founder_description: String,
    pub founder_image_url: Option<String>,
    pub founder_linkedin_url: Option<String>,
    pub founder_emails: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JobData {
    job_url: String,
    job_title: String,
    job_salary_range: String,
    job_tags: Vec<String>,
    job_description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CompanyData {
    company_name: String,
    company_url: String,
    company_description: String,
    company_tags: Vec<String>,
    company_image: String,
    company_social_links: Vec<String>,
    company_job_links: Vec<String>,
    company_founders: Vec<FounderData>,
    job_data: Vec<JobData>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ScrapedData {
    scraped_data: Vec<CompanyData>,
}

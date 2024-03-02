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
pub struct JobData {
    pub job_url: String,
    pub job_title: String,
    pub job_salary_range: String,
    pub job_tags: Vec<String>,
    pub job_description: String,
}

impl Default for JobData {
fn default() -> Self {
Self::new()
}
}

impl JobData {
    pub fn new() -> Self {
        JobData {
            job_url: String::new(),
            job_title: String::new(),
            job_salary_range: String::new(),
            job_tags: Vec::new(),
            job_description: String::new(),
        }
    }
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

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FounderData {
    pub founder_name: String,
    pub founder_description: String,
    pub founder_image_url: Option<String>,
    pub founder_linkedin_url: Option<String>,
    pub founder_emails: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CompanyData {
    pub company_name: String,
    pub company_url: String,
    pub company_description: String,
    pub company_tags: Vec<String>,
    pub company_image: String,
    pub company_social_links: Vec<String>,
    pub company_job_links: Vec<String>,
    pub company_founders: Vec<FounderData>,
    pub job_data: Vec<JobData>,
}

impl Default for CompanyData {
    fn default() -> Self {
        Self::new()
    }
}

impl CompanyData {
    pub fn new() -> Self {
        CompanyData {
            company_url: String::new(),
            company_image: String::new(),
            company_description: String::new(),
            company_founders: Vec::<FounderData>::new(),
            company_name: String::new(),
            company_job_links: Vec::new(),
            company_social_links: Vec::new(),
            company_tags: Vec::new(),
            job_data: Vec::<JobData>::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScrapedData {
    pub scraped_data: Vec<CompanyData>,
}

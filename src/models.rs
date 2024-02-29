use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct FounderData {
    founder_name: String,
    founder_description: String,
    founder_image_url: String,
    founder_linkedin_url: String,
    founder_emails: Option<Vec<String>>,
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

pub fn test_model() {
    // Example usage: deserialize JSON string to Rust struct
    let json_str: &str = r#"
        {
            "scraped_data": [
                {
                    "company_name": "Example Company",
                    "company_url": "https://example.com",
                    "company_description": "Description of Example Company",
                    "company_tags": ["tag1", "tag2"],
                    "company_image": "image_url",
                    "company_social_links": ["link1", "link2"],
                    "company_job_links": ["job1", "job2"],
                    "company_founders": [
                        {
                            "founder_name": "Founder 1",
                            "founder_description": "Description of Founder 1",
                            "founder_image_url": "founder_image_url",
                            "founder_linkedin_url": "linkedin_url",
                            "founder_emails": ["email1", "email2"]
                        }
                    ],
                    "job_data": [
                        {
                            "job_url": "job_url",
                            "job_title": "Job Title",
                            "job_salary_range": "Salary Range",
                            "job_tags": ["tag1", "tag2"],
                            "job_description": "Job Description"
                        }
                    ]
                }
            ]
        }
    "#;

    let scraped_data: ScrapedData =
        serde_json::from_str(json_str).expect("Failed to deserialize JSON");
    println!("{:?}", scraped_data);
}

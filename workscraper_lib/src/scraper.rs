#![allow(dead_code)]
use crate::models::{CompanyData, FounderData, JobData};
use crate::selectors;
use crate::utils::{strip_html_tags, validate_company_url, validate_job_url};
use log::info;
use std::collections::HashMap;
use std::process::Command;
use std::thread;
use std::time::Duration;
use thirtyfour::prelude::*;

const DRIVER_WAIT_DURATION: u64 = 10;
const SCROLL_PAUSE_TIME: f32 = 0.5;

pub async fn find_element_by_class(
    driver: WebDriver,
    class_name: &str,
) -> WebDriverResult<WebElement> {
    driver.find(By::ClassName(class_name)).await
}

pub async fn find_elements_by_class(
    driver: WebDriver,
    class_name: &str,
) -> WebDriverResult<Vec<WebElement>> {
    driver.find_all(By::ClassName(class_name)).await
}

pub async fn create_chrome_driver(port: i32) -> WebDriverResult<WebDriver> {
    let chromedriver_cmd: &str = if std::env::consts::OS == "windows" {
        "../chromedriver.exe"
    } else {
        "../chromedriver"
    };

    let _chromedriver_url: std::process::Child = Command::new(chromedriver_cmd)
        .arg(format!("--port={port}"))
        .spawn()
        .expect("Chromedriver not found. Make sure you have it installed!");
    std::thread::sleep(std::time::Duration::from_secs(1));
    let chrome_options: fn() -> thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome;
    let mut capabilities = chrome_options();
    capabilities
        .add_chrome_arg("--headless")
        .expect("Error occurred with headless mode");

    capabilities
        .add_chrome_arg("--disable-dev-shm-usage")
        .expect("Error occurred with shm usage");

    capabilities
        .add_chrome_arg("--log-level=3")
        .expect("Error occurred reducing log level");

    capabilities
        .add_chrome_arg("--no-sandbox")
        .expect("Error occurred setting sandbox mode");

    let local_host_url: String = format!("http://localhost:{port}");
    let driver: WebDriver = WebDriver::new(&local_host_url, capabilities).await?;
    Ok(driver)
}

/// Function to login to WorkataStartup.com
pub async fn login(username: &str, password: &str) -> WebDriverResult<bool> {
    let port: i32 = 9515;

    let driver: WebDriver = create_chrome_driver(port).await?;
    info!("WebDriver initialized");
    driver.goto("https://workatastartup.com/companies").await?;
    let login_button: WebElement = driver
        .query(By::XPath(selectors::LOGIN_BUTTON_XPATH))
        .first()
        .await?;
    login_button.wait_until().displayed().await?;
    login_button.click().await?;

    let username_input: WebElement = driver
        .query(By::XPath(selectors::USERNAME_INPUT_XPATH))
        .first()
        .await?;
    username_input.wait_until().displayed().await?;
    let password_input: WebElement = driver
        .query(By::XPath(selectors::PASSWORD_INPUT_XPATH))
        .first()
        .await?;
    password_input.wait_until().displayed().await?;
    let submit_button: WebElement = driver
        .query(By::XPath(selectors::SUBMIT_BUTTON_XPATH))
        .first()
        .await?;
    submit_button.wait_until().displayed().await?;

    username_input.send_keys(username).await?;
    password_input.send_keys(password).await?;
    submit_button.click().await?;
    info!("Successfully logged in!");
    driver.quit().await?;
    Ok(true)
}
/// Scrape Company Data
pub async fn scrape_company_data(company_url: &str) -> Result<(CompanyData, bool), WebDriverError> {
    // Validate the company URL
    match validate_company_url(company_url) {
        Ok(()) => {
            // Proceed with scraping
            println!("Scraping company data from: {}", company_url);

            let port: i32 = 9515;

            let driver: WebDriver = create_chrome_driver(port).await?;
            info!("WebDriver initialized");
            driver.goto(company_url).await?;
            let mut company_data: CompanyData = CompanyData::new();
            company_data.company_url = String::from(company_url);
            thread::sleep(Duration::from_secs(DRIVER_WAIT_DURATION));
            company_data.company_name =
                find_element_by_class(driver.clone(), selectors::COMPANY_NAME_CLASS)
                    .await?
                    .text()
                    .await?;
            company_data.company_image =
                find_element_by_class(driver.clone(), selectors::COMPANY_IMAGE_CLASS)
                    .await?
                    .text()
                    .await?;
            company_data.company_description = driver
                .find(By::ClassName(selectors::COMPANY_DESCRIPTION_CLASS_ONE))
                .await?
                .text()
                .await
                .or(driver
                    .find(By::ClassName(selectors::COMPANY_DESCRIPTION_CLASS_TWO))
                    .await?
                    .text()
                    .await)?;

            let mut job_links = Vec::new();
            let company_job_links = driver
                .find_all(By::ClassName(selectors::COMPANY_JOB_CLASS))
                .await?;
            for i in 0..company_job_links.len() {
                if let Some(job_link) = company_job_links[i].attr("href").await? {
                    job_links.push(job_link);
                }
            }
            company_data.company_job_links = job_links.clone();

            let company_tags_elements: Vec<WebElement> = driver
                .find_all(By::ClassName(selectors::COMPANY_TAGS_CLASS))
                .await?;
            company_data.company_tags = Vec::new();
            for tag in company_tags_elements {
                company_data
                    .company_tags
                    .push(String::from(tag.text().await?.trim()));
            }

            let social_prefixes: HashMap<usize, &str> = [
                (0, "https://"),
                (1, "https://twitter.com/"),
                (2, "https://facebook.com/"),
            ]
            .iter()
            .cloned()
            .collect();

            let mut social_links: Vec<String> = Vec::new();
            let elements = driver
                .find_all(By::ClassName(selectors::COMPANY_SOCIAL_CLASS))
                .await?;
            for (i, link) in elements.iter().enumerate() {
                if let Ok(text) = link.text().await {
                    let trimmed_text = text.trim().to_owned();
                    if !trimmed_text.is_empty() {
                        let prefix = social_prefixes.get(&i).unwrap_or(&"");
                        social_links.push(format!("{}{}", prefix, trimmed_text));
                    }
                }
            }
            let (founders_data, _) = scrape_founders_data(company_url).await?;
            println!("{:?}", founders_data);
            let mut job_data_list: Vec<JobData> = Vec::new();
            let new_job_links = job_links.clone();
            for job_link in new_job_links.iter() {
                let (job_data, _) = scrape_job_data(job_link).await?;
                job_data_list.push(job_data);
            }
            company_data.job_data = job_data_list;
            company_data.company_founders = founders_data;
            company_data.company_social_links = social_links;
            info!("Successfully scraped company details from: {}", company_url);
            driver.quit().await?;

            Ok((company_data, true))
        }
        Err(e) => Err(e),
    }
}

/// Scraper Founders Data Based on the company url.
pub async fn scrape_founders_data(
    company_url: &str,
) -> Result<(Vec<FounderData>, bool), WebDriverError> {
    // Validate the company URL
    match validate_company_url(company_url) {
        Ok(()) => {
            // Proceed with scraping
            println!("Scraping company founders from: {}", company_url);

            let mut founders_list: Vec<FounderData> = Vec::new();

            let port: i32 = 9515;

            let driver: WebDriver = create_chrome_driver(port).await?;
            info!("WebDriver initialized");
            driver.goto(company_url).await?;
            thread::sleep(Duration::from_secs(DRIVER_WAIT_DURATION));
            let founders_names: Vec<WebElement> = driver
                .find_all(By::ClassName(selectors::FOUNDER_NAME_CLASS))
                .await?;
            let founders_images: Vec<WebElement> = driver
                .find_all(By::ClassName(selectors::FOUNDER_IMAGE_CLASS))
                .await?;
            let founders_descriptions: Vec<WebElement> = driver
                .find_all(By::ClassName(selectors::FOUNDER_DESCRIPTION_CLASS_ONE))
                .await
                .or(driver
                    .find_all(By::ClassName(selectors::FOUNDER_DESCRIPTION_CLASS_TWO))
                    .await)?;
            let founders_linkedins: Vec<WebElement> = driver
                .find_all(By::ClassName(selectors::FOUNDER_LINKEDIN_CLASS))
                .await?;
            for i in 0..founders_names.len() {
                let founder_data: FounderData = FounderData {
                    founder_name: founders_names[i].text().await?,
                    founder_image_url: founders_images[i].attr("src").await?,
                    founder_description: founders_descriptions[i].text().await?,
                    founder_linkedin_url: founders_linkedins[i].attr("href").await?,
                    founder_emails: None,
                };
                founders_list.push(founder_data);
            }

            info!(
                "Successfully scraped founder's details from: {}",
                company_url
            );
            driver.quit().await?;

            Ok((founders_list, true))
        }
        Err(e) => Err(e),
    }
}

///Function to scrape job details from WorkataStartup.com
pub async fn scrape_job_data(job_url: &str) -> Result<(JobData, bool), WebDriverError> {
    // Validate the company URL
    match validate_job_url(job_url) {
        Ok(()) => {
            // Proceed with scraping
            println!("Scraping job details from: {}", job_url);

            let port: i32 = 9515;

            let driver: WebDriver = create_chrome_driver(port).await?;
            info!("WebDriver initialized");
            driver.goto(job_url).await?;
            let mut job_data: JobData = JobData::new();
            job_data.job_url = String::from(job_url);
            thread::sleep(Duration::from_secs(DRIVER_WAIT_DURATION));

            job_data.job_title = find_element_by_class(driver.clone(), selectors::JOB_TITLE_CLASS)
                .await?
                .text()
                .await?;
            job_data.job_description = strip_html_tags(
                &find_element_by_class(driver.clone(), selectors::JOB_DESCRIPTION_CLASS)
                    .await?
                    .text()
                    .await?,
            );
            let job_tags_elements: Vec<WebElement> = driver
                .find_all(By::ClassName(selectors::JOB_TAGS_CLASS))
                .await?;
            job_data.job_tags = Vec::new();
            for tag in job_tags_elements {
                let new_tag: String = tag.text().await?;
                let split_tags: Vec<&str> = new_tag.split('\n').collect();
                for split_tag in split_tags {
                    job_data.job_tags.push(split_tag.to_string());
                }
            }

            job_data.job_salary_range =
                find_element_by_class(driver.clone(), selectors::SALARY_RANGE_CLASS)
                    .await?
                    .text()
                    .await?
                    .trim()
                    .replace('•', "");
            info!("Successfully scraped job details from: {}", job_url);
            driver.quit().await?;

            Ok((job_data, true))
        }
        Err(e) => Err(e),
    }
}

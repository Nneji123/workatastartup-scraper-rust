#![allow(dead_code)]
use crate::models::{FounderData, JobData};
use crate::selectors;
use crate::utils::{strip_html_tags, validate_company_url, validate_job_url};
use log::info;
use std::process::Command;
use std::thread;
use std::time::Duration;
use thirtyfour::prelude::*;

const DRIVER_WAIT_DURATION: u64 = 10;
const SCROLL_PAUSE_TIME: f32 = 0.5;

pub async fn perform_action_on_element(
    driver: WebDriver,
    xpath: &str,
    action: &str,
    value: Option<&str>,
) -> WebDriverResult<WebElement> {
    let element: WebElement = driver.query(By::XPath(xpath)).first().await?;
    element.wait_until().displayed().await?;
    match action {
        "click" => element.click().await?,
        "send_keys" => {
            if let Some(keys) = value {
                element.send_keys(keys).await?;
            }
        }
        _ => (), // Handle other actions if needed
    };
    Ok(element)
}

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
        "./chromedriver.exe"
    } else {
        "./chromedriver"
    };

    let chromedriver_url: std::process::Child = Command::new(chromedriver_cmd)
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
        .expect("Error occurred with headless mode");

    capabilities
        .add_chrome_arg("--log-level=3")
        .expect("Error occurred with headless mode");

    capabilities
        .add_chrome_arg("--no-sandbox")
        .expect("Error occurred with headless mode");

    let local_host_url: String = format!("http://localhost:{port}");
    let driver: WebDriver = WebDriver::new(&local_host_url, capabilities).await?;
    Ok(driver)
}

/// Function to login to WorkataStartup.com
pub async fn login(username: &str, password: &str) -> WebDriverResult<bool> {
    let port: i32 = 9515;

    let driver: WebDriver = create_chrome_driver(port).await?;
    info!("WebDriver initialized");
    driver
        .goto("https://www.workatastartup.com/companies")
        .await?;

    let username_input: WebElement = perform_action_on_element(
        driver.clone(),
        selectors::USERNAME_INPUT_XPATH,
        "send_keys",
        Some(username),
    )
    .await?;
    let password_input: WebElement = perform_action_on_element(
        driver.clone(),
        selectors::PASSWORD_INPUT_XPATH,
        "send_keys",
        Some(password),
    )
    .await?;
    let login_button: WebElement =
        perform_action_on_element(driver.clone(), selectors::LOGIN_BUTTON_XPATH, "click", None)
            .await?;
    let submit_button: WebElement = perform_action_on_element(
        driver.clone(),
        selectors::SUBMIT_BUTTON_XPATH,
        "click",
        None,
    )
    .await?;
    driver.quit().await?;
    info!("Successfully logged in!");
    Ok(true)
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
            let founders_names: Vec<WebElement> =
                find_elements_by_class(driver.clone(), selectors::FOUNDER_NAME_CLASS).await?;
            let founders_images: Vec<WebElement> =
                find_elements_by_class(driver.clone(), selectors::FOUNDER_IMAGE_CLASS).await?;
            let founders_descriptions: Vec<WebElement> =
                find_elements_by_class(driver.clone(), selectors::FOUNDER_DESCRIPTION_CLASS_ONE)
                    .await
                    .or(find_elements_by_class(
                        driver.clone(),
                        selectors::FOUNDER_DESCRIPTION_CLASS_TWO,
                    )
                    .await)?;
            let founders_linkedins: Vec<WebElement> =
                find_elements_by_class(driver.clone(), selectors::FOUNDER_LINKEDIN_CLASS).await?;
            for i in 0..founders_names.len() {
                let founder_name: String = founders_names[i].text().await?;
                let founder_image_url: Option<String> = founders_images[i].attr("src").await?;
                let founder_description: String = founders_descriptions[i].text().await?;
                let founder_linkedin_url: Option<String> =
                    founders_linkedins[i].attr("href").await?;

                let founder_data: FounderData = FounderData {
                    founder_name,
                    founder_image_url,
                    founder_description,
                    founder_linkedin_url,
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

            let job_title_element =
                find_element_by_class(driver.clone(), selectors::JOB_TITLE_CLASS).await?;
            let job_title_text = job_title_element.text().await?;

            let job_description_element =
                find_element_by_class(driver.clone(), selectors::JOB_DESCRIPTION_CLASS).await?;
            let job_description_text = strip_html_tags(&job_description_element.text().await?);

            let job_tags_elements =
                find_elements_by_class(driver.clone(), selectors::JOB_TAGS_CLASS).await?;
            let mut job_tags: Vec<String> = Vec::new();
            for tag in job_tags_elements {
                let new_tag: String = tag.text().await?;
                let split_tags: Vec<&str> = new_tag.split('\n').collect();
                for split_tag in split_tags {
                    job_tags.push(split_tag.to_string());
                }
            }

            let salary_range_element =
                find_element_by_class(driver.clone(), selectors::SALARY_RANGE_CLASS).await?;
            let salary_range_text = salary_range_element.text().await?.trim().replace('•', "");

            job_data.job_salary_range = salary_range_text;
            job_data.job_title = job_title_text;
            job_data.job_description = job_description_text;
            job_data.job_tags = job_tags;

            info!("Successfully scraped job details from: {}", job_url);
            driver.quit().await?;

            Ok((job_data, true))
        }
        Err(e) => Err(e),
    }
}

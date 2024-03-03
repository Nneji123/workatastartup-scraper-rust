#![allow(dead_code)]
use crate::models::{FounderData, JobData};
use crate::selectors;
use crate::utils::{strip_html_tags, validate_company_url, validate_job_url};
use log::info;
use std::thread;
use std::time::Duration;
use thirtyfour::prelude::*;

const DRIVER_WAIT_DURATION: u64 = 10;
const SCROLL_PAUSE_TIME: f32 = 0.5;

async fn perform_action_on_element(
    driver: &WebDriver,
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

async fn find_element_by_class(
    driver: &WebDriver,
    class_name: &str,
) -> WebDriverResult<WebElement> {
    driver.find(By::ClassName(class_name)).await
}

async fn find_elements_by_class(
    driver: &WebDriver,
    class_name: &str,
) -> WebDriverResult<Vec<WebElement>> {
    driver.find_all(By::ClassName(class_name)).await
}

pub struct Scraper {
    pub driver: Option<WebDriver>,
}

impl Default for Scraper {
    fn default() -> Self {
        Self::new()
    }
}

impl Scraper {
    pub fn new() -> Self {
        Self { driver: None }
    }
    async fn initialize_driver(&self) -> WebDriverResult<WebDriver> {
        let chrome_options: fn() -> thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome;
        chrome_options()
            .add_chrome_arg("--headless")
            .expect("Error occured with headless mode");

        let driver: WebDriver = WebDriver::new("http://localhost:9515", chrome_options()).await?;
        info!("WebDriver initialized");
        Ok(driver)
    }
    /// Function to login to WorkataStartup.com
    pub async fn login(&mut self, username: &str, password: &str) -> WebDriverResult<bool> {
        if self.driver.is_none() {
            self.driver = Some(self.initialize_driver().await?);
        }

        let driver: &WebDriver = self.driver.as_ref().unwrap();
        driver
            .goto("https://www.workatastartup.com/companies")
            .await?;

        let _username_input: WebElement = perform_action_on_element(
            driver,
            selectors::USERNAME_INPUT_XPATH,
            "send_keys",
            Some(username),
        )
        .await?;
        let _password_input: WebElement = perform_action_on_element(
            driver,
            selectors::PASSWORD_INPUT_XPATH,
            "send_keys",
            Some(password),
        )
        .await?;
        let _login_button: WebElement =
            perform_action_on_element(driver, selectors::LOGIN_BUTTON_XPATH, "click", None).await?;
        let _submit_button: WebElement =
            perform_action_on_element(driver, selectors::SUBMIT_BUTTON_XPATH, "click", None)
                .await?;
        driver.quit();
        info!("Successfully logged in!");
        Ok(true)
    }
    /// Scraper Founders Data Based on the company url.
    pub async fn scrape_founders_data(
        &mut self,
        company_url: &str,
    ) -> Result<(Vec<FounderData>, bool), WebDriverError> {
        // Validate the company URL
        match validate_company_url(company_url) {
            Ok(()) => {
                // Proceed with scraping
                println!("Scraping company founders from: {}", company_url);

                let mut founders_list: Vec<FounderData> = Vec::new();

                if self.driver.is_none() {
                    self.driver = Some(self.initialize_driver().await?);
                }

                let driver: &WebDriver = self.driver.as_ref().unwrap();
                driver.goto(company_url).await?;

                thread::sleep(Duration::from_secs(DRIVER_WAIT_DURATION));
                let founders_names: Vec<WebElement> =
                    find_elements_by_class(driver, selectors::FOUNDER_NAME_CLASS).await?;
                let founders_images: Vec<WebElement> =
                    find_elements_by_class(driver, selectors::FOUNDER_IMAGE_CLASS).await?;
                let founders_descriptions: Vec<WebElement> =
                    find_elements_by_class(driver, selectors::FOUNDER_DESCRIPTION_CLASS_ONE)
                        .await
                        .or(find_elements_by_class(
                            driver,
                            selectors::FOUNDER_DESCRIPTION_CLASS_TWO,
                        )
                        .await)?;
                let founders_linkedins: Vec<WebElement> =
                    find_elements_by_class(driver, selectors::FOUNDER_LINKEDIN_CLASS).await?;
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
                driver.quit();
                Ok((founders_list, true))
            }
            Err(e) => Err(e),
        }
    }

    ///Function to scrape job details from WorkataStartup.com
    pub async fn scrape_job_data(
        &mut self,
        job_url: &str,
    ) -> Result<(JobData, bool), WebDriverError> {
        // Validate the company URL
        match validate_job_url(job_url) {
            Ok(()) => {
                // Proceed with scraping
                println!("Scraping job details from: {}", job_url);

                if self.driver.is_none() {
                    self.driver = Some(self.initialize_driver().await?);
                }

                let driver: &WebDriver = self.driver.as_ref().unwrap();
                driver.goto(job_url).await?;
                let mut job_data: JobData = JobData::new();
                job_data.job_url = String::from(job_url);
                thread::sleep(Duration::from_secs(DRIVER_WAIT_DURATION));

                let job_title_element =
                    find_element_by_class(driver, selectors::JOB_TITLE_CLASS).await?;
                let job_title_text = job_title_element.text().await?;

                let job_description_element =
                    find_element_by_class(driver, selectors::JOB_DESCRIPTION_CLASS).await?;
                let job_description_text = strip_html_tags(&job_description_element.text().await?);

                let job_tags_elements =
                    find_elements_by_class(driver, selectors::JOB_TAGS_CLASS).await?;
                let mut job_tags: Vec<String> = Vec::new();
                for tag in job_tags_elements {
                    let new_tag: String = tag.text().await?;
                    let split_tags: Vec<&str> = new_tag.split('\n').collect();
                    for split_tag in split_tags {
                        job_tags.push(split_tag.to_string());
                    }
                }

                let salary_range_element =
                    find_element_by_class(driver, selectors::SALARY_RANGE_CLASS).await?;
                let salary_range_text = salary_range_element.text().await?.trim().replace('•', "");

                job_data.job_salary_range = salary_range_text;
                job_data.job_title = job_title_text;
                job_data.job_description = job_description_text;
                job_data.job_tags = job_tags;

                info!("Successfully scraped job details from: {}", job_url);
                driver.quit();

                Ok((job_data, true))
            }
            Err(e) => Err(e),
        }
    }
}

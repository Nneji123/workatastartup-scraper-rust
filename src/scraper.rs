#![allow(dead_code)]
use crate::models::FounderData;
use crate::selectors;
// use crate::utils::{strip_html_tags, validate_company_url};
use log::info;
// use std::process::Command;
use std::thread;
use std::time::Duration;
use thirtyfour::prelude::*;
// use tokio::time;

const MAX_WORKERS: i32 = 5;
const SCROLL_PAUSE_TIME: f32 = 0.5;

pub struct Scraper {
    pub driver: Option<WebDriver>,
}

impl Scraper {
    pub fn new() -> Self {
        Self { driver: None }
    }
    async fn initialize_driver(&self) -> WebDriverResult<WebDriver> {
        let chrome_options: fn() -> thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome;

        chrome_options()
            .set_binary("./chromedriver.exe")
            .expect("Error getting chromedriver path");
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
        login_button.click().await?;
        info!("Successfully logged in!");
        Ok(true)
    }
    /// Scraper Founders Data Based on the company url.
    pub async fn scrape_founders_data(
        &mut self,
        company_url: &str,
    ) -> WebDriverResult<(Vec<FounderData>, bool)> {
        let mut founders_list: Vec<FounderData> = Vec::new();

        if self.driver.is_none() {
            self.driver = Some(self.initialize_driver().await?);
        }

        let driver: &WebDriver = self.driver.as_ref().unwrap();
        driver.goto(company_url).await?;

        thread::sleep(Duration::from_secs(5));
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
            let founder_name: String = founders_names[i].text().await?;
            let founder_image_url: Option<String> = founders_images[i].attr("src").await?;
            let founder_description: String = founders_descriptions[i].text().await?;
            let founder_linkedin_url: Option<String> = founders_linkedins[i].attr("href").await?;

            let founder_data: FounderData = FounderData {
                founder_name: founder_name,
                founder_image_url: founder_image_url,
                founder_description: founder_description,
                founder_linkedin_url: founder_linkedin_url,
                founder_emails: None,
            };

            founders_list.push(founder_data);
        }

        info!(
            "Successfully scraped founder's details from: {}",
            company_url
        );
        Ok((founders_list, true))
    }
}

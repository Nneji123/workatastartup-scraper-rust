#![allow(dead_code)]
// use crate::utils;
use crate::selectors::{
    LOGIN_BUTTON_XPATH, PASSWORD_INPUT_XPATH, SUBMIT_BUTTON_XPATH, USERNAME_INPUT_XPATH,
};
use log::info;
use std::path::Path;
use std::process::Command;
use thirtyfour::prelude::*;

const MAX_WORKERS: i32 = 5;
const SCROLL_PAUSE_TIME: f32 = 0.5;

pub struct Scraper {
    pub driver: Option<WebDriver>,
}

impl Scraper {
    pub fn new() -> Self {
        Self {
            driver: None,
        }
    }
    pub async fn initialize_driver(&self) -> WebDriverResult<WebDriver> {
        let chrome_options: fn() -> thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome;

        let _ = chrome_options().set_headless();
        let _ = chrome_options().add_chrome_arg("--log-level=3");
        let _ = chrome_options().add_chrome_arg("--blink-settings=imagesEnabled=false");
        let _ = chrome_options().add_chrome_arg("--disable-extensions");
        let _ = chrome_options().add_chrome_arg("--disable-dev-shm-usage");
        let _ = chrome_options().add_chrome_arg("--no-sandbox");

        let kill_chrome_driver: std::process::Output= Command::new("pkill")
        .arg("chromedriver")
        .output()
        .expect("failed to execute process");

        if kill_chrome_driver.status.success() {
            println!("Successfully killed all chromedriver processes");
        } else {
            eprintln!("Error: failed to kill chromedriver processes");
        }
        let output: std::process::Output = Command::new("./chromedriver")
            .output()
            .expect("failed to execute process");
        if output.status.success() {
            println!("chromedriver was executed successfully");
        } else {
            eprintln!("Error: chromedriver failed to execute");
        }

        println!("chromedriver was executed successfully");
        let driver: WebDriver = WebDriver::new("http://localhost:9515", chrome_options()).await?;
        info!("WebDriver initialized");
        Ok(driver)
    }
    pub async fn login(&mut self, username: &str, password: &str) -> WebDriverResult<bool> {
        if self.driver.is_none() {
            self.driver = Some(self.initialize_driver().await?);
        }

        let driver = self.driver.as_ref().unwrap();
        driver
            .goto("https://www.workatastartup.com/companies")
            .await?;

        let login_button = driver.query(By::XPath(LOGIN_BUTTON_XPATH)).first().await?;
        login_button.wait_until().displayed().await?;
        login_button.click().await?;

        let username_input = driver
            .query(By::XPath(USERNAME_INPUT_XPATH))
            .first()
            .await?;
        username_input.wait_until().displayed().await?;
        let password_input = driver
            .query(By::XPath(PASSWORD_INPUT_XPATH))
            .first()
            .await?;
        password_input.wait_until().displayed().await?;
        let submit_button = driver.query(By::XPath(SUBMIT_BUTTON_XPATH)).first().await?;
        submit_button.wait_until().displayed().await?;

        username_input.send_keys(username).await?;
        password_input.send_keys(password).await?;
        login_button.click().await?;
        info!("Successfully logged in!");
        Ok(true)
    }
}

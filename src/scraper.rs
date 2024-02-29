// use crate::utils;
use log::info;
use std::path::Path;
use thirtyfour::prelude::*;
use std::process::Command;
use ycombinator_scraper_rust::selectors{
    LOGIN_BUTTON_XPATH, PASSWORD_INPUT_XPATH, SUBMIT_BUTTON_XPATH, USERNAME_INPUT_XPATH,
};
use ycombinator_scraper_rust::config::config::Config;


const MAX_WORKERS: i32 = 5;
const SCROLL_PAUSE_TIME: f32 = 0.5;

pub struct Scraper {
    pub driver: Option<WebDriver>,
    // script_directory: Path,
}

impl Scraper {
    fn new() -> Self {
        let script_directory: std::path::PathBuf = Path::new(file!()).parent().unwrap().to_owned();
        Self {
            driver: None,
            // script_directory,
        }
    }
    #[tokio::main]
    pub async fn initialize_driver(&self) -> WebDriverResult<WebDriver> {
        
        let chrome_options = DesiredCapabilities::chrome;
        
        chrome_options().set_headless();
        chrome_options().add_chrome_arg("--log-level=3");
        chrome_options().add_chrome_arg("--blink-settings=imagesEnabled=false");
        chrome_options().add_chrome_arg("--disable-extensions");
        chrome_options().add_chrome_arg("--disable-dev-shm-usage");
        chrome_options().add_chrome_arg("--no-sandbox");
        let output = Command::new("./chromedriver")
        .output()
        .expect("failed to execute process");


        println!("chromedriver was executed successfully");
        let driver = WebDriver::new("http://localhost:9515", chrome_options()).await?;
        info!("WebDriver initialized");
        Ok(driver)
    }
    #[tokio::main]
    pub async fn login(&mut self, username: &str, password: &str) -> WebDriverResult<bool> {
        if self.driver.is_none() {
            self.driver = Some(self.initialize_driver()?);
        }

        let driver = self.driver.as_ref().unwrap();
        driver.goto("https://www.workatastartup.com/companies").await?;

        let login_button =
            driver.query(By::XPath(LOGIN_BUTTON_XPATH)).first().await?;
            login_button.wait_until().displayed().await?;
        login_button.click().await?;

        let username_input =
            driver.query(By::XPath(USERNAME_INPUT_XPATH)).first().await?;
            username_input.wait_until().displayed().await?;
        let password_input =
            driver.query(By::XPath(PASSWORD_INPUT_XPATH)).first().await?;
            password_input.wait_until().displayed().await?;
        let submit_button =
            driver.query(By::XPath(SUBMIT_BUTTON_XPATH)).first().await?;
            submit_button.wait_until().displayed().await?;
            
        let config: Config = Config::from_env();
        let username: String = config.login_username;
        let password: String = config.login_password;
        username_input.send_keys(username).await?;
        password_input.send_keys(password).await?;
        login_button.click().await?;
        info!("Successfully logged in!");
        Ok(true)
    }
}

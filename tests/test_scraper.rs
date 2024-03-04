#[cfg(test)]
mod test_scraper {
    use thirtyfour::prelude::*;
    use workatastartup_scraper::config;
    use workatastartup_scraper::scraper::{
        create_chrome_driver, find_elements_by_class, login, scrape_company_data,
        scrape_founders_data, scrape_job_data,
    };

    // Initialize WebDriver as a test fixture
    async fn init_driver(port: i32) -> WebDriver {
        create_chrome_driver(port)
            .await
            .expect("Failed to create driver")
    }

    #[tokio::test]

    async fn test_find_elements_by_class() {
        let port: i32 = 44454; // Reuse the same port as other tests
        let driver: WebDriver = init_driver(port).await;
        let class_name: &str = "example-class";

        // Test with valid class name
        let result: Result<Vec<WebElement>, WebDriverError> =
            find_elements_by_class(driver.clone(), class_name).await;
        let _ = driver.quit().await;
        assert!(result.is_ok());
    }

    #[tokio::test]

    async fn test_create_chrome_driver() {
        // Test create_chrome_driver function
        let port: i32 = 44454; // Reuse the same port as other tests
        let result: Result<WebDriver, WebDriverError> = create_chrome_driver(port).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_login_successful() {
        let dets: config::Config = config::Config::from_env();
        let username: &str = dets.login_username.as_str();
        let password: &str  = dets.login_password.as_str();

        // Ensure that login is successful with valid credentials
        assert!(login(username, password).await.is_ok());
    }

    #[tokio::test]
    async fn test_scrape_founders_data_successful() {
        // Provide a valid company URL for testing
        let company_url: &str = "https://www.workatastartup.com/companies/superkalam";

        // Ensure that scraping founders data is successful with a valid URL
        assert!(scrape_founders_data(company_url).await.is_ok());
    }

    #[tokio::test]
    async fn test_scrape_job_data_successful() {
        // Provide a valid job URL for testing
        let job_url: &str = "https://www.workatastartup.com/jobs/64333";

        // Ensure that scraping job data is successful with a valid URL
        assert!(scrape_job_data(job_url).await.is_ok());
    }
    // #[tokio::test]
    // async fn test_scrape_company_data_successful() {
    //     // Provide a valid job URL for testing
    //     let company_url: &str = "https://www.workatastartup.com/companies/superkalam";

    //     // Ensure that scraping job data is successful with a valid URL
    //     assert!(scrape_company_data(company_url).await.is_ok());
    // }
}

#[cfg(test)]
mod test_scraper {
    use thirtyfour::prelude::*;
    use ycombinator_scraper_rust::scraper::{
        create_chrome_driver, find_element_by_class, find_elements_by_class,
        perform_action_on_element,
    };

    // Mock WebDriver for testing
    #[tokio::test]
    async fn test_perform_action_on_element() {
        let port: i32 = 4444;
        let driver: WebDriver = create_chrome_driver(port)
            .await
            .expect("Failed to create driver");
        let xpath: &str = "//button[@id='submit-btn']";
        let action: &str = "click";
        let value: Option<&str> = None;

        // Test with valid xpath and action
        let result: Result<WebElement, WebDriverError> =
            perform_action_on_element(driver.clone(), xpath, action, value).await;
        assert!(result.is_ok());

        // Test with invalid xpath
        let invalid_xpath: &str = "//button[@id='non-existing']";
        let result: Result<WebElement, WebDriverError> =
            perform_action_on_element(driver.clone(), invalid_xpath, action, value).await;
        assert!(result.is_err());
        driver.quit().await;
    }

    #[tokio::test]
    async fn test_find_element_by_class() {
        let port: i32 = 4445;
        let driver: WebDriver = create_chrome_driver(port)
            .await
            .expect("Failed to create driver");
        let class_name: &str = "example-class";

        // Test with valid class name
        let result: Result<WebElement, WebDriverError> =
            find_element_by_class(driver.clone(), class_name).await;
        assert!(result.is_ok());
        driver.quit().await;

    }

    #[tokio::test]
    async fn test_find_elements_by_class() {
        let port: i32 = 4446;
        let driver: WebDriver = create_chrome_driver(port)
            .await
            .expect("Failed to create driver");
        let class_name: &str = "example-class";

        // Test with valid class name
        let result: Result<Vec<WebElement>, WebDriverError> =
            find_elements_by_class(driver.clone(), class_name).await;
        assert!(result.is_ok());
        driver.quit().await;

    }

    #[tokio::test]
    async fn test_create_chrome_driver() {
        // Test create_chrome_driver function
        let port: i32 = 4447;
        let result: Result<WebDriver, WebDriverError> = create_chrome_driver(port).await;
        assert!(result.is_ok());
    }
}

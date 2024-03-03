#[cfg(test)]
mod test_scraper {
    use serial_test::serial;
    use thirtyfour::prelude::*;
    use workatastartup_scraper::scraper::{
        create_chrome_driver, find_element_by_class, find_elements_by_class,
        perform_action_on_element,
    };

    // Initialize WebDriver as a test fixture
    async fn init_driver(port: i32) -> WebDriver {
        create_chrome_driver(port)
            .await
            .expect("Failed to create driver")
    }

    #[tokio::main]
    #[test]
    #[serial]
    async fn test_perform_action_on_element() {
        let port: i32 = 44454;
        let driver: WebDriver = init_driver(port).await;
        let xpath: &str = r"//button[@id='submit-btn']";
        let action: &str = "click";
        let value: Option<&str> = None;

        // Test with valid xpath and action
        let result: Result<WebElement, WebDriverError> =
            perform_action_on_element(driver.clone(), xpath, action, value).await;
        // println!("{:?}", result);

        // assert!(result.is_ok());

        // Test with invalid xpath
        let invalid_xpath: &str = "//button[@id='non-existing']";
        let result: Result<WebElement, WebDriverError> =
            perform_action_on_element(driver.clone(), invalid_xpath, action, value).await;
        println!("{:?}", result);

        assert!(result.is_err());
    }

    #[tokio::main]
    #[test]
    #[serial]
    async fn test_find_element_by_class() {
        let port: i32 = 44454; // Reuse the same port as other tests
        let driver: WebDriver = init_driver(port).await;
        let class_name: &str = r"mb-1.font-medium";

        // Test with valid class name
        let result: Result<WebElement, WebDriverError> =
            find_element_by_class(driver.clone(), class_name).await;
        println!("{:?}", result);
        assert!(result.is_err());
    }

    #[tokio::main]
    #[test]
    #[serial]
    async fn test_find_elements_by_class() {
        let port: i32 = 44454; // Reuse the same port as other tests
        let driver: WebDriver = init_driver(port).await;
        let class_name: &str = "example-class";

        // Test with valid class name
        let result: Result<Vec<WebElement>, WebDriverError> =
            find_elements_by_class(driver.clone(), class_name).await;
        assert!(result.is_ok());
    }

    #[tokio::main]
    #[test]
    #[serial]
    async fn test_create_chrome_driver() {
        // Test create_chrome_driver function
        let port: i32 = 44454; // Reuse the same port as other tests
        let result: Result<WebDriver, WebDriverError> = create_chrome_driver(port).await;
        assert!(result.is_ok());
    }
}

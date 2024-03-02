#[cfg(test)]
mod test_utils {
    use ycombinator_scraper_rust::utils::{
        strip_html_tags, validate_company_url, validate_job_url,
    };

    #[test]
    fn test_valid_job_url() {
        let valid_url: &str = "https://www.workatastartup.com/jobs/example";
        assert!(validate_job_url(valid_url).is_ok(), "Expected valid URL");
    }

    #[test]
    fn test_invalid_job_url() {
        let invalid_url = "https://www.example.com";
        assert!(
            validate_job_url(invalid_url).is_err(),
            "Expected invalid URL"
        );
    }

    #[test]
    fn test_valid_company_url() {
        let valid_url: &str = "https://www.workatastartup.com/companies/example";
        assert!(
            validate_company_url(valid_url).is_ok(),
            "Expected valid URL"
        );
    }

    #[test]
    fn test_invalid_companies_url() {
        let invalid_url = "https://www.example.com";
        assert!(
            validate_company_url(invalid_url).is_err(),
            "Expected invalid URL"
        );
    }

    #[test]
    fn test_strip_html_tags() {
        let html_content: &str = "<p>This is <b>bold</b> text.</p>";
        let expected_output: &str = "This is bold text.";
        let stripped_text: String = strip_html_tags(html_content);
        assert_eq!(stripped_text, expected_output);
    }
}

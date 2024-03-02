mod test_config{
    use ycombinator_scraper_rust::config::Config;

#[test]
fn test_from_env() {
    // Set up the test environment
    std::env::set_var("LOGIN_USERNAME", "test_username");
    std::env::set_var("LOGIN_PASSWORD", "test_password");

    // Call the from_env function to retrieve the config
    let config = Config::from_env();

    // Verify that the retrieved config matches the expected values
    assert_eq!(config.login_username, "test_username");
    assert_eq!(config.login_password, "test_password");
}
}
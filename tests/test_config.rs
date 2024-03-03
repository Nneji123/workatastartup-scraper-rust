mod test_config {
    use workatastartup_scraper::config::Config;

    #[test]
    fn test_from_env() {
        std::env::set_var("LOGIN_USERNAME", "test_username");
        std::env::set_var("LOGIN_PASSWORD", "test_password");
        let config = Config::from_env();
        assert_eq!(config.login_username, "test_username");
        assert_eq!(config.login_password, "test_password");
    }
}

mod test_config {
    use workscraper_lib::config::Config;

    #[test]
    fn test_from_env() {
        std::env::set_var("LOGIN_USERNAME", "test_username");
        std::env::set_var("LOGIN_PASSWORD", "test_password");
        std::env::set_var("REDIS_URL", "redis://");
        std::env::set_var("DATABASE_URI", "mongodb://");

        let config: Config = Config::from_env();
        assert_eq!(config.login_username, "test_username");
        assert_eq!(config.login_password, "test_password");
        assert_eq!(config.redis_url, "redis://");
        assert_eq!(config.database_url, "mongodb://");
    
    }
}

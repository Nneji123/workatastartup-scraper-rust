
// Define a struct to hold your configuration

pub mod config {
    use dotenv::dotenv;
    use std::env;

    #[derive(Debug)]
    pub struct Config {
    pub login_username: String,
    pub login_password: String,
}

/// Function to load environment variables into the struct
impl Config {
    pub fn from_env() -> Self {
        dotenv().ok(); // Load environment variables from .env file

        let login_username =
            env::var("LOGIN_USERNAME").expect("LOGIN_USERNAME must be set in .env file");
        let login_password =
            env::var("LOGIN_PASSWORD").expect("LOGIN_PASSWORD must be set in .env file");

        Config {
            login_username,
            login_password,
        }
    }
}
}
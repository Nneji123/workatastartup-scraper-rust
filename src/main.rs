// mod selectors;
// mod utils;
// mod models;
// mod config;
// mod scraper;
// use webdriver_downloader::driver_impls::chromedriver_info::ChromedriverInfo;
// use webdriver_downloader::prelude::WebdriverDownloadInfo;

// fn main() {
//     // let url: &str = "https://www.workatastartup.com/jobs/example";
//     // match utils::validate_job_url(url) {
//     //     Ok(_) => println!("Valid URL"),
//     //     Err(e) => println!("{}", e),
//     // }

//     // // Example usage
//     // let html_content: &str = r#"<html><body><p>Hello, <b>world</b>!</p></body></html>"#;
//     // let text_content: String = utils::strip_html_tags(html_content);
//     // println!("{}", text_content); // Output: Hello, world!
//     // models::test_model();
//     // // Load environment variables into the struct
//     // let config = config::Config::from_env();

//     // // Print the loaded configuration
//     // println!("{:?}", config);

//     #[tokio::main]
//     async fn main() {
//         let driver_info = ChromedriverInfo::new_default().unwrap();

//         // Tries up to 5 versions of webdrivers if it is not installed.
//         if !driver_info.is_ {
//             driver_info.download_verify_install(5).await.unwrap();
//         }

//         // webdriver is installed.
//         // Default installation path is %USERPROFILE%/bin/chromedriver.exe ($HOME/bin/chromedriver for unix family)
//     }
// }

mod scraper;
fn main(){
    scraper::Scraper.
}

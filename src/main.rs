// mod selectors;
mod utils;
mod models;

fn main() {
    let url: &str = "https://www.workatastartup.com/jobs/example";
    match utils::validate_job_url(url) {
        Ok(_) => println!("Valid URL"),
        Err(e) => println!("{}", e),
    }

    // Example usage
    let html_content: &str = r#"<html><body><p>Hello, <b>world</b>!</p></body></html>"#;
    let text_content: String = utils::strip_html_tags(html_content);
    println!("{}", text_content); // Output: Hello, world!
    models::
}

mod test_models {
        use ycombinator_scraper_rust::models::
        FounderData;
// Import necessary modules
use serde_json;

    // Define a test function to serialize and deserialize structs
    #[test]
    fn test_serde() {
        // Create sample data
        let founder_data = FounderData {
            founder_name: "John Doe".to_string(),
            founder_description: "Co-founder & CEO".to_string(),
            founder_image_url: Some("http://example.com/image.jpg".to_string()),
            founder_linkedin_url: Some("http://linkedin.com/johndoe".to_string()),
            founder_emails: Some(vec!["john@example.com".to_string(), "doe@example.com".to_string()]),
        };

        // Serialize and then deserialize founder_data
        let serialized_founder_data = serde_json::to_string(&founder_data).unwrap();
        let deserialized_founder_data: FounderData = serde_json::from_str(&serialized_founder_data).unwrap();

        // Verify that serialization and deserialization worked correctly
        assert_eq!(founder_data.founder_name, deserialized_founder_data.founder_name);
        assert_eq!(founder_data.founder_description, deserialized_founder_data.founder_description);
        assert_eq!(founder_data.founder_image_url, deserialized_founder_data.founder_image_url);
        assert_eq!(founder_data.founder_linkedin_url, deserialized_founder_data.founder_linkedin_url);
        assert_eq!(founder_data.founder_emails, deserialized_founder_data.founder_emails);
    }
}
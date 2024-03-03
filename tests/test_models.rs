mod test_models {
    use serde_json;
    use workatastartup_scraper::models::{CompanyData, FounderData, JobData, ScrapedData};

    #[test]
    fn test_serde_founders() {
        let job_data = FounderData {
            founder_name: "John Doe".to_string(),
            founder_description: "Co-founder & CEO".to_string(),
            founder_image_url: Some("http://example.com/image.jpg".to_string()),
            founder_linkedin_url: Some("http://linkedin.com/johndoe".to_string()),
            founder_emails: Some(vec![
                "john@example.com".to_string(),
                "doe@example.com".to_string(),
            ]),
        };

        let serialized_job_data = serde_json::to_string(&job_data).unwrap();
        let deserialized_job_data: FounderData =
            serde_json::from_str(&serialized_job_data).unwrap();

        assert_eq!(job_data.founder_name, deserialized_job_data.founder_name);
        assert_eq!(
            job_data.founder_description,
            deserialized_job_data.founder_description
        );
        assert_eq!(
            job_data.founder_image_url,
            deserialized_job_data.founder_image_url
        );
        assert_eq!(
            job_data.founder_linkedin_url,
            deserialized_job_data.founder_linkedin_url
        );
        assert_eq!(
            job_data.founder_emails,
            deserialized_job_data.founder_emails
        );
    }
    #[test]

    fn test_serde_jobs() {
        // Create sample data
        let job_data: JobData = JobData {
            job_url: "https://workatastartup.com/jobs/12313".to_string(),
            job_title: "ML Engineer".to_string(),
            job_salary_range: "$220k - $300k".to_string(),
            job_tags: vec!["Engineering".to_string(), "Data Science".to_string()],
            job_description: "Skilled Ml engineer specializing in LLM's".to_string(),
        };

        let serialized_job_data: String = serde_json::to_string(&job_data).unwrap();
        let deserialized_job_data: JobData = serde_json::from_str(&serialized_job_data).unwrap();

        assert_eq!(job_data.job_title, deserialized_job_data.job_title);
        assert_eq!(
            job_data.job_description,
            deserialized_job_data.job_description
        );
        assert_eq!(job_data.job_url, deserialized_job_data.job_url);
        assert_eq!(job_data.job_tags, deserialized_job_data.job_tags);
        assert_eq!(
            job_data.job_salary_range,
            deserialized_job_data.job_salary_range
        );
    }
    // Define a test function for serialization and deserialization
    #[test]
    fn test_serde_company_data() {
        // Create sample CompanyData
        let company_data: CompanyData = CompanyData {
            company_name: "Example Inc.".to_string(),
            company_url: "http://example.com".to_string(),
            company_description: "A sample company".to_string(),
            company_tags: vec!["tag1".to_string(), "tag2".to_string()],
            company_image: "http://example.com/image.jpg".to_string(),
            company_social_links: vec![
                "http://facebook.com/example".to_string(),
                "http://twitter.com/example".to_string(),
            ],
            company_job_links: vec![
                "http://example.com/job1".to_string(),
                "http://example.com/job2".to_string(),
            ],
            company_founders: vec![],
            job_data: vec![JobData::default()],
        };

        let serialized_company_data: String = serde_json::to_string(&company_data).unwrap();
        let deserialized_company_data: CompanyData =
            serde_json::from_str(&serialized_company_data).unwrap();

        assert_eq!(
            company_data.company_name,
            deserialized_company_data.company_name
        );
        assert_eq!(
            company_data.company_url,
            deserialized_company_data.company_url
        );
        assert_eq!(
            company_data.company_description,
            deserialized_company_data.company_description
        );
        assert_eq!(
            company_data.company_tags,
            deserialized_company_data.company_tags
        );
        assert_eq!(
            company_data.company_image,
            deserialized_company_data.company_image
        );
        assert_eq!(
            company_data.company_social_links,
            deserialized_company_data.company_social_links
        );
        assert_eq!(
            company_data.company_job_links,
            deserialized_company_data.company_job_links
        );
        assert_eq!(
            company_data.job_data.len(),
            deserialized_company_data.job_data.len()
        );
        for (expected, actual) in company_data
            .job_data
            .iter()
            .zip(deserialized_company_data.job_data.iter())
        {
            assert_eq!(expected.job_url, actual.job_url);
            assert_eq!(expected.job_title, actual.job_title);
            assert_eq!(expected.job_salary_range, actual.job_salary_range);
            assert_eq!(expected.job_tags, actual.job_tags);
            assert_eq!(expected.job_description, actual.job_description);
        }
    }

    #[test]
    fn test_serde_scraped_data() {
        // Create sample ScrapedData
        let scraped_data = ScrapedData {
            scraped_data: vec![CompanyData::default()],
        };

        let serialized_scraped_data = serde_json::to_string(&scraped_data).unwrap();
        let deserialized_scraped_data: ScrapedData =
            serde_json::from_str(&serialized_scraped_data).unwrap();

        assert_eq!(
            scraped_data.scraped_data.len(),
            deserialized_scraped_data.scraped_data.len()
        );
        for (expected, actual) in scraped_data
            .scraped_data
            .iter()
            .zip(deserialized_scraped_data.scraped_data.iter())
        {
            assert_eq!(expected.company_name, actual.company_name);
            assert_eq!(expected.company_url, actual.company_url);
            assert_eq!(expected.company_description, actual.company_description);
            assert_eq!(expected.company_tags, actual.company_tags);
            assert_eq!(expected.company_image, actual.company_image);
            assert_eq!(expected.company_social_links, actual.company_social_links);
            assert_eq!(expected.company_job_links, actual.company_job_links);

            // Compare job_data vectors element-wise
            assert_eq!(expected.job_data.len(), actual.job_data.len());
            for (expected_job, actual_job) in expected.job_data.iter().zip(actual.job_data.iter()) {
                assert_eq!(expected_job.job_url, actual_job.job_url);
                assert_eq!(expected_job.job_title, actual_job.job_title);
                assert_eq!(expected_job.job_salary_range, actual_job.job_salary_range);
                assert_eq!(expected_job.job_tags, actual_job.job_tags);
                assert_eq!(expected_job.job_description, actual_job.job_description);
            }
        }
    }
}

#[path = "../src/database.rs"]
mod database;

mod test_crud{
    use mongodb::{
        bson::{doc, Document},
        error::Error,
        Client,
    };
    use serde::{Deserialize, Serialize};
    use workscraper_lib::models::{CompanyData, FounderData, JobData, ScrapedData};
    use tokio;
    use crate::database;
    #[tokio::test]
    async fn test_insert_job_data() -> Result<(), Error> {
        let database_name = "test_database";
        let collection_name = "jobs";
    
        let client: Client = Client::with_uri_str("mongodb://localhost:27017").await?;
        let database: mongodb::Database = client.database(database_name);
    
        let job_data: JobData = JobData {
            job_url: "https://example.com/job".to_string(),
            job_title: "Software Engineer".to_string(),
            job_salary_range: "$80,000 - $100,000".to_string(),
            job_tags: vec!["Software Engineering".to_string(), "Rust".to_string()],
            job_description: "Description of the job".to_string(),
        };
    
        database::insert_job_data(&database, &job_data, collection_name).await?;
    
        let filter: Document = doc! { "job_title": "Software Engineer" };
        let result: Option<Document> = database.collection::<Document>(collection_name).find_one(filter, None).await?;
    
        assert!(result.is_some());
    
        Ok(())
    }
    
    #[tokio::test]
    async fn test_update_job_data() -> Result<(), Error> {
        let database_name = "test_database";
        let collection_name = "jobs";
    
        let client = Client::with_uri_str("mongodb://localhost:27017").await?;
        let database = client.database(database_name);
    
        let job_data: JobData = JobData {
            job_url: "https://example.com/job".to_string(),
            job_title: "Software Engineer".to_string(),
            job_salary_range: "$80,000 - $100,000".to_string(),
            job_tags: vec!["Software Engineering".to_string(), "Rust".to_string()],
            job_description: "Description of the job".to_string(),
        };
    
        database::insert_job_data(&database, &job_data, collection_name).await?;
    
        let updated_job_data: JobData = JobData {
            job_url: "https://example.com/job".to_string(),
            job_title: "Senior Software Engineer".to_string(),
            job_salary_range: "$100,000 - $120,000".to_string(),
            job_tags: vec!["Software Engineering".to_string(), "Rust".to_string()],
            job_description: "Updated description of the job".to_string(),
        };
    
        database::update_job_data(&database, "Software Engineer", &updated_job_data).await?;
    
        let filter: Document = doc! { "job_title": "Senior Software Engineer" };
        let result: Option<Document> = database.collection::<Document>(collection_name).find_one(filter, None).await?;
    
        assert!(result.is_some());
    
        Ok(())
    }
    

    // Mock CompanyData for testing
    fn mock_company_data() -> CompanyData {
        CompanyData {
            company_name: "Example Company".to_string(),
            company_url: "https://example.com".to_string(),
            company_description: "Description of Example Company".to_string(),
            company_tags: vec!["Technology".to_string(), "Startup".to_string()],
            company_image: "https://example.com/logo.png".to_string(),
            company_social_links: vec!["https://twitter.com/example".to_string(), "https://linkedin.com/company/example".to_string()],
            company_job_links: vec!["https://example.com/jobs".to_string()],
            company_founders: vec![
                FounderData {
                    founder_name: "John Doe".to_string(),
                    founder_description: "Co-founder and CEO".to_string(),
                    founder_image_url: Some("https://example.com/john_doe.jpg".to_string()),
                    founder_linkedin_url: Some("https://linkedin.com/in/johndoe".to_string()),
                    founder_emails: Some(vec!["john.doe@example.com".to_string()]),
                }
            ],
            job_data: vec![
                JobData {
                    job_url: "https://example.com/job".to_string(),
                    job_title: "Software Engineer".to_string(),
                    job_salary_range: "$80,000 - $100,000".to_string(),
                    job_tags: vec!["Software Engineering".to_string(), "Rust".to_string()],
                    job_description: "Description of the job".to_string(),
                }
            ],
        }
    }

    #[tokio::test]
    async fn test_insert_company_data() {
        let database_name = "test_database";
        let collection_name = "companies";

        // Connect to the MongoDB database
        let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
        let database = client.database(database_name);

        // Generate mock company data
        let company_data = mock_company_data();

        // Call the insert_company_data function
        database::insert_company_data(&database, &company_data).await.unwrap();

        // Check if the company data exists in the collection
        let filter = doc! { "company_name": "Example Company" };
        let result = database.collection::<Document>(collection_name)
            .find_one(filter, None)
            .await
            .unwrap();

        assert!(result.is_some());
    }
    #[tokio::test]
    async fn test_update_company_data() -> Result<(), Error> {
        let database_name: &str = "test_database";
        let collection_name: &str = "companies";

        // Connect to the MongoDB database
        let client = Client::with_uri_str("mongodb://localhost:27017").await?;
        let database = client.database(database_name);

        // Generate mock company data
        let company_data = mock_company_data();

        // Insert the mock company data
        database::insert_company_data(&database, &company_data).await?;

        // Generate updated company data
        let updated_company_data = CompanyData {
            // Update fields as needed
            company_name: company_data.company_name.clone(),
            company_url: company_data.company_url.clone(),
            company_description: "Updated description".to_string(),
            company_tags: company_data.company_tags.clone(),
            company_image: company_data.company_image.clone(),
            company_social_links: company_data.company_social_links.clone(),
            company_job_links: company_data.company_job_links.clone(),
            company_founders: company_data.company_founders.clone(),
            job_data: company_data.job_data.clone(),
        };

        // Call the update_company_data function
        database::update_company_data(&database, &company_data.company_name, &updated_company_data)
            .await?;

        // Read the updated company data
        let filter = doc! { "company_name": &company_data.company_name };
        let result = database::read_company_data(&database, &company_data.company_name).await?;

        // Check if the read company data matches the inserted data
        let expected_company_data = Some(company_data);
        assert_eq!(result, expected_company_data);

        Ok(())
    }

    #[tokio::test]
    async fn test_read_company_data() -> Result<(), Error> {
        let database_name = "test_database";
        let collection_name = "companies";

        // Connect to the MongoDB database
        let client = Client::with_uri_str("mongodb://localhost:27017").await?;
        let database = client.database(database_name);

        // Generate mock company data
        let company_data = mock_company_data();

        // Insert the mock company data
        database::insert_company_data(&database, &company_data).await?;

        // Call the read_company_data function
        let result = database::read_company_data(&database, &company_data.company_name).await?;

        // Check if the read company data matches the inserted data
        let expected_company_data = Some(company_data);
        assert_eq!(result, expected_company_data);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_company_data() -> Result<(), Error> {
        let database_name = "test_database";
        let collection_name = "companies";

        // Connect to the MongoDB database
        let client = Client::with_uri_str("mongodb://localhost:27017").await?;
        let database = client.database(database_name);

        // Generate mock company data
        let company_data = mock_company_data();

        // Insert the mock company data
        database::insert_company_data(&database, &company_data).await?;

        // Call the delete_company_data function
        database::delete_company_data(&database, &company_data.company_name).await?;

        // Check if the company data was deleted from the collection
        let filter = doc! { "company_name": &company_data.company_name };
        let result: Option<Document> = database.collection(collection_name).find_one(filter, None).await?;

        assert_eq!(result, None);

        Ok(())
    }
}


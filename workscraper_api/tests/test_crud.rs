mod test_crud{
    use mongodb::{
        bson::{doc, Document},
        error::Error,
        Client,
    };
    use serde::{Deserialize, Serialize};
    use workscraper_lib::models::{CompanyData, FounderData, JobData, ScrapedData};
    use tokio;
    use workscraper_api::database;
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
    
        let filter = doc! { "job_title": "Software Engineer" };
        let result = database.collection::<Document>(collection_name).find_one(filter, None).await?;
    
        assert!(result.is_some());
    
        Ok(())
    }
    
    #[tokio::test]
    async fn test_update_job_data() -> Result<(), Error> {
        let database_name = "test_database";
        let collection_name = "jobs";
    
        let client = Client::with_uri_str("mongodb://localhost:27017").await?;
        let database = client.database(database_name);
    
        let job_data = JobData {
            job_url: "https://example.com/job".to_string(),
            job_title: "Software Engineer".to_string(),
            job_salary_range: "$80,000 - $100,000".to_string(),
            job_tags: vec!["Software Engineering".to_string(), "Rust".to_string()],
            job_description: "Description of the job".to_string(),
        };
    
        database::insert_job_data(&database, &job_data, collection_name).await?;
    
        let updated_job_data = JobData {
            job_url: "https://example.com/job".to_string(),
            job_title: "Senior Software Engineer".to_string(),
            job_salary_range: "$100,000 - $120,000".to_string(),
            job_tags: vec!["Software Engineering".to_string(), "Rust".to_string()],
            job_description: "Updated description of the job".to_string(),
        };
    
        database::update_job_data(&database, "Software Engineer", &updated_job_data).await?;
    
        let filter = doc! { "job_title": "Senior Software Engineer" };
        let result = database.collection::<Document>(collection_name).find_one(filter, None).await?;
    
        assert!(result.is_some());
    
        Ok(())
    }
    

}


use mongodb::{bson::{doc, Document}, error::Error, results::UpdateResult, Client, Database};
use serde::{Deserialize, Serialize};
use mongodb::bson;
use workscraper_lib::models::{CompanyData, FounderData, JobData, ScrapedData};

// Define the data structures as provided earlier

pub async fn insert_company_data(database: &Database, company_data: &CompanyData) -> Result<(), Error> {
    let collection: mongodb::Collection<Document> = database.collection::<Document>("companies");

    let company_doc: Document = bson::to_document(company_data)?;

    collection.insert_one(company_doc, None).await?;

    Ok(())
}

pub async fn update_company_data(database: &Database, company_name: &str, updated_company_data: &CompanyData) -> Result<UpdateResult, Error> {
    let collection: mongodb::Collection<Document> = database.collection::<Document>("companies");

    let filter: Document = doc! { "company_name": company_name };
    let updated_doc: Document = bson::to_document(updated_company_data)?;

    collection.update_one(filter, updated_doc, None).await
}

pub async fn read_company_data(database: &Database, company_name: &str) -> Result<Option<CompanyData>, Error> {
    let collection: mongodb::Collection<Document> = database.collection::<Document>("companies");

    let filter: Document = doc! { "company_name": company_name };

    let result: Option<Document> = collection.find_one(filter, None).await?;

    match result {
        Some(doc) => {
            let company_data: CompanyData = bson::from_document(doc)?;
            Ok(Some(company_data))
        }
        None => Ok(None),
    }
}

pub async fn delete_company_data(database: &Database, company_name: &str) -> Result<(), Error> {
    let collection: mongodb::Collection<Document> = database.collection::<Document>("companies");

    let filter: Document = doc! { "company_name": company_name };

    collection.delete_one(filter, None).await?;

    Ok(())
}

pub async fn insert_founder_data(database: &Database, founder_data: &FounderData, company_name: &str) -> Result<(), Error> {
    let collection: mongodb::Collection<Document> = database.collection::<Document>("founders");

    let mut founder_doc: Document = bson::to_document(founder_data)?;
    founder_doc.insert("company_name", company_name);

    collection.insert_one(founder_doc, None).await?;

    Ok(())
}

pub async fn update_founder_data(database: &Database, founder_name: &str, updated_founder_data: &FounderData) -> Result<UpdateResult, Error> {
    let collection: mongodb::Collection<Document> = database.collection::<Document>("founders");

    let filter: Document = doc! { "founder_name": founder_name };
    let updated_doc: Document = bson::to_document(updated_founder_data)?;

    collection.update_one(filter, updated_doc, None).await
}

pub async fn read_founder_data(database: &Database, founder_name: &str) -> Result<Option<FounderData>, Error> {
    let collection: mongodb::Collection<Document> = database.collection::<Document>("founders");

    let filter: Document = doc! { "founder_name": founder_name };

    let result: Option<Document> = collection.find_one(filter, None).await?;

    match result {
        Some(doc) => {
            let founder_data: FounderData = bson::from_document(doc)?;
            Ok(Some(founder_data))
        }
        None => Ok(None),
    }
}

pub async fn delete_founder_data(database: &Database, founder_name: &str) -> Result<(), Error> {
    let collection: mongodb::Collection<Document> = database.collection::<Document>("founders");

    let filter: Document = doc! { "founder_name": founder_name };

    collection.delete_one(filter, None).await?;

    Ok(())
}

pub async fn insert_job_data(database: &Database, job_data: &JobData, company_name: &str) -> Result<(), Error> {
    let collection: mongodb::Collection<Document> = database.collection::<Document>("jobs");

    let mut job_doc: Document = bson::to_document(job_data)?;
    job_doc.insert("company_name", company_name);

    collection.insert_one(job_doc, None).await?;

    Ok(())
}

pub async fn update_job_data(database: &Database, job_title: &str, updated_job_data: &JobData) -> Result<UpdateResult, Error> {
    let collection: mongodb::Collection<Document> = database.collection::<Document>("jobs");

    let filter: Document = doc! { "job_title": job_title };
    let updated_doc: Document = bson::to_document(updated_job_data)?;

    collection.update_one(filter, updated_doc, None).await
}

pub async fn read_job_data(database: &Database, job_title: &str) -> Result<Option<JobData>, Error> {
    let collection: mongodb::Collection<Document> = database.collection::<Document>("jobs");

    let filter: Document = doc! { "job_title": job_title };

    let result: Option<Document> = collection.find_one(filter, None).await?;

    match result {
        Some(doc) => {
            let job_data: JobData = bson::from_document(doc)?;
            Ok(Some(job_data))
        }
        None => Ok(None),
    }
}

pub async fn delete_job_data(database: &Database, job_title: &str) -> Result<(), Error> {
    let collection: mongodb::Collection<Document> = database.collection::<Document>("jobs");

    let filter: Document = doc! { "job_title": job_title };

    collection.delete_one(filter, None).await?;

    Ok(())
}
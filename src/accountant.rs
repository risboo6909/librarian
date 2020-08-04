use crate::crawler::Err;
use crate::model::Document;
use surf;

pub(crate) const URL: &'static str = "http://meilisearch.api";

//Accountant works with documents
pub(crate) struct Accountant {
    /*
Useful parameters for sending/searching documents to meilisearch will be stored here
*/}

impl Accountant {
    pub(crate) async fn send(docs: &Vec<Document>) -> Result<String, Err> {
        let mut response = surf::post(format!("{}/indexes/libraries/documents", URL))
            .body_json(docs)?
            .await?;
        response.body_string().await
    }
}

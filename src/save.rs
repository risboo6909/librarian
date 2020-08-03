
use crate::model::Document;
use surf;
use crate::crawler::Err;

pub(crate) async fn send(docs: &Vec<Document>) -> Result<String, Err> {
    let url = "http://127.0.0.1:7700/indexes/libraries/documents";
    let mut response = surf::post(url).body_json(docs)?.await?;
    Ok(response.body_string().await?)
}
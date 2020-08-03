use crate::crawler::Err;
use crate::model::Document;
use surf;

pub(crate) async fn send(url: &str, docs: &Vec<Document>) -> Result<String, Err> {
    let mut response = surf::post(format!("{}/indexes/libraries/documents", url))
        .body_json(docs)?
        .await?;
    response.body_string().await
}

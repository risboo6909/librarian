use crate::crawler::Err;
use crate::model::Document;
use serde::{Deserialize, Serialize};
use serde_json;

// Meilisearch api for testing
pub(crate) const URL: &str = "http://127.0.0.1:7700";

#[derive(Deserialize, Serialize)]
// SearchRequest holds parameters for searching
// q - mandatory, others - optional
// filters attribute can hold many fileds and many conditions to filter at once, for example:
//          "filters": "release_date > 1590537600" 05/27/2020 @ 12:00am (UTC)
//          "filters": "target_language = 'Rust' AND usage = 'game dev'"
pub(crate) struct SearchRequest<'a> {
    q: &'a str,
    filters: Option<&'a str>,
    offset: Option<u32>,
    limit: Option<u32>,
}
#[derive(Deserialize, Serialize)]
pub(crate) struct SearchResponse {
    hits: Vec<Document>,
}

//Accountant works with documents
pub(crate) struct Accountant {
    /*
Useful parameters for sending/searching documents to meilisearch will be stored here
*/}

impl Accountant {
    pub(crate) fn new() -> Self {
        Accountant {}
    }

    pub(crate) async fn search(self, req: SearchRequest<'_>) -> Result<Vec<Document>, Err> {
        let response = surf::post(format!("{}/indexes/libraries/search", URL))
            .body_json(&req)?
            .await?
            .body_string()
            .await?;
        let SearchResponse { hits } = serde_json::from_str(response.as_str())?;
        Ok(hits)
    }

    pub(crate) async fn send(self, docs: &Vec<Document>) -> Result<String, Err> {
        let mut response = surf::post(format!("{}/indexes/libraries/documents", URL))
            .body_json(docs)?
            .await?;
        response.body_string().await
    }
}

impl<'a> SearchRequest<'a> {
    pub(crate) fn new(q: &'a str) -> Self {
        SearchRequest {
            q,
            filters: None,
            offset: None,
            limit: None,
        }
    }
    pub(crate) fn filter_by(mut self, filters: &'a str) -> Self {
        self.filters = Some(filters);
        self
    }

    pub(crate) fn set_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
    pub(crate) fn set_offset(mut self, offfset: u32) -> Self {
        self.offset = Some(offfset);
        self
    }
}

impl SearchResponse {
    fn new() -> Self {
        SearchResponse { hits: vec![] }
    }
}

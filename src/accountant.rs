use crate::helpers::surf2anyhow;
use crate::model::Document;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MeiliIndexes {
    uid: String,
    primary_key: String,
    created_at: String,
    updated_at: String,
}

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

// Accountant works with documents
pub(crate) struct Accountant {
    // Useful parameters for sending/searching documents to meilisearch will be stored here
    meili_url: String,
}

impl Accountant {
    pub(crate) fn new() -> Self {
        Accountant {
            meili_url: format!(
                "{}:{}",
                super::CONF.read().unwrap().get_str("meili_host").unwrap(),
                super::CONF.read().unwrap().get_int("meili_port").unwrap(),
            ),
        }
    }

    pub(crate) async fn get_indexes(&self) -> anyhow::Result<Vec<MeiliIndexes>> {
        let response: Vec<MeiliIndexes> = surf2anyhow(
            surf::get(format!("{}/indexes", self.meili_url))
                .recv_json()
                .await,
        )?;
        Ok(response)
    }

    pub(crate) async fn is_index_exists(&self, index_name: &str) -> anyhow::Result<bool> {
        let indexes = self.get_indexes().await?;
        for index in indexes {
            if index.uid == index_name {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(crate) async fn create_index(&self, index_name: &str) {
        todo!();
    }

    // To trigger search you need to construct a search request
    // It is made by creating new SearchRequest object:
    //  let request =  SearchRequest::new("graphics");
    // As query string for search is mandatory, you have to provide it while creating request
    // Then, you can add filters for search result. Filters are optional.
    // It can be done like this:
    // let request =  SearchRequest::new("graphics").filter_by("target_language = Rust");
    // You can filter any document filed you want as well as any logic is accepted:
    // let request =  SearchRequest::new("graphics").filter_by("release_date > 3125890 OR usage = 'modeling'");
    // let request =  SearchRequest::new("graphics").filter_by("target_language = 'All' AND usage = 'AI'");
    // You can also set an offset with SearchRequest::new("graphics").set_offset(10) to tell the engine how many documents to skip
    // or set limit by SearchRequest::new("graphics").set_limit(100) to set a constraint for the quantity of documents listed
    pub(crate) async fn search(&self, req: SearchRequest<'_>) -> anyhow::Result<Vec<Document>> {
        let mut req = surf2anyhow(
            surf::post(format!("{}/indexes/libraries/search", self.meili_url))
                .body_json(&req)?
                .await,
        )?;

        let resp = surf2anyhow(req.body_string().await)?;
        let SearchResponse { hits } = serde_json::from_str(resp.as_str())?;

        Ok(hits)
    }

    pub(crate) async fn send(&self, docs: &Vec<Document>) -> anyhow::Result<String> {
        let mut req = surf2anyhow(
            surf::post(format!("{}/indexes/libraries/documents", self.meili_url))
                .body_json(docs)?
                .await,
        )?;

        surf2anyhow(req.body_string().await)
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

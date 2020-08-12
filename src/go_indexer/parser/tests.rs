use super::{fetch, parse};
use crate::accountant;
use crate::crawler::Err;
use crate::model::Document;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use chrono::prelude::*;
use http::Uri;
use std::hash::Hash;
use surf;

#[test]
fn test_parse() {
    let contents = include_str!("go.mod");

    let parsed: HashMap<String, HashSet<Uri>> = parse(contents);

    let mut expected = HashMap::new();

    expected.insert(
        String::from("github.com/360EntSecGroup-Skylar/excelize"),
        HashSet::from_iter(vec!["https://github.com/360EntSecGroup-Skylar/excelize"
            .parse::<Uri>()
            .unwrap()]),
    );

    expected.insert(
        String::from("github.com/spf13/pflag"),
        HashSet::from_iter(vec!["https://github.com/spf13/pflag"
            .parse::<Uri>()
            .unwrap()]),
    );

    expected.insert(
        String::from("github.com/go-chi/chi"),
        HashSet::from_iter(vec!["https://github.com/go-chi/chi"
            .parse::<Uri>()
            .unwrap()]),
    );

    assert_eq!(expected, parsed);
}

#[async_std::test]
async fn test_fetcher() {
    let links = fetch("https://awesome-go.com").await;
    assert!(links.is_ok());
}

#[async_std::test]
// TODO: enable this test when meilisearch mocks will be ready
#[ignore]
async fn send() {
    let lib = Document::new(1)
        .name("librarian")
        .description("best lib ever")
        .link("http://")
        .target_language("All")
        .last_commit(
            Utc.datetime_from_str("2020-07-24 12:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .timestamp(),
        )
        .last_release(
            Utc.datetime_from_str("2014-07-19 12:30:00", "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .timestamp(),
        )
        .license("MIT")
        .usage("web search");
    let response =
        surf::post("http://127.0.0.1:7700/indexes/libraries/documents").body_json(&vec![lib]);
    match response {
        Ok(req) => {
            let r = req.await;
            match r {
                Ok(resp) => assert_eq!(resp.status(), 202),
                Err(err) => panic!("unexpected, {}", err),
            }
        }
        Err(error) => panic!("error occurred, {}", error),
    }
}

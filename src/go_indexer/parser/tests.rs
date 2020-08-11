use super::{fetch, parse};
use crate::accountant;
use crate::crawler::Err;
use crate::model::Document;

use std::collections::HashSet;
use std::iter::FromIterator;

use chrono::prelude::*;
use http::Uri;
use std::hash::Hash;
use surf;

#[test]
fn test_parse() {
    let contents = include_str!("go.mod");

    let parsed: HashSet<Uri> = HashSet::from_iter(parse(contents));
    let compare: HashSet<Uri> = vec![
        "https://github.com/360EntSecGroup-Skylar/excelize",
        "https://github.com/go-chi/chi",      //v4.0.2+incompatible
        "https://github.com/go-chi/cors",     //v1.0.0
        "https://github.com/go-openapi/spec", //v0.19.3
        "https://github.com/golang/protobuf", //v1.4.2
        "https://github.com/grpc-ecosystem/grpc-gateway", //v1.14.2
        "https://github.com/lib/pq",          //v1.3.0  indirect
        "https://github.com/pkg/errors",      //v0.8.1
        "https://github.com/spf13/pflag",     //v1.0.5
        "https://github.com/spf13/viper",     //v1.6.1
        "https://github.com/stretchr/testify", //v1.4.0
        "https://github.com/utrack/clay/v2",  //v2.4.7
        "https://gitlab.ru/internal-projects/staff-api", //v0.0.0-20200619100014-b44a8677723f
        "https://gitlab.ru/platform/database-go", //v0.15.2
        "https://gitlab.ru/platform/errors",  //v1.3.6
        "https://gitlab.ru/platform/realtime-config-go", //v1.8.7
        "https://gitlab.ru/platform/scratch", //v1.6.8
        "https://gitlab.ru/platform/tracer-go", //v1.16.0
    ]
    .iter()
    .map(|uri| uri.parse::<Uri>().unwrap())
    .collect();
    assert_eq!(parsed, compare);
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

#[test]
fn test_link_extractor() {
    // º and ª symbols not covered
    let input = "github.com/bodagovsky\\
        github.com/bodagovsky!
        github.com/bodagovsky,
        github.com/bodagovsky*
        github.com/bodagovsky¡
        github.com/bodagovsky@
        github.com/bodagovsky™
        github.com/bodagovsky#
        github.com/bodagovsky£
        github.com/bodagovsky$
        github.com/bodagovsky¢
        github.com/bodagovsky%
        github.com/bodagovsky∞
        github.com/bodagovsky^
        github.com/bodagovsky§
        github.com/bodagovsky&
        github.com/bodagovsky¶
        github.com/bodagovsky•
        github.com/bodagovsky(
        github.com/bodagovsky)
        github.com/bodagovsky>
        github.com/bodagovsky≥
        github.com/bodagovsky<
        github.com/bodagovsky≤
        github.com/bodagovsky+
        github.com/bodagovsky=
        github.com/bodagovsky≠
        github.com/bodagovsky}
        github.com/bodagovsky‘
        github.com/bodagovsky{
        github.com/bodagovsky“
        github.com/bodagovsky|
        github.com/bodagovsky«
        github.com/bodagovsky~
        github.com/bodagovsky`
        github.com/bodagovsky±
        github.com/bodagovsky_
        github.com/bodagovsky-
        github.com/bodagovsky.";
    let mut output: HashSet<Uri> = HashSet::from_iter(parse(input));
    let mut assertion_sample: HashSet<Uri> = vec![
        "https://github.com/bodagovsky",
        "https://github.com/bodagovsky_",
        "https://github.com/bodagovsky-",
        "https://github.com/bodagovsky.",
    ]
    .iter()
    .map(|uri| uri.parse::<Uri>().unwrap())
    .collect();

    assert_eq!(output, assertion_sample)
}

#[async_std::test]
#[ignore]
async fn test_search() {
    let acc = accountant::Accountant::new();
    let req = accountant::SearchRequest::new("web")
        .filter_by("license = 'APACHE 2.0'")
        .set_limit(1);
    let response = acc.search(req).await.unwrap();
    println!("{:?}", response)
}

use super::{fetch, parse};
use crate::model::Document;
use chrono::prelude::*;
use surf;

#[test]
fn test_parse() {
    let contents = include_str!("go.mod");
    let mut parsed_file = parse(contents);

    let mut compare = vec![
        "github.com/360EntSecGroup-Skylar/excelize".to_owned(),
        "github.com/go-chi/chi".to_owned(),  //v4.0.2+incompatible
        "github.com/go-chi/cors".to_owned(), //v1.0.0
        "github.com/go-openapi/spec".to_owned(), //v0.19.3
        "github.com/golang/protobuf".to_owned(), //v1.4.2
        "github.com/grpc-ecosystem/grpc-gateway".to_owned(), //v1.14.2
        "github.com/lib/pq".to_owned(),      //v1.3.0  indirect
        "github.com/pkg/errors".to_owned(),  //v0.8.1
        "github.com/spf13/pflag".to_owned(), //v1.0.5
        "github.com/spf13/viper".to_owned(), //v1.6.1
        "github.com/stretchr/testify".to_owned(), //v1.4.0
        "github.com/utrack/clay/v2".to_owned(), //v2.4.7
        "gitlab.ru/internal-projects/staff-api".to_owned(), //v0.0.0-20200619100014-b44a8677723f
        "gitlab.ru/platform/database-go".to_owned(), //v0.15.2
        "gitlab.ru/platform/errors".to_owned(), //v1.3.6
        "gitlab.ru/platform/realtime-config-go".to_owned(), //v1.8.7
        "gitlab.ru/platform/scratch".to_owned(), //v1.6.8
        "gitlab.ru/platform/tracer-go".to_owned(), //v1.16.0
    ];

    parsed_file.sort();
    compare.sort();
    assert_eq!(parsed_file, compare);
}

#[async_std::test]
async fn test_fetcher() {
    let links = fetch("https://awesome-go.com").await;
    assert!(links.is_ok());
    match links {
        Ok(resp) => {
            for link in &resp[..10] {
                let req = surf::get(format!("https://{}", link.as_str())).await;
                assert!(req.is_ok());
            }
        }
        Err(error) => println!("error {:?}", error),
    }
}

#[test]
fn test_link_extractor() {
    // º and ª symbols not covered
    let input = "github.com/bodagovsky\\
        github.com/bodagovsky!
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
    let mut output: Vec<String> = parse(input);
    let mut assertion_sample = vec![
        "github.com/bodagovsky".to_owned(),
        "github.com/bodagovsky_".to_owned(),
        "github.com/bodagovsky-".to_owned(),
        "github.com/bodagovsky.".to_owned(),
    ];
    output.sort();
    assertion_sample.sort();
    assert_eq!(output, assertion_sample)
}

#[async_std::test]
async fn send() {
    let lib = Document::new(1)
        .name("librarian")
        .description("best lib ever")
        .link("http://")
        .target_language("All")
        .last_commit(
            Utc.datetime_from_str("2020-07-24 12:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        )
        .last_release(
            Utc.datetime_from_str("2014-07-19 12:30:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        )
        .license("MIT")
        .usage("web search");
    let response =
        surf::post("http://127.0.0.1:7700/indexes/libraries/documents").body_json(&vec![lib]);
    match response {
        Ok(req) => {
            let r = req.await;
            match r {
                Ok(resp) => assert!(resp.status() == 202),
                Err(err) => panic!("unexpected, {}", err),
            }
        }
        Err(error) => panic!("error occured, {}", error),
    }
}

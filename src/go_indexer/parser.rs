#[cfg(test)]
mod tests;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use anyhow::anyhow;
use http::Uri;

use crate::crawler::Err;

/// Github API:
///
/// First, call https://api.github.com/repos/risboo6909/when to get a list of handlers,
/// and some general info such as license, description, pushed_at, etc.
///
/// Then use:
/// link from "releases_url" - to fetch releases data
/// https://raw.githubusercontent.com/risboo6909/when/master/README.md - to download raw README.md
/// or any other file

fn prepare_github_links(uri: Uri) -> Vec<Uri> {
    // TODO: implement
    // println!("{:?}", uri.path());
    vec![uri]
}

fn prepare_gitlab_links(uri: Uri) -> Vec<Uri> {
    // TODO: implement
    vec![uri]
}

/// Parse library uri and return a set of api handlers to call later from crawler for each uri
pub(crate) fn parse(input: &str) -> HashMap<String, Vec<Uri>> {

    // id -> {set of uris}
    let mut parsed: HashMap<String, Vec<Uri>> = HashMap::new();

    let re = Regex::new(r"(github|gitlab)\.(com|ru)[\w\d/\-\.]*").unwrap();

    for link in re.captures_iter(input) {

        // assume all links are HTTPS ones
        let uri = match format!("https://{}", &link[0]).parse::<Uri>() {
            Ok(s) => s,
            Err(_) => continue,
        };

        // separate handlers for github and gitlab repos
        match &link[1] {
            "github" => {
                let tmp = prepare_github_links(uri);
                parsed.insert(link[0].to_owned(), tmp);
            }
            "gitlab" => {
                let tmp = prepare_gitlab_links(uri);
                parsed.insert(link[0].to_owned(), tmp);
            }
            _ => continue,
        };

    }

    parsed

}

pub(crate) async fn fetch(url: &str) -> anyhow::Result<HashMap<String, Vec<Uri>>> {
    match surf::get(url).recv_string().await {
        Ok(res) => Ok(parse(res.as_str())),
        Err(_) => Err(anyhow!("error fetching url: '{}'", url)),
    }
}

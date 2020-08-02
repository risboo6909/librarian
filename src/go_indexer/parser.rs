#[cfg(test)]
mod tests;

use regex::Regex;
use std::collections::HashSet;

use anyhow::anyhow;
use http::Uri;

use crate::crawler::Err;

/// Parses go.mod file fetching github/gitlab unique link
/// and returning a vector of parsed links as strings
pub(crate) fn parse(input: &str) -> Vec<Uri> {
    let mut parsed: HashSet<String> = HashSet::new();
    let re = Regex::new(r"(github|gitlab)\.(com|ru)[\w\d/\-\.]*").unwrap();
    for link in re.captures_iter(input) {
        parsed.insert((&link[0]).to_string());
    }
    // assume all links are HTTPS ones
    parsed.iter().filter_map(|uri|
        // ignore invalid uris
        format!("https://{}", uri).parse::<Uri>().ok()
    ).collect()
}

pub(crate) async fn fetch(url: &str) -> anyhow::Result<Vec<Uri>> {
    match surf::get(url).recv_string().await {
        Ok(res) => Ok(parse(res.as_str())),
        Err(_) => Err(anyhow!("error fetching url: '{}'", url)),
    }
}

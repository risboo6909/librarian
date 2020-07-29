#[cfg(test)]
mod tests;

use regex::Regex;
use std::collections::HashMap;

use crate::crawler::Err;

/// Parses go.mod file fetching github/gitlab unique link
/// and returning a vector of parsed links as strings
pub(crate) fn parse(input: &str) -> Vec<String> {
    let mut link_map: HashMap<String, usize> = HashMap::new();
    let re = Regex::new(r"(github|gitlab)\.(com|ru)[\w\d/\-\.]*").unwrap();
    for link in re.captures_iter(input) {
        link_map.insert((&link[0]).to_string(), 0);
    }
    let links: Vec<String> = link_map.keys().map(|x| x.to_owned()).collect();
    links
}

pub(crate) async fn fetch(url: String) -> Result<Vec<String>, Err> {
    let req = surf::get(url).recv_string().await?;
    let keys = parse(req.as_str());
    Ok(keys)
}

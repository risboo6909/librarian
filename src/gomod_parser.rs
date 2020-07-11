#[cfg(test)]
mod tests;

use regex::Regex;
use std::collections::HashMap;

/// Parses go.mod file fetching github/gitlab unique link
/// and returning a vector of parsed links as strings
pub(crate) fn parse(input: &str) -> Vec<String> {
    let mut link_map: HashMap<&str, usize> = HashMap::new();
    let re = Regex::new(r"^(github|gitlab)\.(com|ru)\S*").unwrap();
    input
        .split(char::is_whitespace)
        .filter(|x| re.is_match(x))
        .map(|x| link_map.insert(x, 0))
        .for_each(drop);
    let links: Vec<&str> = link_map.keys().map(|x| x.to_owned()).collect();
    links.into_iter().map(|x| x.to_owned()).collect()
}

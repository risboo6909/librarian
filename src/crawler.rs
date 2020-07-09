use surf;

struct Crawler {
    max_downloads: usize,
}

pub(crate) async fn crawl(urls: &[&str]) {
    let client = surf::Client::new();

}

#[cfg(test)]
mod tests {

}

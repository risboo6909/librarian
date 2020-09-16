use async_trait::async_trait;
use chrono::Duration;
use http::Uri;

use std::collections::HashMap;
use std::time::Duration as StdDur;

mod parser;

use super::crawler::{Crawler, RepoUri, HandlerUri, Err as CrawlerErr};
use super::post_proc::post_proc;
use super::scheduler::IndexerTrait;
use parser::{fetch, parse};

trait GoIndexerExt {
    fn fetch_go_mod_urls(&self, inp: &HashMap<RepoUri, HashMap<HandlerUri, Result<String, CrawlerErr>>>) -> HashMap<String, Vec<Uri>>; 
}

pub(crate) struct Indexer {
    run_delay: Duration,
    max_scan_depth: usize,
}

impl Indexer {
    pub(crate) fn new(run_delay: Duration, max_scan_depth: usize) -> Self {
        Indexer { 
            run_delay,
            max_scan_depth,
        }
    }
}

impl GoIndexerExt for Indexer {

    fn fetch_go_mod_urls(&self, inp: &HashMap<RepoUri, HashMap<HandlerUri, Result<String, CrawlerErr>>>) -> HashMap<String, Vec<Uri>> {
        let mut result: HashMap<String, Vec<Uri>> = HashMap::new();

        for (repo_uri, uri_data) in inp {
            for (uri, res) in uri_data {
                if uri.ends_with(".mod") && res.is_ok() {
                    let body = res.as_ref().unwrap();
                    let tmp = parse(&body);

                    result.extend(tmp);
                }
            }
        }

        result
    }
    
}

#[async_trait]
impl IndexerTrait for Indexer {

    async fn refresh_index(&mut self) -> anyhow::Result<()> {
        unimplemented!()
    }


    async fn update_index(&mut self) -> anyhow::Result<()> {

        // first, fetch awesome go page
        let mut to_fetch = fetch("https://awesome-go.com").await?;
        let mut cur_depth = 0;

        while cur_depth < self.max_scan_depth {
            println!("Current depth {} of {}, uris to crawl: {}...", cur_depth+1, self.max_scan_depth, to_fetch.len());

            // crawl urls
            let mut crawler = Crawler::new(&to_fetch, 10, StdDur::from_secs(5));
            crawler.set_post_proc(post_proc);
    
            let results = crawler.crawl().await;

            // parse go.mod data for each repo if exists
            to_fetch = self.fetch_go_mod_urls(&results);
            cur_depth += 1; 
        }

        Ok(())
    }

    fn get_id(&self) -> String {
        "go_indexer".to_owned()
    }

    fn next_start_delay(&self) -> Duration {
        self.run_delay
    }
}

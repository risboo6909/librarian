use async_trait::async_trait;
use chrono::Duration;
use std::time::Duration as StdDur;

mod parser;

use super::crawler::Crawler;
use super::post_proc::post_proc;
use super::scheduler::IndexerTrait;
use parser::fetch;

pub(crate) struct Indexer {
    run_delay: Duration,
}

impl Indexer {
    pub(crate) fn new(run_delay: Duration) -> Self {
        Indexer { run_delay }
    }
}

#[async_trait]
impl IndexerTrait for Indexer {
    async fn refresh_index(&mut self) -> anyhow::Result<()> {
        unimplemented!()
    }

    async fn update_index(&mut self) -> anyhow::Result<()> {
        // first, fetch awesome go page
        let parsed = fetch("https://awesome-go.com").await?;

        // println!("{:?}", parsed);

        // second, crawl urls
        let mut crawler = Crawler::new(parsed, 10, StdDur::from_secs(5));
        crawler.set_post_proc(post_proc);

        let results = crawler.crawl().await;

        Ok(())
    }

    fn get_id(&self) -> String {
        "go_indexer".to_owned()
    }

    fn next_start_delay(&self) -> Duration {
        self.run_delay
    }
}

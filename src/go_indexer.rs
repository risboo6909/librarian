use async_trait::async_trait;
use chrono::Duration;

mod parser;

use super::crawler::Crawler;
use super::scheduler::IndexerTrait;
use anyhow::bail;
use parser::{fetch, parse};

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

        // cant use ? here, due to https://github.com/dtolnay/anyhow/issues/35
        let parsed = match fetch("https://awesome-go.com").await {
            Ok(parsed) => parsed,
            Err(err) => bail!(err),
        };

        println!("{:?}", parsed);
        //let mut crawler = Crawler::new()
        Ok(())
    }

    fn get_id(&self) -> String {
        "go_indexer".to_owned()
    }

    fn next_start_delay(&self) -> Duration {
        self.run_delay
    }
}

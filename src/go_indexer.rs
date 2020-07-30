use chrono::Duration;
use async_trait::async_trait;

mod parser;

use super::scheduler::IndexerTrait;
use super::crawler::Crawler;
use parser::{parse, fetch};
use anyhow::bail;

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

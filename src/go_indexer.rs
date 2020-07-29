use anyhow::{Error, Result};
use chrono::Duration;

mod parser;

use super::scheduler::IndexerTrait;

pub(crate) struct Indexer {
    run_delay: Duration,
}

impl Indexer {
    pub(crate) fn new(run_delay: Duration) -> Self {
        Indexer { run_delay }
    }
}

impl IndexerTrait for Indexer {
    fn refresh_index(&mut self) -> Result<(), Error> {
        unimplemented!()
    }

    fn update_index(&mut self) -> Result<(), Error> {
        unimplemented!()
    }

    fn get_id(&self) -> String {
        "go_indexer".to_owned()
    }

    fn next_start_delay(&self) -> Duration {
        self.run_delay
    }
}

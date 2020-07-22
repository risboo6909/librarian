mod parser;

use std::error::Error;
use anyhow::Result;

use super::indexer::IndexerTrait;

struct Indexer {

}

impl IndexerTrait for Indexer {
    fn update_index() -> Result<()> {
        unimplemented!()
    }
}

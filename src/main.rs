use anyhow::{Error, Result};
use async_std::task;
use chrono::Duration;
use log::Level;

mod crawler;
mod go_indexer;
mod model;
mod scheduler;
mod save;

use scheduler::Scheduler;

fn main() -> Result<(), Error> {
    simple_logger::init_with_level(Level::Info).unwrap();

    let mut sched = Scheduler::new();
    let go_idx = go_indexer::Indexer::new(Duration::minutes(1));
    sched.add_indexer(Box::new(go_idx))?;

    task::block_on(sched.run());

    Ok(())
}

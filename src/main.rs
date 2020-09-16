use anyhow::{Error, Result};
use async_std::task;
use chrono::Duration;
use log::Level;

mod accountant;
mod crawler;
mod go_indexer;
mod model;
mod post_proc;
mod scheduler;

use std::sync::RwLock;

#[macro_use]
use lazy_static::lazy_static;
use config::Config;
use scheduler::Scheduler;

lazy_static! {
    static ref CONF: RwLock<Config> = RwLock::new(Config::default());
}

fn main() -> Result<(), Error> {
    CONF.write()
        .unwrap()
        .set_default("meili_host", "http://localhost")
        .unwrap();
    CONF.write()
        .unwrap()
        .set_default("meili_port", 7700)
        .unwrap();
    CONF.write()
        .unwrap()
        .merge(config::File::with_name("conf/settings.toml").required(false))
        .unwrap();

    simple_logger::init_with_level(Level::Error).unwrap();

    let mut sched = Scheduler::new();
    let go_idx = go_indexer::Indexer::new(Duration::minutes(1), 2);
    sched.add_indexer(Box::new(go_idx))?;

    task::block_on(sched.run());

    Ok(())
}

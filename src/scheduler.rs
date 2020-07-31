use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::thread::sleep;
use std::time::{Duration as StdDur, SystemTime, UNIX_EPOCH};

use anyhow::anyhow;
use async_trait::async_trait;
use chrono::Duration;
use log::info;

#[async_trait]
pub(crate) trait IndexerTrait {
    // should be called to refresh existent index
    async fn refresh_index(&mut self) -> anyhow::Result<()>;

    // scan to find new items for index
    async fn update_index(&mut self) -> anyhow::Result<()>;

    fn get_id(&self) -> String;

    fn next_start_delay(&self) -> Duration;
}

struct Item {
    next_start_ts: usize,
    indexer: Box<dyn IndexerTrait>,
}

impl Eq for Item {}

impl PartialEq for Item {
    fn eq(&self, other: &Item) -> bool {
        other.next_start_ts == self.next_start_ts
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Item) -> Ordering {
        other.next_start_ts.cmp(&self.next_start_ts)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
        Some(other.next_start_ts.cmp(&self.next_start_ts))
    }
}

pub(crate) struct Scheduler {
    indexers: BinaryHeap<Item>,
}

impl Scheduler {
    pub(crate) fn new() -> Self {
        Scheduler {
            indexers: BinaryHeap::new(),
        }
    }

    fn now() -> usize {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
    }

    fn check_exists(&self, id: &str) -> bool {
        for item in &self.indexers {
            if item.indexer.get_id() == id {
                return true;
            }
        }
        false
    }

    pub(crate) fn add_indexer(&mut self, indexer: Box<dyn IndexerTrait>) -> anyhow::Result<()> {
        if self.check_exists(&indexer.get_id()) {
            return Err(anyhow!(
                "Indexer with id '{}' already exists",
                indexer.get_id()
            ));
        }

        let tmp = Item {
            // first time we start as soon as possible
            next_start_ts: Scheduler::now(),
            indexer,
        };

        self.indexers.push(tmp);
        Ok(())
    }

    pub(crate) async fn run(&mut self) -> anyhow::Result<()> {
        while let Some(Item {
            next_start_ts,
            mut indexer,
        }) = self.indexers.pop()
        {
            if next_start_ts <= Scheduler::now() {
                info!("starting update for {}", indexer.get_id());
                indexer.update_index().await?;

                info!("starting refresh for {}", indexer.get_id());
                //indexer.refresh_index()?;

                // enqueue task again
                let delay = indexer.next_start_delay();
                self.indexers.push(Item {
                    next_start_ts: delay.num_seconds() as usize + Scheduler::now(),
                    indexer,
                })
            } else {
                self.indexers.push(Item {
                    next_start_ts,
                    indexer,
                });
                let now = Scheduler::now();
                if next_start_ts >= now {
                    let delta = next_start_ts - now;
                    info!("going to sleep for {} seconds", delta);
                    sleep(StdDur::from_secs(delta as u64));
                }
            }
        }

        Err(anyhow!("Nothing left to execute"))
    }
}

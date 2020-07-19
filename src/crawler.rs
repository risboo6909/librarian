use std::collections::{HashMap, VecDeque};
use std::time::Duration;
use std::{cmp::min, pin::Pin};

use futures::stream::FuturesUnordered;
use futures::{stream, Future, FutureExt, StreamExt};
use log::{error, info};

type StringPair = (String, String);

type Err = Box<dyn std::error::Error + Send + Sync>;
type MyErr = (String, Err);

type MyResult = Result<Result<StringPair, MyErr>, async_std::future::TimeoutError>;

pub(crate) struct Crawler {
    max_clients: usize,
    request_timeout: Duration,
    uris: VecDeque<String>,

    strm: FuturesUnordered<Pin<Box<dyn Future<Output = MyResult>>>>,
}

impl Crawler {
    pub(crate) fn new(uris: &[&str], max_clients: usize, request_timeout: u64) -> Self {
        Crawler {
            max_clients,
            request_timeout: Duration::from_secs(request_timeout),
            uris: uris.iter().map(|s| String::from(*s)).collect(),

            strm: stream::FuturesUnordered::new(),
        }
    }

    pub(crate) fn enqueue_job(&mut self) {
        if self.uris.is_empty() {
            return;
        }

        let uri = self.uris.pop_front().unwrap();

        self.strm.push(
            async_std::future::timeout(self.request_timeout, async {
                let client = surf::get(uri.clone());
                match client.recv_string().await {
                    Ok(body) => Ok((uri, body)),
                    Err(err) => Err((uri, err)),
                }
            })
            .boxed(),
        );
    }

    // Fetches given urls asynchronously consuming self
    pub(crate) async fn crawl(mut self) -> HashMap<String, Result<String, Err>> {
        let mut res = HashMap::new();

        // create initial clients
        for _ in 0..min(self.max_clients, self.uris.len()) {
            self.enqueue_job();
        }

        // process downloads
        while let Some(result) = self.strm.next().await {
            match result.unwrap() {
                Ok((uri, body)) => {
                    info!("{} fetched successfully", uri);
                    res.insert(uri, Ok(body));
                }
                Err((uri, err)) => {
                    error!("error fetching uri: {}, reason: {}", uri, err);
                    res.insert(uri, Err(err));
                }
            };
            self.enqueue_job();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Crawler;
    use log::Level;

    #[async_std::test]
    async fn test_fetch() {
        simple_logger::init_with_level(Level::Error).unwrap();

        let crawler = Crawler::new(
            &[
                "https://www.google.com",
                "https://dsfs",
                "https://yahoo.com",
                "https://reddit.com",
            ],
            2,
            5,
        );

        let res = crawler.crawl().await;

        // total number of results, including erroneous ones
        assert_eq!(res.len(), 4);

        // hope this websites will work most of the time :)
        assert!(res["https://www.google.com"].is_ok());
        assert!(res["https://yahoo.com"].is_ok());
        assert!(res["https://reddit.com"].is_ok());

        // ...and this one should fail
        assert!(res["https://dsfs"].is_err());
    }
}

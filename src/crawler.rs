use std::collections::{HashMap, VecDeque};
use std::time::Duration;
use std::{cmp::min, pin::Pin};

use futures::stream::FuturesUnordered;
use futures::{stream, Future, FutureExt, StreamExt};
use log::{error, info};
use http::Uri;

type StringPair = (String, String);

pub type Err = Box<dyn std::error::Error + Send + Sync>;

type MyErr = (String, Err);
type MyResult = Result<Result<StringPair, MyErr>, async_std::future::TimeoutError>;

pub(crate) struct Crawler {
    max_clients: usize,
    request_timeout: Duration,
    uris: VecDeque<String>,

    strm: FuturesUnordered<Pin<Box<dyn Future<Output = MyResult> + Send>>>,
}

impl Crawler {
    pub(crate) fn new(uris: &[Uri], max_clients: usize, request_timeout: Duration) -> Self {
        Crawler {
            max_clients,
            request_timeout,
            uris: uris.iter().map(|uri| uri.to_string()).collect(),

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
                let client = surf::get(uri.to_string());
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
    use http::Uri;
    use std::time::Duration;

    #[async_std::test]
    async fn test_fetch() {
        simple_logger::init_with_level(Level::Error).unwrap();

        let crawler = Crawler::new(
            &[
                "https://www.google.com".parse::<Uri>().unwrap(),
                "https://dsfs".parse::<Uri>().unwrap(),
                "https://yahoo.com".parse::<Uri>().unwrap(),
                "https://reddit.com".parse::<Uri>().unwrap(),
            ],
            2,
            Duration::from_secs(5),
        );

        let res = crawler.crawl().await;

        // total number of results, including erroneous ones
        assert_eq!(res.len(), 4);

        // hope this websites will work most of the time :)
        assert!(res["https://www.google.com/"].is_ok());
        assert!(res["https://yahoo.com/"].is_ok());
        assert!(res["https://reddit.com/"].is_ok());

        // ...and this one should fail
        assert!(res["https://dsfs/"].is_err());
    }
}

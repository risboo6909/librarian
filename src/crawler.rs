use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Duration;
use std::{cmp::min, pin::Pin};

use futures::stream::FuturesUnordered;
use futures::{stream, Future, FutureExt, StreamExt};
use http::Uri;
use log::{error, info};

pub type Err = Box<dyn std::error::Error + Send + Sync>;

// first item is an unique id, second item is uri to fetch data
// for instance: IdUri = ("google.com", "https://www.google.com")
type IdUri = (String, String);

// first item is IdUri (see above), second item is response body
// as a string
type ResultOk = (IdUri, String);

// the same as ResultOk, but the last item is an Err type
type ResultErr = (IdUri, Err);

type MyResult = Result<Result<ResultOk, ResultErr>, async_std::future::TimeoutError>;

pub(crate) struct Crawler {
    max_clients: usize,
    request_timeout: Duration,
    ids_uris: VecDeque<IdUri>,

    strm: FuturesUnordered<Pin<Box<dyn Future<Output = MyResult> + Send>>>,
}

impl Crawler {
    pub(crate) fn new(
        uris: HashMap<String, HashSet<Uri>>,
        max_clients: usize,
        request_timeout: Duration,
    ) -> Self {
        let mut converted: Vec<IdUri> = vec![];

        // convert to Vec<(group_name, handler)> form
        for (uri, handlers) in uris {
            for handler in handlers {
                converted.push((uri.clone(), handler.to_string()));
            }
        }

        Crawler {
            max_clients,
            request_timeout,
            ids_uris: VecDeque::from(converted),

            strm: stream::FuturesUnordered::new(),
        }
    }

    pub(crate) fn enqueue_job(&mut self) {
        if self.ids_uris.is_empty() {
            return;
        }

        let pair = self.ids_uris.pop_front().unwrap();

        self.strm.push(
            async_std::future::timeout(self.request_timeout, async move {
                let (_id, uri) = pair.clone();
                let client = surf::get(uri);

                match client.recv_string().await {
                    Ok(body) => Ok((pair, body)),
                    Err(err) => Err((pair, err)),
                }
            })
            .boxed(),
        );
    }

    // fetches given urls asynchronously consuming self
    pub(crate) async fn crawl(mut self) -> HashMap<String, HashMap<String, Result<String, Err>>> {
        let mut res = HashMap::new();

        // create initial clients
        for _ in 0..min(self.max_clients, self.ids_uris.len()) {
            self.enqueue_job();
        }

        // process downloads
        while let Some(result) = self.strm.next().await {
            match result.unwrap() {
                Ok(((id, uri), body)) => {
                    info!("{} fetched successfully", uri);
                    res.entry(id)
                        .or_insert_with(HashMap::new)
                        .insert(uri, Ok(body));
                }
                Err(((id, uri), err)) => {
                    error!("error fetching uri: {}, reason: {}", uri, err);
                    res.entry(id)
                        .or_insert_with(HashMap::new)
                        .insert(uri, Err(err));
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
    use http::Uri;
    use log::Level;
    use std::collections::{HashMap, HashSet};
    use std::iter::FromIterator;
    use std::time::Duration;

    #[async_std::test]
    async fn test_crawler() {
        simple_logger::init_with_level(Level::Error).unwrap();

        let mut input = HashMap::new();

        input.insert(
            String::from("google.com"),
            HashSet::from_iter(vec!["https://www.google.com".parse::<Uri>().unwrap()]),
        );
        input.insert(
            String::from("sdfdf"),
            HashSet::from_iter(vec!["https://sdfdf".parse::<Uri>().unwrap()]),
        );
        input.insert(
            String::from("yahoo.com"),
            HashSet::from_iter(vec!["https://www.yahoo.com".parse::<Uri>().unwrap()]),
        );
        input.insert(
            String::from("reddit.com"),
            HashSet::from_iter(vec!["https://www.reddit.com".parse::<Uri>().unwrap()]),
        );

        let crawler = Crawler::new(input, 2, Duration::from_secs(5));

        let res = crawler.crawl().await;

        // total number of results, including erroneous ones
        assert_eq!(res.len(), 4);

        // hope this websites will work most of the time :)
        assert!(res["google.com"]["https://www.google.com/"].is_ok());
        assert!(res["yahoo.com"]["https://www.yahoo.com/"].is_ok());
        assert!(res["reddit.com"]["https://www.reddit.com/"].is_ok());

        // ...and this one should fail
        assert!(res["sdfdf"]["https://sdfdf/"].is_err());
    }
}

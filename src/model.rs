use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surf;

#[derive(Serialize, Deserialize)]
pub(crate) struct Library {
    id: u32,
    name: String,
    description: String,
    link: String,
    target_language: String,
    last_commit: Option<DateTime<Utc>>,
    last_release: Option<DateTime<Utc>>,
    license: String,
    usage: String,
}

// sending vector of libraries to meilisearch server
// fn send(lib: Library) -> Result<()> {
//     let request = surf::get("127.0.0.1:7700").recv_string()?;
//     Ok()
// }

impl Library {
    fn new() -> Self {
        Library {
            id: 0,
            name: "".to_owned(),
            description: "".to_owned(),
            link: "".to_owned(),
            target_language: "".to_owned(),
            last_commit: None,
            last_release: None,
            license: "".to_owned(),
            usage: "".to_owned(),
        }
    }
}

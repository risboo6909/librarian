use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Document {
    id: u32,
    name: String,
    description: String,
    link: String,
    target_language: String,
    last_commit: i64,
    last_release: i64,
    license: String,
    usage: String,
}

impl Document {
    pub(crate) fn new(id: u32) -> Self {
        Document {
            id,
            name: "".to_owned(),
            description: "".to_owned(),
            link: "".to_owned(),
            target_language: "".to_owned(),
            last_commit: 0,
            last_release: 0,
            license: "".to_owned(),
            usage: "".to_owned(),
        }
    }

    pub(crate) fn name(mut self, n: &str) -> Self {
        self.name = n.to_owned();
        self
    }

    pub(crate) fn description(mut self, text: &str) -> Self {
        self.description = text.to_owned();
        self
    }

    pub(crate) fn link(mut self, uri: &str) -> Self {
        self.link = uri.to_owned();
        self
    }

    pub(crate) fn target_language(mut self, lang: &str) -> Self {
        self.target_language = lang.to_owned();
        self
    }

    pub(crate) fn last_commit(mut self, date: i64) -> Self {
        self.last_commit = date;
        self
    }

    pub(crate) fn last_release(mut self, date: i64) -> Self {
        self.last_release = date;
        self
    }

    pub(crate) fn license(mut self, l: &str) -> Self {
        self.license = l.to_owned();
        self
    }
    pub(crate) fn usage(mut self, u: &str) -> Self {
        self.usage = u.to_owned();
        self
    }
}

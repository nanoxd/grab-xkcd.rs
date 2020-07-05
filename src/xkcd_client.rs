use crate::models::{Comic, ComicResponse, Options};
use anyhow::Result;
use std::convert::TryInto;
use std::time::Duration;

const BASE_URL: &str = "https://xkcd.com";

pub struct XkcdClient {
    pub options: Options,
}

impl XkcdClient {
    pub fn new(options: Options) -> Self {
        XkcdClient { options }
    }

    pub fn run(self) -> Result<()> {
        let url = self.options.num.map_or_else(
            || format!("{}/info.0.json", BASE_URL),
            |n| format!("{}/{}/info.0.jsonl", BASE_URL, n),
        );

        let client = reqwest::blocking::ClientBuilder::new()
            .timeout(Duration::from_secs(self.options.timeout))
            .build()?;

        let resp: ComicResponse = client.get(&url).send()?.text()?.try_into()?;
        let comic: Comic = resp.into();

        if self.options.save {
            comic.save()?;
        }

        comic.print(self.options.output)?;

        Ok(())
    }
}

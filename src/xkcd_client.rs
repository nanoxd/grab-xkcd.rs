use crate::models::Options;
use anyhow::Result;

pub struct XkcdClient {
    pub options: Options,
}

impl XkcdClient {
    pub fn new(options: Options) -> Self {
        XkcdClient { options }
    }

    pub fn run(self) -> Result<()> {
        println!("Retrieving XKCD Comic");
        Ok(())
    }
}

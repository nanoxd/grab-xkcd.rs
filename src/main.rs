use anyhow::Result;
use clap::Clap;
use xkcd_client::XkcdClient;

mod models;
mod xkcd_client;

const BASE_URL: &str = "https://xkcd.com";

/// XKCD API treats this number as the newest item
const LATEST_COMIC: usize = 0;

fn main() -> Result<()> {
    let options = models::Options::parse();
    let client = XkcdClient::new(options);
    client.run()
}

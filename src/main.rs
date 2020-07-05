use anyhow::Result;
use clap::Clap;
use xkcd_client::XkcdClient;

mod models;
mod xkcd_client;

fn main() -> Result<()> {
    let options = models::Options::parse();
    let client = XkcdClient::new(options);
    client.run()
}

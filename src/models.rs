use clap::Clap;
use serde_derive::Deserialize;
use std::convert::TryFrom;

#[derive(Debug, Clap)]
pub struct Options {
    /// Sets a connection timeout
    #[clap(long, short, default_value = "60")]
    pub timeout: u64,

    /// Prints output in the given format
    #[clap(long, short, arg_enum, default_value = "text")]
    pub output: OutputFormat,

    /// The comic to loads
    #[clap(long, short, default_value = "0")]
    pub num: usize,

    /// Save image file to current directory
    #[clap(long, short)]
    pub save: bool,
}

#[derive(Debug, Clap, Copy, Clone)]
pub enum OutputFormat {
    Json,
    Text,
}

#[derive(Deserialize)]
pub struct ComicResponse {
    pub month: String,
    pub num: usize,
    pub link: String,
    pub year: String,
    pub news: String,
    pub safe_title: String,
    pub transcript: String,
    pub alt: String,
    pub img: String,
    pub title: String,
    pub day: String,
}

impl TryFrom<String> for ComicResponse {
    type Error = anyhow::Error;

    fn try_from(json: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&json).map_err(|e| e.into())
    }
}

pub struct Comic {
    pub title: String,
    pub num: usize,
    pub date: String,
    pub desc: String,
    pub img_url: String,
}

impl From<ComicResponse> for Comic {
    fn from(cr: ComicResponse) -> Self {
        Comic {
            title: cr.title,
            num: cr.num,
            date: format!("{}-{}-{}", cr.day, cr.month, cr.year),
            desc: cr.alt,
            img_url: cr.img,
        }
    }
}

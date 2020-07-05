use anyhow::Result;
use clap::Clap;
use serde_derive::{Deserialize, Serialize};
use std::io::Write;
use std::{convert::TryFrom, fmt};
use url::Url;

#[derive(Debug, Clap)]
pub struct Options {
    /// Sets a connection timeout
    #[clap(long, short, default_value = "60")]
    pub timeout: u64,

    /// Prints output in the given format
    #[clap(long, short, arg_enum, default_value = "text")]
    pub output: OutputFormat,

    /// The comic to loads
    #[clap(long, short)]
    pub num: Option<usize>,

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

#[derive(Serialize)]
pub struct Comic {
    pub title: String,
    pub num: usize,
    pub date: String,
    pub desc: String,
    pub img_url: String,
}

impl Comic {
    pub fn print(&self, of: OutputFormat) -> Result<()> {
        match of {
            OutputFormat::Text => println!("{}", self),
            OutputFormat::Json => println!("{}", serde_json::to_string(self)?),
        }

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let url = Url::parse(&*self.img_url)?;
        let img_name = url.path_segments().unwrap().last().unwrap();
        let path = std::env::current_dir()?;
        let path = path.join(img_name);
        let mut file = std::fs::File::create(path)?;

        let body = reqwest::blocking::get(&self.img_url)?;
        file.write_all(&*body.bytes()?).map_err(|e| e.into())
    }
}

impl fmt::Display for Comic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Title: {}\n\
            Comic No: {}\n\
            Date: {}\n\
            Description: {}\n\
            Image: {}\n",
            self.title, self.num, self.date, self.desc, self.img_url
        )
    }
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

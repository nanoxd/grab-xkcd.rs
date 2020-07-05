use clap::Clap;

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

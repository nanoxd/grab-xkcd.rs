use crate::models::Options;

pub struct XkcdClient {
    pub options: Options,
}

impl XkcdClient {
    pub fn new(options: Options) -> Self {
        XkcdClient { options }
    }

    pub fn run(self) {
        println!("Retrieving XKCD Comic");
    }
}

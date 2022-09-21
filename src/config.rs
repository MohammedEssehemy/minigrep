use std::env;


#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new<T: Iterator<Item = String>>(args: T) -> Result<Config, &'static str> {
        // skip first input which is program name
        let mut args = args.skip(1);
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }

    pub fn from<T: Iterator<Item = String>>(args: T) -> Result<Config, &'static str> {
       Self::new(args)
    }
}

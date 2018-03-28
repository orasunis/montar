//! Montar is a minecraft server written entirely in rust.

extern crate tokio;

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

#[derive(Deserialize)]
pub struct Config {
    /// The address the server is binding to.
    #[serde(default = "default_address")]
    pub address: String,

    /// Logging
    pub log: LogConfig,
}

#[derive(Deserialize)]
pub struct LogConfig {
    /// The log level.
    #[serde(default = "default_level_filter")]
    pub level: log::LevelFilter,

    /// The date format.
    /// As formatting syntax, strftime is used.
    #[serde(default = "default_date_format")]
    pub date_format: String,

    /// Additional files to save the logging output to.
    /// Does not support formatting like dates at the moment.
    #[serde(default = "Vec::new")]
    pub files: Vec<String>,
}

// default values for the config
fn default_address() -> String {
    "127.0.0.1:25565".to_owned()
}

fn default_level_filter() -> log::LevelFilter {
    log::LevelFilter::Info
}

fn default_date_format() -> String {
    "%d-%m-%Y %H-%M-%S".to_owned()
}

pub struct Montar {
    config: Config,
}

impl Montar {
    /// Create a new montar instance with a configuration.
    pub fn new(config: Config) -> Montar {
        Montar { config }
    }

    /// Start montar.
    pub fn start(&mut self) {}
}

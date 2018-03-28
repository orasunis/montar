extern crate montar;

extern crate log;

extern crate chrono;
extern crate fern;
extern crate toml;

use montar::{Config, Montar};

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

fn main() {
    let config_file = env::args()
        .nth(1)
        .unwrap_or_else(|| "Montar.toml".to_owned());

    // First load the configuration file.
    let mut config_file = File::open(&config_file).unwrap_or_else(move |err| {
        eprintln!(
            "Could not open configuration file located at '{}' because of the following error:",
            config_file
        );
        eprintln!("{}", err);
        process::exit(1);
    });

    // Read the entire config to a string.
    let mut config_content = String::new();
    config_file
        .read_to_string(&mut config_content)
        .unwrap_or_else(|err| {
            eprintln!(
                "The configuration file could not be read, maybe try converting it to UTF-8?"
            );
            eprintln!("{}", err);
            process::exit(2);
        });

    // Parse the configuration.
    let config: Config = toml::from_str(&*config_content).unwrap_or_else(|err| {
        eprintln!("The provided configuration file contains at least one error:");
        eprintln!("{}", err);
        process::exit(3);
    });

    // Cloning the date format to satisfy the borrow checker.
    let date_format = config.log.date_format.clone();

    // Configure logging.
    let mut fe = fern::Dispatch::new()
        .format(move |out, msg, record| {
            out.finish(format_args!(
                "{} [{}] [{}]: {}",
                chrono::Local::now().format(&*date_format),
                record.target(),
                record.level(),
                msg
            ))
        })
        .level(config.log.level)
        .chain(std::io::stdout());

    // Allow for files to save to.
    for file in &config.log.files {
        let log_file = fern::log_file(&file).unwrap_or_else(move |err| {
            eprintln!(
                "Could not open log file located at '{}' because of the following error:",
                file
            );
            eprintln!("{}", err);
            process::exit(4);
        });
        fe = fe.chain(log_file);
    }

    // Apply the logging configuration.
    fe.apply().unwrap_or_else(|err| {
        eprintln!("The logging configuration could not be set up because of the following error:");
        eprintln!("{}", err);
        process::exit(4);
    });

    let mut montar = Montar::new(config);

    montar.start();
}

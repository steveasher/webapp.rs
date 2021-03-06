#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate webapp;
extern crate webapp_backend;

use clap::App;
use failure::Fallible;
use std::{env::set_var, process::exit};
use webapp::config::Config;
use webapp_backend::Server;

fn main() -> Fallible<()> {
    // Load the CLI parameters from the yaml file
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    // Retrieve the config file path
    let config_filename = matches
        .value_of("config")
        .ok_or_else(|| format_err!("No 'config' provided"))?;

    // Parse the configuration
    let config = Config::from_file(config_filename)?;

    // Set the logging verbosity
    set_var(
        "RUST_LOG",
        format!(
            "actix_web={},webapp={},backend={}",
            config.log.actix_web, config.log.webapp, config.log.webapp
        ),
    );

    // Initialize the logger
    env_logger::init();

    // Create and start the server
    info!(
        "Starting server from config path {} for url {}",
        config_filename, config.server.url
    );
    let server = Server::from_config(&config)?;

    // Start the server
    exit(server.start());
}

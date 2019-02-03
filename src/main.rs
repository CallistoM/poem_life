#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;

use std::env;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use poem_life::Config;
use std::io::prelude::*;

fn init_logger() {
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%d %H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}

fn main() {
    init_logger();
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        info!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    info!("searching for: {:?}", config.query);
    info!("in file: {:?}", config.filename);

    if let Err(e) = poem_life::run(config) {
        info!("Application error: {}", e);
        std::process::exit(1);
    };
}
extern crate clap;
extern crate rand;
extern crate tiny_http;

mod game;
mod commands {
    pub mod http;
}

use clap::{App, AppSettings};

fn main() {
    let matches = App::new("Guessing Game")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(commands::http::declare())
        .get_matches();

    match matches.subcommand() {
        ("http", Some(matches)) =>  commands::http::run(matches),
        _ =>                        {}
    };
}

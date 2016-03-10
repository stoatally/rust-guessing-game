extern crate clap;
extern crate rand;
extern crate tiny_http;

mod game;
mod commands {
    pub mod http;
}

use clap::{Arg, App, AppSettings, SubCommand};

fn main() {
    let matches = App::new("Guessing Game")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("http")
                .about("Play via a HTTP server and web interface")
                .arg(
                    Arg::with_name("port")
                        .help("Port for the http server to listen on.")
                        .short("p")
                        .long("port")
                        .takes_value(true)
                        .value_name("PORT")
                )
        )
        .get_matches();

    match matches.subcommand() {
        ("http", Some(http_matches)) => commands::http::run(http_matches),
        _ =>                            {}
    };
}

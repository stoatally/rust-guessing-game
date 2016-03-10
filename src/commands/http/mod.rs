extern crate clap;
extern crate tiny_http;

use clap::{App, Arg, ArgMatches, SubCommand};
use game::{Game, GameError, ProvidesGuess, ProvidesGuessError};
use std::io::Error as IoError;
use tiny_http::{Server, Request, Response};

struct FormData {
    data:       String
}

impl FormData {
    pub fn new(request: &mut Request) -> FormData {
        match request.body_length() {
            Some(length) => {
                                let reader = request.as_reader();
                                let mut data = String::with_capacity(length);

                                reader.read_to_string(&mut data).unwrap();

                                FormData {
                                    data: data
                                }
                            },
            None =>         FormData {
                                data: String::from("")
                            }
        }
    }

    fn read_guess(&self) -> String {
        self.data.split('=')
            .map(String::from)
            .collect::<Vec<String>>()
            .last()
            .unwrap()
            .to_owned()
    }
}

impl ProvidesGuess for FormData {
    fn guess(&self) -> Result<String, ProvidesGuessError> {
        match self.data.is_empty() {
            true =>         Err(ProvidesGuessError::Empty),
            false =>        Ok(self.read_guess())
        }
    }
}

pub fn declare<'a, 'b>() -> App<'a, 'b> {
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
}

pub fn run(matches: &ArgMatches) {
    match matches.value_of("port") {
        Some(port) =>   parse_port_arg(port),
        None =>         run_on_port(9090)
    }
}

fn parse_port_arg(port: &str) {
    match port.to_string().parse::<u32>() {
        Err(_) =>       println!("Invalid value for --port."),
        Ok(port) =>     run_on_port(port)
    }
}

fn run_on_port(port: u32) {
    match server(port) {
        Err(_) =>       println!("Could not start port using --port {}.", port),
        Ok(_) =>        {}
    }
}

fn server(port: u32) -> Result<(), ()> {
    let mut game = Game::new();

    let server = match Server::http(format!("0.0.0.0:{}", port).as_str()) {
        Ok(server) =>   server,
        _ =>            return Err(())
    };

    println!("Game available on http://127.0.0.1:{}/.", port);

    for request in server.incoming_requests() {
        handle(&mut game, request);
    };

    Ok(())
}

fn handle(game: &mut Game, mut request: Request) {
    let mut html = String::new();

    match game.make_guess(&FormData::new(&mut request)) {
        Err(GameError::GuessNotMade) =>
                                {
                                    html.push_str("<p>Guess a number between 1 and 100:</p>");
                                },
        Err(GameError::GuessIsInvalid) =>
                                {
                                    html.push_str(format!("<p>You guessed &ldquo;{}&rdquo;, that isn't actually a number...</p>", game.guess()).as_str());
                                },
        Err(GameError::GuessIsLow) =>
                                {
                                    html.push_str(format!("<p>You guessed {}, which is too small...</p>", game.guess()).as_str());
                                },
        Err(GameError::GuessIsHigh) =>
                                {
                                    html.push_str(format!("<p>You guessed {}, which is too big...</p>", game.guess()).as_str());
                                },
        Ok(_) =>                {
                                    html.push_str("<p>You guessed the number correctly! Guess a new number between 1 and 100:</p>");
                                    game.reset()
                                }
    };

    html.push_str("<form method=\"post\"><input name=\"guess\" type=\"number\" /><button>Submit</button></form>");

    respond(request, html)
        .expect("Failed to send the response.");
}

fn respond(request: Request, html: String) -> Result<(), IoError> {
    let response = Response::from_string(html)
        .with_header(tiny_http::Header {
            field: "Content-Type".parse().unwrap(),
            value: "text/html".parse().unwrap()
        });

    let result = request.respond(response);

    result
}

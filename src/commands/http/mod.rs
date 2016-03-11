extern crate clap;
extern crate tiny_http;

use clap::{App, Arg, ArgMatches, SubCommand};
use game::{Game, GameState, ProvidesGuess, ProvidesGuessError};
use std::io::Error as IoError;
use tiny_http::{Server, Request, Response};

struct HttpData {
    data:       String
}

impl HttpData {
    pub fn new(request: &mut Request) -> HttpData {
        return match request.body_length() {
            Some(length) => {
                                let reader = request.as_reader();
                                let mut data = String::with_capacity(length);

                                reader.read_to_string(&mut data).unwrap();

                                HttpData {
                                    data: data
                                }
                            },
            None =>         HttpData {
                                data: String::from("")
                            }
        };
    }

    fn read_guess(&self) -> String {
        return self.data.split('=')
            .map(String::from)
            .collect::<Vec<String>>()
            .last()
            .unwrap()
            .to_owned();
    }
}

impl ProvidesGuess for HttpData {
    fn guess(&self) -> Result<String, ProvidesGuessError> {
        return match self.data.is_empty() {
            true =>         Err(ProvidesGuessError::Empty),
            false =>        Ok(self.read_guess())
        };
    }
}

struct HttpWorker {
    game:   Game
}

impl HttpWorker {
    fn new(game: Game) -> HttpWorker {
        return HttpWorker {
            game:   game
        };
    }

    fn handle(&mut self, mut request: Request) {
        let mut html = self.make_guess(HttpData::new(&mut request));

        html.push_str("<form method=\"post\"><input name=\"guess\" type=\"number\" /><button>Submit</button></form>");

        self.respond(request, html)
            .expect("Failed to send the response.");
    }

    fn make_guess(&mut self, data: HttpData) -> String {
        return match self.game.make_guess(&data) {
            GameState::GuessNotMade =>      self.guess_not_made(),
            GameState::GuessIsInvalid =>    self.guess_is_invalid(),
            GameState::GuessIsLow =>        self.guess_is_low(),
            GameState::GuessIsHigh =>       self.guess_is_high(),
            GameState::GuessWon =>          self.guess_won()
        };
    }

    fn guess_not_made(&mut self) -> String {
        return String::from("<p>Guess a number between 1 and 100:</p>");
    }

    fn guess_is_invalid(&mut self) -> String {
        return format!("<p>You guessed &ldquo;{}&rdquo;, that isn't actually a number...</p>", self.game.guess());
    }

    fn guess_is_low(&mut self) -> String {
        return format!("<p>You guessed {}, which is too small...</p>", self.game.guess());
    }

    fn guess_is_high(&mut self) -> String {
        return format!("<p>You guessed {}, which is too big...</p>", self.game.guess());
    }

    fn guess_won(&mut self) -> String {
        self.game.reset();

        return String::from("<p>You guessed the number correctly! Guess a new number between 1 and 100:</p>");
    }

    fn respond(&mut self, request: Request, html: String) -> Result<(), IoError> {
        let response = Response::from_string(html)
            .with_header(tiny_http::Header {
                field: "Content-Type".parse().unwrap(),
                value: "text/html".parse().unwrap()
            });

        return request.respond(response);
    }
}

struct HttpServer {
    port:   u32,
    server: Server
}

impl HttpServer {
    fn new(port: u32) -> Result<HttpServer, ()> {
        return match Server::http(format!("0.0.0.0:{}", port).as_str()) {
            Ok(server) =>   Ok(HttpServer {
                                port:   port,
                                server: server
                            }),
            _ =>            Err(())
        };
    }

    fn listen(&self, worker: &mut HttpWorker) {
        for request in self.server.incoming_requests() {
            worker.handle(request);
        }
    }
}

pub fn declare<'a, 'b>() -> App<'a, 'b> {
    return SubCommand::with_name("http")
        .about("Play via a HTTP server and web interface")
        .arg(
            Arg::with_name("port")
                .help("Port for the http server to listen on.")
                .short("p")
                .long("port")
                .takes_value(true)
                .value_name("PORT")
        );
}

pub fn run(matches: &ArgMatches) {
    return match matches.value_of("port") {
        Some(port) =>   parse_port_arg(port),
        None =>         run_on_port(9090)
    };
}

fn parse_port_arg(port: &str) {
    return match port.to_string().parse::<u32>() {
        Err(_) =>       println!("Invalid value for --port."),
        Ok(port) =>     run_on_port(port)
    };
}

fn run_on_port(port: u32) {
    return match HttpServer::new(port) {
        Err(_) =>       println!("Could not start port using --port {}.", port),
        Ok(server) =>   {
                            println!("Game available on http://127.0.0.1:{}/.", server.port);

                            server.listen(&mut HttpWorker::new(Game::new()));
                        }
    };
}

extern crate rand;

use rand::Rng;
use std::cmp::Ordering;

pub enum ProvidesGuessError {
    Empty
}

pub trait ProvidesGuess {
    fn guess(&self) -> Result<String, ProvidesGuessError>;
}

pub enum GameError {
    GuessNotMade,
    GuessIsInvalid,
    GuessIsLow,
    GuessIsHigh
}

pub struct Game {
    secret: u32,
    guess: String
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            secret: 0,
            guess: String::from("")
        };

        game.reset();

        game
    }

    pub fn reset(&mut self) {
        self.secret = rand::thread_rng().gen_range(1, 101);
    }

    pub fn guess(&self) -> String {
        self.guess.clone()
    }

    pub fn make_guess(&mut self, provider: &ProvidesGuess) -> Result<(), GameError> {
        self.guess = String::from("");

        match provider.guess() {
            Err(ProvidesGuessError::Empty) =>   Err(GameError::GuessNotMade),
            Ok(guess) =>                        self.parse_guess(&guess)
        }
    }

    fn parse_guess(&mut self, guess: &String) -> Result<(), GameError> {
        match guess.trim().parse::<u32>() {
            Ok(guess) =>                        self.compare_guess(&guess),
            Err(_) =>                           Err(GameError::GuessIsInvalid)
        }
    }

    fn compare_guess(&mut self, guess: &u32) -> Result<(), GameError> {
        self.guess = guess.to_string();

        match guess.cmp(&self.secret) {
            Ordering::Less =>                   Err(GameError::GuessIsLow),
            Ordering::Greater =>                Err(GameError::GuessIsHigh),
            Ordering::Equal =>                  Ok(())
        }
    }
}

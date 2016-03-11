extern crate rand;

use rand::Rng;
use std::cmp::Ordering;

pub enum ProvidesGuessError {
    Empty
}

pub trait ProvidesGuess {
    fn guess(&self) -> Result<String, ProvidesGuessError>;
}

pub enum GameState {
    GuessNotMade,
    GuessIsInvalid,
    GuessIsLow,
    GuessIsHigh,
    GuessWon
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

        return game;
    }

    pub fn reset(&mut self) {
        self.secret = rand::thread_rng().gen_range(1, 101);
    }

    pub fn guess(&self) -> String {
        self.guess.clone()
    }

    pub fn make_guess(&mut self, provider: &ProvidesGuess) -> GameState {
        self.guess = String::from("");

        return match provider.guess() {
            Err(ProvidesGuessError::Empty) =>   GameState::GuessNotMade,
            Ok(guess) =>                        self.parse_guess(&guess)
        };
    }

    fn parse_guess(&mut self, guess: &String) -> GameState {
        return match guess.trim().parse::<u32>() {
            Err(_) =>                           GameState::GuessIsInvalid,
            Ok(guess) =>                        self.compare_guess(&guess)
        };
    }

    fn compare_guess(&mut self, guess: &u32) -> GameState {
        self.guess = guess.to_string();

        return match guess.cmp(&self.secret) {
            Ordering::Less =>                   GameState::GuessIsLow,
            Ordering::Greater =>                GameState::GuessIsHigh,
            Ordering::Equal =>                  GameState::GuessWon
        };
    }
}

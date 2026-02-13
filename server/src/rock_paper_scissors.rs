use std::fmt;

use rand::RngExt;
use serde::{Deserialize, Serialize};

pub const WIN_SCORE: u8 = 3;
pub const LOSE_SCORE: u8 = 1; //subtracted

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub score: u8,
}

impl Game {
    pub fn new() -> Self {
        Game { score: 0 }
    }
    pub fn process_turn(&mut self, player_choice: Option) -> TurnResult {
        let opponent_choice = get_random(); // random opponent choice

        let result = match (player_choice, opponent_choice) {
            (Option::Rock, Option::Scissors)
            | (Option::Paper, Option::Rock)
            | (Option::Scissors, Option::Paper) => {
                self.score = self.score.saturating_add(WIN_SCORE);
                TurnResult::Win
            }
            (Option::Scissors, Option::Rock)
            | (Option::Rock, Option::Paper)
            | (Option::Paper, Option::Scissors) => {
                self.score = self.score.saturating_sub(LOSE_SCORE);
                TurnResult::Lose
            }
            _ => TurnResult::Draw,
        };

        result
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TurnResult {
    Win,
    Lose,
    Draw,
}

impl fmt::Display for TurnResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            TurnResult::Win => "You win!",
            TurnResult::Lose => "You lose!",
            TurnResult::Draw => "It's a draw!",
        };
        write!(f, "{}", text)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Option {
    Rock,
    Paper,
    Scissors,
}

fn get_random() -> Option {
    let mut rng = rand::rng();
    match rng.random_range(0..3) {
        0 => Option::Rock,
        1 => Option::Paper,
        _ => Option::Scissors,
    }
}

extern crate rand;
extern crate rayon;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use rayon::prelude::*;

use std::io::{stdin, stdout, Write};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Dice {
    max_pips: Vec<usize>,
    pips: Vec<usize>,
    roll_count: usize,
    rng: SmallRng,
}

impl Dice {
    pub fn new(max_pips: Vec<usize>, rolls: usize) -> Self {
        let num_dices = max_pips.len();
        Self {
            max_pips: max_pips,
            pips: vec![0, num_dices],
            roll_count: rolls,
            rng: SmallRng::seed_from_u64(98050120),
        }
    }

    pub fn roll(&mut self) {
        self.pips = self
            .max_pips
            .clone()
            .into_iter()
            .map(|max_pip| self.rng.gen_range(1..=max_pip))
            .collect();
    }

    pub fn pip_sum(&self) -> usize {
        self.pips.iter().sum()
    }

    pub fn rolls(&self) -> usize {
        self.roll_count
    }
}

fn run_games(num_games: usize) -> (usize, usize, usize) {
    (1..=num_games)
        .into_par_iter()
        .fold(
            || (Dice::new(vec![6, 6], 0), 0, 0),
            |(mut dice, win_count, lose_count), _| {
                dice.roll();
                let first_roll = dice.pip_sum();
                let win = if (first_roll == 7) || (first_roll == 11) {
                    true
                } else if (first_roll == 2) || (first_roll == 3) || (first_roll == 12) {
                    false
                } else {
                    loop {
                        dice.roll();
                        let roll = dice.pip_sum();
                        if roll == first_roll {
                            break true;
                        } else if roll == 7 {
                            break false;
                        }
                    }
                };
                if win {
                    (dice, win_count + 1, lose_count)
                } else {
                    (dice, win_count, lose_count + 1)
                }
            },
        )
        .map(|(dice, win_count, lose_count)| (dice.rolls(), win_count, lose_count))
        .reduce(
            || (0, 0, 0),
            |(total_roll, total_win, total_lose), (current_roll, current_win, current_lose)| {
                (
                    total_roll + current_roll,
                    total_win + current_win,
                    total_lose + current_lose,
                )
            },
        )
}

fn main() {
    print!("Enter # of games to play: ");
    let mut buffer = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut buffer).unwrap();
    let num_games: usize = buffer.trim().parse().expect("Unable to parse user input");

    let start = Instant::now();

    let (total_roll, total_win, total_lose) = run_games(num_games);

    let duration = start.elapsed();

    println!(
        r#"  Raw Wins/Lose = {total_win} / {total_lose}
  % Wins/Lose   = {win_percent} / {lose_percent}

  Dice Thrown   = {total_roll}
  Avg Dice/game = {avg_roll}

  Elapsed Time  = {time_sec} seconds
  Speed         = {speed} games/second"#,
        total_win = total_win,
        total_lose = total_lose,
        win_percent = ((total_win as f64) / (num_games as f64) * 100.0),
        lose_percent = ((total_lose as f64) / (num_games as f64) * 100.0),
        total_roll = total_roll,
        avg_roll = ((total_roll as f64) / (num_games as f64)),
        time_sec = duration.as_secs_f64(),
        speed = ((num_games as f64) / (duration.as_secs_f64()))
    );
}

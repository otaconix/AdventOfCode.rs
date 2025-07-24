use aoc_timing::trace::log_run;
use std::io;

#[derive(PartialEq, Eq, Clone, Copy)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

use RockPaperScissors::{Rock, Paper, Scissors};

impl RockPaperScissors {
    fn shape_score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn parse_opponent(str: &str) -> RockPaperScissors {
        match str {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!("Unexpected opponent hand: {str}"),
        }
    }

    fn parse_own(str: &str) -> RockPaperScissors {
        match str {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => panic!("Unexpected own hand: {str}"),
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

use Outcome::{Lose, Draw, Win};

impl Outcome {
    fn parse(str: &str) -> Self {
        match str {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Unknown outcome detected in input: {str}"),
        }
    }

    fn determine_needed_hand(&self, opponent_hand: &RockPaperScissors) -> RockPaperScissors {
        match self {
            Draw => *opponent_hand,
            Win => match opponent_hand {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
            Lose => match opponent_hand {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },
        }
    }
}

fn score(opponent_hand: &RockPaperScissors, own_hand: &RockPaperScissors) -> u32 {
    let shape_score = own_hand.shape_score();
    let win_score = match opponent_hand.cmp(own_hand) {
        std::cmp::Ordering::Less => 6,
        std::cmp::Ordering::Equal => 3,
        std::cmp::Ordering::Greater => 0,
    };

    shape_score + win_score
}

impl Ord for RockPaperScissors {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::{Equal, Less, Greater};

        match (self, other) {
            (Rock, Rock) => Equal,
            (Rock, Paper) => Less,
            (Rock, Scissors) => Greater,
            (Paper, Rock) => Greater,
            (Paper, Paper) => Equal,
            (Paper, Scissors) => Less,
            (Scissors, Rock) => Less,
            (Scissors, Paper) => Greater,
            (Scissors, Scissors) => Equal,
        }
    }
}

impl PartialOrd for RockPaperScissors {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    env_logger::init();

    let (silver, gold) = log_run("Both parts", || {
        io::stdin()
            .lines()
            .map(|result| result.expect("I/O error"))
            .map(|line| {
                let hands: Vec<&str> = line.split_whitespace().collect();
                assert_eq!(hands.len(), 2, "Hands per line must be two!");

                let opponent = RockPaperScissors::parse_opponent(hands[0]);
                let own_silver = RockPaperScissors::parse_own(hands[1]);
                let outcome = Outcome::parse(hands[1]);
                let own_gold = outcome.determine_needed_hand(&opponent);

                (score(&opponent, &own_silver), score(&opponent, &own_gold))
            })
            .fold((0u32, 0u32), |(silver, gold), score| {
                (silver + score.0, gold + score.1)
            })
    });

    println!("Silver: {silver}");
    println!("Gold: {gold}");
}

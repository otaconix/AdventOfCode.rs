use pom::utf8::*;
use std::{io, str::FromStr};

#[derive(Debug)]
struct ScratchCard {
    id: u8,
    winning_numbers: Vec<u8>,
    scratched_numbers: Vec<u8>,
}

impl FromStr for ScratchCard {
    type Err = pom::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = || {
            sym(' ').repeat(0..)
                * is_a(|c| c.is_ascii_digit())
                    .repeat(1..)
                    .collect()
                    .convert(|digits| digits.parse::<u8>())
        };
        let id = seq("Card ") * call(number) - seq(": ");
        let number_list = || list(call(number), sym(' '));
        let scratch_card = (id + (call(number_list) - seq(" | ")) + call(number_list)).map(
            |((id, winning_numbers), scratched_numbers)| ScratchCard {
                id,
                winning_numbers,
                scratched_numbers,
            },
        );

        scratch_card.parse(s.as_bytes())
    }
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| line.parse().expect("Parse error"))
        .collect::<Vec<ScratchCard>>();

    let part_1 = input
        .iter()
        .map(|scratch_card| {
            match scratch_card
                .scratched_numbers
                .iter()
                .filter(|scratched_number| scratch_card.winning_numbers.contains(scratched_number))
                .count()
            {
                0 => 0,
                count => 2u32.pow(count as u32 - 1),
            }
        })
        .sum::<u32>();

    println!("Part 1: {part_1}");
}

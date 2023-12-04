use pom::utf8::*;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
struct ScratchCard {
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
        let scratch_card = (id * (call(number_list) - seq(" | ")) + call(number_list)).map(
            |(winning_numbers, scratched_numbers)| ScratchCard {
                winning_numbers,
                scratched_numbers,
            },
        );

        scratch_card.parse(s.as_bytes())
    }
}

impl ScratchCard {
    fn winning_numbers_count(&self) -> usize {
        self.scratched_numbers
            .iter()
            .filter(|scratched_number| self.winning_numbers.contains(scratched_number))
            .count()
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
        .map(|scratch_card| match scratch_card.winning_numbers_count() {
            0 => 0,
            count => 2u32.pow(count as u32 - 1),
        })
        .sum::<u32>();

    println!("Part 1: {part_1}");

    let mut card_counts = std::iter::repeat(1).take(input.len()).collect::<Vec<_>>();

    for index in 0..input.len() {
        if card_counts[index] == 0 {
            break;
        }

        let winners = input[index].winning_numbers_count();

        for to_add in ((index + 1)..).take(winners) {
            card_counts[to_add] += card_counts[index];
        }
    }

    let part_2 = card_counts.iter().sum::<usize>();

    println!("Part 2: {part_2}");
}

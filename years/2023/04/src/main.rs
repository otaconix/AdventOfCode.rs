use aoc_timing::trace::log_run;
use nom::bytes::complete::tag;
use nom::character::complete::{char, space1, u8};
use nom::combinator::all_consuming;
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded, terminated};
use nom::{IResult, Parser};
use std::io;

#[derive(Debug)]
struct ScratchCard {
    winning_numbers: Vec<u8>,
    scratched_numbers: Vec<u8>,
}

impl ScratchCard {
    fn parser(input: &str) -> IResult<&str, Self> {
        let id = preceded(tag("Card").and(space1), u8);
        let numbers = || separated_list1(space1, u8);

        (
            terminated(id, pair(char(':'), space1)),
            numbers(),
            preceded(tag(" |").and(space1), all_consuming(numbers())),
        )
            .map(|(_, winning_numbers, scratched_numbers)| ScratchCard {
                winning_numbers,
                scratched_numbers,
            })
            .parse(input)
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

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Vec<ScratchCard> {
    input
        .map(|line| ScratchCard::parser(line.as_ref()).expect("Parse error").1)
        .collect::<Vec<ScratchCard>>()
}

fn part_1(input: &[ScratchCard]) -> u32 {
    input
        .iter()
        .map(|scratch_card| match scratch_card.winning_numbers_count() {
            0 => 0,
            count => 2u32.pow(count as u32 - 1),
        })
        .sum()
}

fn part_2(input: &[ScratchCard]) -> usize {
    let mut card_counts = std::iter::repeat_n(1, input.len()).collect::<Vec<_>>();

    for index in 0..input.len() {
        if card_counts[index] == 0 {
            break;
        }

        let winners = input[index].winning_numbers_count();

        for to_add in ((index + 1)..).take(winners) {
            card_counts[to_add] += card_counts[index];
        }
    }

    card_counts.iter().sum()
}

fn main() {
    env_logger::init();
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = log_run("Part 1", || part_1(&input));
    println!("Part 1: {part_1}");

    let part_2 = log_run("Part 2", || part_2(&input));
    println!("Part 2: {part_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_1() {
        let input = parse(&mut INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_2() {
        let input = parse(&mut INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 30);
    }
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;

use aoc_macros::EnumVariants;
use aoc_timing::trace::log_run;
use aoc_utils::EnumVariants;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumVariants, Clone, Copy)]
enum CardType {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone)]
struct Hand {
    types: Vec<CardType>,
    counts: Vec<u8>,
}

fn compare_part_1(a: &Bid, b: &Bid) -> Ordering {
    a.card
        .counts
        .cmp(&b.card.counts)
        .then_with(|| a.card.types.cmp(&b.card.types))
}

fn compare_part_2(a: &Bid, b: &Bid) -> Ordering {
    a.card.counts.cmp(&b.card.counts).then_with(|| {
        a.card
            .types
            .iter()
            .zip(b.card.types.iter())
            .map(|types| match types {
                (CardType::Jack, CardType::Jack) => Ordering::Equal,
                (CardType::Jack, _) => Ordering::Less,
                (_, CardType::Jack) => Ordering::Greater,
                (type_a, type_b) => type_a.cmp(type_b),
            })
            .find(|ord| ord.is_ne())
            .unwrap_or(Ordering::Equal)
    })
}

impl Hand {
    fn new(types: Vec<CardType>) -> Self {
        Hand {
            counts: {
                let mut sorted = CardType::variants()
                    .into_iter()
                    .map(|card_type| types.iter().filter(|t| t == &&card_type).count() as u8)
                    .filter(|count| *count != 0)
                    .collect::<BinaryHeap<_>>()
                    .into_sorted_vec();
                sorted.reverse();

                sorted
            },
            types,
        }
    }

    fn with_jokers(&self) -> Self {
        Hand {
            counts: {
                let mut sorted = CardType::variants()
                    .into_iter()
                    .filter(|card_type| card_type != &CardType::Jack)
                    .map(|card_type| self.types.iter().filter(|t| t == &&card_type).count() as u8)
                    .filter(|count| *count != 0)
                    .collect::<BinaryHeap<_>>()
                    .into_sorted_vec();
                sorted.reverse();

                let joker_count = self.types.iter().filter(|t| t == &&CardType::Jack).count() as u8;

                if sorted.is_empty() {
                    sorted = vec![5];
                } else {
                    sorted[0] += joker_count;
                }

                sorted
            },
            types: self.types.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Bid {
    card: Hand,
    amount: u32,
}

impl Bid {
    fn with_jokers(&self) -> Self {
        Bid {
            amount: self.amount,
            card: self.card.with_jokers(),
        }
    }
}

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Vec<Bid> {
    use CardType::*;
    input
        .map(|line| line.to_string())
        .map(|line| {
            let (raw_types, amount) = line.split_once(' ').expect("No space in line");
            let types = raw_types
                .chars()
                .map(|raw_type| match raw_type {
                    '2' => Two,
                    '3' => Three,
                    '4' => Four,
                    '5' => Five,
                    '6' => Six,
                    '7' => Seven,
                    '8' => Eight,
                    '9' => Nine,
                    'T' => Ten,
                    'J' => Jack,
                    'Q' => Queen,
                    'K' => King,
                    'A' => Ace,
                    _ => panic!("Unexpected card type: {raw_type}"),
                })
                .collect();

            Bid {
                card: Hand::new(types),
                amount: amount.parse().expect("Invalid bid amount"),
            }
        })
        .collect()
}

fn part_1(input: &[Bid]) -> usize {
    let mut input = input.to_vec();
    input.sort_unstable_by(compare_part_1);

    input
        .iter()
        .enumerate()
        .map(|(index, bid)| bid.amount as usize * (index + 1))
        .sum()
}

fn part_2(input: &[Bid]) -> usize {
    let mut input = input.iter().map(Bid::with_jokers).collect::<Vec<_>>();
    input.sort_unstable_by(compare_part_2);

    input
        .iter()
        .enumerate()
        .map(|(index, bid)| bid.amount as usize * (index + 1))
        .sum()
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
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 6440);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 5905);
    }
}

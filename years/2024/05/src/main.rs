use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

use aoc_timing::trace::log_run;

#[derive(Debug, Clone, Copy)]
struct PageOrderingPair {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq)]
struct Update {
    pages: Vec<u32>,
}

impl Update {
    /// Checks whether this update is already sorted.
    ///
    /// This works by basically checking every page `n`, if there is a page `m` further down in the
    /// update that has `n` as one of its _nexts_. If an `m` is found, this update isn't sorted.
    /// Otherwise it is sorted.
    ///
    /// This makes use of [LaunchSafetyManual#page_order]
    fn is_sorted(&self, input: &Input) -> bool {
        for (i, n) in self.pages.iter().enumerate() {
            for m in &self.pages[i + 1..] {
                match input.page_order.get(m) {
                    Some(nexts) if nexts.contains(n) => return false,
                    _ => {}
                }
            }
        }

        true
    }

    /// Returns a sorted copy of this update.
    ///
    /// Algorithm:
    /// 1. Start with a copy that only contains the first page of this update
    /// 2. For every next page `n`, find the first page in the sorted copy that must come after, and
    ///    insert `n` before it. If none can be found, add it at the end.
    fn sort(&self, input: &Input) -> Self {
        let mut sorted = Vec::with_capacity(self.pages.len());

        sorted.push(self.pages[0]);

        for n in &self.pages[1..] {
            if let Some(nexts) = input.page_order.get(n) {
                if let Some((i, _)) = sorted.iter().enumerate().find(|(_, m)| nexts.contains(m)) {
                    sorted.insert(i, *n);
                } else {
                    sorted.push(*n);
                }
            } else {
                sorted.push(*n);
            }
        }

        Self { pages: sorted }
    }
}

#[derive(Debug)]
struct LaunchSafetyManual {
    page_order: HashMap<u32, HashSet<u32>>,
    updates: Vec<Update>,
}

enum ParsingState {
    OrderingPairs(Vec<PageOrderingPair>),
    Updates(LaunchSafetyManual),
}

type Input = LaunchSafetyManual;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let parsing_state = input.fold(ParsingState::OrderingPairs(vec![]), |mut state, line| {
        let line = line.as_ref();

        if line.is_empty() {
            ParsingState::Updates(LaunchSafetyManual {
                page_order: match state {
                    ParsingState::OrderingPairs(pairs) => {
                        pairs
                            .iter()
                            .fold(HashMap::new(), |mut map, PageOrderingPair { x, y }| {
                                if let Some(nexts) = map.get_mut(x) {
                                    nexts.insert(*y);
                                } else {
                                    map.insert(*x, HashSet::from([*y]));
                                }

                                map
                            })
                    }
                    _ => panic!("In wrong parsing state when encountering empty line"),
                },
                updates: vec![],
            })
        } else {
            match state {
                ParsingState::OrderingPairs(ref mut pairs) => {
                    let (x, y) = line.split_once('|').unwrap();
                    pairs.push(PageOrderingPair {
                        x: x.parse().unwrap(),
                        y: y.parse().unwrap(),
                    });
                }
                ParsingState::Updates(ref mut manual) => {
                    manual.updates.push(Update {
                        pages: line.split(',').map(|page| page.parse().unwrap()).collect(),
                    });
                }
            }

            state
        }
    });

    match parsing_state {
        ParsingState::OrderingPairs(_) => {
            panic!("Still in ordering pairs state when done parsing last line")
        }
        ParsingState::Updates(manual) => manual,
    }
}

fn part_1(input: &Input) -> u32 {
    input
        .updates
        .iter()
        .filter(|update| update.is_sorted(input))
        .map(|update| update.pages[update.pages.len() / 2])
        .sum()
}

fn part_2(input: &Input) -> u32 {
    input
        .updates
        .iter()
        .filter(|update| !update.is_sorted(input))
        .map(|update| update.sort(input))
        .map(|update| update.pages[update.pages.len() / 2])
        .sum()
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        let part_1 = log_run("Part 1", || part_1(&input));
        println!("Part 1: {part_1}");

        let part_2 = log_run("Part 2", || part_2(&input));
        println!("Part 2: {part_2}");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 143);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 123);
    }
}

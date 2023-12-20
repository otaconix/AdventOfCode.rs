use std::{collections::HashMap, io, iter::successors};

use aoc_timing::trace::log_run;
use ranges::Ranges;

struct Input {
    rules: HashMap<String, Rule>,
    parts: Vec<Part>,
}

impl Input {
    const START_RULE: &'static str = "in";

    fn get_chains_leading_to_accept(&self) -> Vec<Vec<Condition>> {
        fn inner(
            input: &Input,
            chain: &mut Vec<Condition>,
            destination: &Destination,
        ) -> Vec<Vec<Condition>> {
            match destination {
                Destination::Reject => vec![],
                Destination::Accept => vec![chain.clone()],
                Destination::NextRule(next) => {
                    let rule = &input.rules[next];

                    let mut pushed_inversions = 0;
                    let mut results = vec![];

                    for ConditionalDestination {
                        condition,
                        destination,
                    } in &rule.conditions
                    {
                        if !matches!(condition, Condition::Unconditional)
                            && matches!(destination, Destination::Reject)
                        {
                            chain.push(condition.invert());
                            pushed_inversions += 1;
                        } else {
                            chain.push(*condition);
                            results.extend(inner(input, chain, destination));
                            chain.pop();
                        }
                    }

                    for _ in 0..pushed_inversions {
                        chain.pop();
                    }

                    results
                }
            }
        }

        let mut chain = vec![];
        inner(
            self,
            &mut chain,
            &Destination::NextRule(Self::START_RULE.to_string()),
        )
    }
}

enum ParsingState {
    Rules(HashMap<String, Rule>),
    Parts(HashMap<String, Rule>, Vec<Part>),
}

#[derive(Clone)]
struct Rule {
    conditions: Vec<ConditionalDestination>,
}

impl Rule {
    fn execute(&self, part: &Part) -> Destination {
        self.conditions
            .iter()
            .find_map(|condition| condition.get_destination(part))
            .expect("No condition matched")
    }
}

#[derive(Clone, Debug)]
struct ConditionalDestination {
    condition: Condition,
    destination: Destination,
}

impl ConditionalDestination {
    fn get_destination(&self, part: &Part) -> Option<Destination> {
        match &self.condition {
            Condition::Unconditional => Some(self.destination.clone()),
            Condition::LessThan(target, n) if part.get_target(target) < *n => {
                Some(self.destination.clone())
            }
            Condition::GreaterThan(target, n) if part.get_target(target) > *n => {
                Some(self.destination.clone())
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Destination {
    Accept,
    Reject,
    NextRule(String),
}

#[derive(Clone, Copy, Debug)]
enum Condition {
    LessThan(Target, usize),
    GreaterThan(Target, usize),
    Unconditional,
}

impl Condition {
    fn invert(&self) -> Condition {
        use Condition::*;

        match *self {
            LessThan(target, n) => GreaterThan(target, n - 1),
            GreaterThan(target, n) => LessThan(target, n + 1),
            Unconditional => panic!("No need to invert unconditionals"),
        }
    }

    fn restrict_part_ranges(&self, ranges: &mut [Ranges<usize>; 4]) {
        match self {
            Condition::Unconditional => (),
            Condition::LessThan(target, n) => {
                ranges[*target as usize] &= (1..*n).into();
            }
            Condition::GreaterThan(target, n) => {
                ranges[*target as usize] &= (n + 1..).into();
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Target {
    X,
    M,
    A,
    S,
}

impl TryFrom<&str> for Target {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "x" => Ok(Target::X),
            "m" => Ok(Target::M),
            "a" => Ok(Target::A),
            "s" => Ok(Target::S),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn get_target(&self, target: &Target) -> usize {
        use Target::*;

        match target {
            X => self.x,
            M => self.m,
            A => self.a,
            S => self.s,
        }
    }

    fn sum_of_targets(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Input {
    use ParsingState::*;

    let end_state =
        input
            .map(|line| line.to_string())
            .fold(Rules(HashMap::new()), |state, line| match state {
                Rules(mut rules) => {
                    if line.is_empty() {
                        Parts(rules, vec![])
                    } else {
                        let (name, rest) = line.split_once('{').unwrap();
                        let (rest, _) = rest.split_once('}').unwrap();
                        let conditions = rest
                            .split(',')
                            .map(|conditional_destination| -> ConditionalDestination {
                                if let Some((condition, destination)) =
                                    conditional_destination.split_once(':')
                                {
                                    if let Some((target, n)) = condition.split_once('<') {
                                        ConditionalDestination {
                                            condition: Condition::LessThan(
                                                target.try_into().unwrap(),
                                                n.parse().unwrap(),
                                            ),
                                            destination: match destination {
                                                "A" => Destination::Accept,
                                                "R" => Destination::Reject,
                                                _ => Destination::NextRule(destination.to_string()),
                                            },
                                        }
                                    } else {
                                        let (target, n) = condition.split_once('>').unwrap();
                                        ConditionalDestination {
                                            condition: Condition::GreaterThan(
                                                target.try_into().unwrap(),
                                                n.parse().unwrap(),
                                            ),
                                            destination: match destination {
                                                "A" => Destination::Accept,
                                                "R" => Destination::Reject,
                                                _ => Destination::NextRule(destination.to_string()),
                                            },
                                        }
                                    }
                                } else {
                                    ConditionalDestination {
                                        condition: Condition::Unconditional,
                                        destination: match conditional_destination {
                                            "A" => Destination::Accept,
                                            "R" => Destination::Reject,
                                            _ => Destination::NextRule(
                                                conditional_destination.to_string(),
                                            ),
                                        },
                                    }
                                }
                            })
                            .collect();

                        let new_rule = Rule { conditions };

                        rules.insert(name.to_string(), new_rule);

                        Rules(rules)
                    }
                }
                Parts(rules, mut parts) => {
                    let mut part = Part::default();
                    line[1..line.len() - 1].split(',').for_each(|component| {
                        let (target, value) = component.split_once('=').unwrap();
                        let value = value.parse().unwrap();

                        match target {
                            "x" => part.x = value,
                            "m" => part.m = value,
                            "a" => part.a = value,
                            "s" => part.s = value,
                            _ => panic!("Unexpected part target {target}"),
                        }
                    });
                    parts.push(part);

                    Parts(rules, parts)
                }
            });

    match end_state {
        Rules(_) => panic!("Didn't reach Parts parsing stage"),
        Parts(rules, parts) => Input { rules, parts },
    }
}

fn part_1(input: &Input) -> usize {
    input
        .parts
        .iter()
        .filter(|part| {
            successors(
                Some(Destination::NextRule(Input::START_RULE.to_string())),
                |prev| match prev {
                    Destination::Reject | Destination::Accept => None,
                    Destination::NextRule(rule_name) => Some(input.rules[rule_name].execute(part)),
                },
            )
            .any(|destination| matches!(destination, Destination::Accept))
        })
        .map(Part::sum_of_targets)
        .sum()
}

fn part_2(input: &Input) -> usize {
    let chains = input.get_chains_leading_to_accept();

    chains
        .iter()
        .inspect(|chain| println!("{chain:?}"))
        .map(|chain| {
            chain
                .iter()
                .fold(
                    [
                        Ranges::from(1..=4000),
                        Ranges::from(1..=4000),
                        Ranges::from(1..=4000),
                        Ranges::from(1..=4000),
                    ],
                    |mut ranges, condition| {
                        condition.restrict_part_ranges(&mut ranges);
                        ranges
                    },
                )
                .into_iter()
                .map(|range| {
                    range
                        .as_slice()
                        .iter()
                        .inspect(|x| println!("{x}"))
                        .map(|slice| slice.into_iter().count())
                        .sum::<usize>()
                })
                .product::<usize>()
        })
        .inspect(|x| println!("Cardinality {x}"))
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

        let part_2 = log_run("Part 1", || part_2(&input));
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

        assert_eq!(result, 19114);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 167409079868000);
    }
}

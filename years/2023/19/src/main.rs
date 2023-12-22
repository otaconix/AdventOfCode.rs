use std::borrow::Borrow;
use std::fmt::Debug;
use std::io;
use std::iter::successors;
use std::ops::RangeBounds;
use std::{collections::HashMap, ops::Index};

use aoc_timing::trace::log_run;
use ranges::{GenericRange, Ranges};

struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Input {
    const START_WORKFLOW: &'static str = "in";

    fn get_chains_leading_to_accept(&self) -> Vec<Vec<Condition>> {
        fn inner(
            input: &Input,
            chain: &mut Vec<Condition>,
            destination: &Destination,
            condition_index: usize, // always 0, unless we just inverted a condition
        ) -> Vec<Vec<Condition>> {
            match destination {
                Destination::Reject => vec![],
                Destination::Accept => vec![chain.clone()],
                Destination::NextWorkflow(next) => {
                    let workflow = &input.workflows[next];

                    let mut results = vec![];

                    if let Some(ConditionalDestination {
                        condition,
                        destination: next_destination,
                    }) = workflow.conditions.get(condition_index)
                    {
                        if !matches!(condition, Condition::Unconditional) {
                            chain.push(*condition);
                            results.extend(inner(input, chain, next_destination, 0));
                            chain.pop();

                            chain.push(condition.invert());
                            results.extend(inner(input, chain, destination, condition_index + 1));
                            chain.pop();
                        } else {
                            results.extend(inner(input, chain, next_destination, 0));
                        }
                    }

                    results
                }
            }
        }

        let mut chain = vec![];
        inner(
            self,
            &mut chain,
            &Destination::NextWorkflow(Self::START_WORKFLOW.to_string()),
            0,
        )
    }
}

enum ParsingState {
    Workflows(HashMap<String, Workflow>),
    Parts(HashMap<String, Workflow>, Vec<Part>),
}

struct Workflow {
    conditions: Vec<ConditionalDestination>,
}

impl Workflow {
    fn execute(&self, part: &Part) -> Destination {
        self.conditions
            .iter()
            .find_map(|condition| condition.get_destination(part))
            .expect("No condition matched")
    }
}

struct ConditionalDestination {
    condition: Condition,
    destination: Destination,
}

impl ConditionalDestination {
    fn get_destination(&self, part: &Part) -> Option<Destination> {
        match &self.condition {
            Condition::Unconditional => Some(self.destination.clone()),
            Condition::LessThan(category, n) if part[category] < *n => {
                Some(self.destination.clone())
            }
            Condition::GreaterThan(category, n) if part[category] > *n => {
                Some(self.destination.clone())
            }
            _ => None,
        }
    }
}

#[derive(Clone)]
enum Destination {
    Accept,
    Reject,
    NextWorkflow(String),
}

#[derive(Clone, Copy)]
enum Condition {
    LessThan(Category, usize),
    GreaterThan(Category, usize),
    Unconditional,
}

impl Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::LessThan(category, n) | Condition::GreaterThan(category, n) => {
                f.write_fmt(format_args!(
                    "{category:?}{}{n}",
                    if matches!(self, Condition::LessThan(_, _)) {
                        '<'
                    } else {
                        '>'
                    }
                ))
            }
            Condition::Unconditional => f.write_str("*=*"),
        }
    }
}

impl Condition {
    fn invert(&self) -> Condition {
        use Condition::*;

        match *self {
            LessThan(category, n) => GreaterThan(category, n - 1),
            GreaterThan(category, n) => LessThan(category, n + 1),
            Unconditional => panic!("No need to invert unconditionals"),
        }
    }

    fn restrict_part_ranges(&self, ranges: &mut [Ranges<usize>; 4]) {
        match self {
            Condition::Unconditional => (),
            Condition::LessThan(category, n) => {
                ranges[*category as usize] &= (1..*n).into();
            }
            Condition::GreaterThan(category, n) => {
                ranges[*category as usize] &= (n + 1..).into();
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

impl TryFrom<&str> for Category {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Category::*;

        match value {
            "x" => Ok(X),
            "m" => Ok(M),
            "a" => Ok(A),
            "s" => Ok(S),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
struct Part {
    x_rating: usize,
    m_rating: usize,
    a_rating: usize,
    s_rating: usize,
}

impl Part {
    fn sum_of_ratings(&self) -> usize {
        self.x_rating + self.m_rating + self.a_rating + self.s_rating
    }
}

impl<T: Borrow<Category>> Index<T> for Part {
    type Output = usize;

    fn index(&self, category: T) -> &Self::Output {
        use Category::*;

        match category.borrow() {
            X => &self.x_rating,
            M => &self.m_rating,
            A => &self.a_rating,
            S => &self.s_rating,
        }
    }
}

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Input {
    use ParsingState::*;

    let end_state = input.map(|line| line.to_string()).fold(
        Workflows(HashMap::new()),
        |state, line| match state {
            Workflows(mut workflows) => {
                if line.is_empty() {
                    Parts(workflows, vec![])
                } else {
                    let (name, rest) = line.split_once('{').unwrap();
                    let (rest, _) = rest.split_once('}').unwrap();
                    let conditions = rest
                        .split(',')
                        .map(|conditional_destination| -> ConditionalDestination {
                            if let Some((condition, destination)) =
                                conditional_destination.split_once(':')
                            {
                                if let Some((category, n)) = condition.split_once('<') {
                                    ConditionalDestination {
                                        condition: Condition::LessThan(
                                            category.try_into().unwrap(),
                                            n.parse().unwrap(),
                                        ),
                                        destination: match destination {
                                            "A" => Destination::Accept,
                                            "R" => Destination::Reject,
                                            _ => Destination::NextWorkflow(destination.to_string()),
                                        },
                                    }
                                } else {
                                    let (category, n) = condition.split_once('>').unwrap();
                                    ConditionalDestination {
                                        condition: Condition::GreaterThan(
                                            category.try_into().unwrap(),
                                            n.parse().unwrap(),
                                        ),
                                        destination: match destination {
                                            "A" => Destination::Accept,
                                            "R" => Destination::Reject,
                                            _ => Destination::NextWorkflow(destination.to_string()),
                                        },
                                    }
                                }
                            } else {
                                ConditionalDestination {
                                    condition: Condition::Unconditional,
                                    destination: match conditional_destination {
                                        "A" => Destination::Accept,
                                        "R" => Destination::Reject,
                                        _ => Destination::NextWorkflow(
                                            conditional_destination.to_string(),
                                        ),
                                    },
                                }
                            }
                        })
                        .collect();

                    let new_workflow = Workflow { conditions };

                    workflows.insert(name.to_string(), new_workflow);

                    Workflows(workflows)
                }
            }
            Parts(workflows, mut parts) => {
                let mut part = Part::default();
                line[1..line.len() - 1].split(',').for_each(|component| {
                    let (category, rating) = component.split_once('=').unwrap();
                    let rating = rating.parse().unwrap();

                    match category {
                        "x" => part.x_rating = rating,
                        "m" => part.m_rating = rating,
                        "a" => part.a_rating = rating,
                        "s" => part.s_rating = rating,
                        _ => panic!("Unexpected part category {category}"),
                    }
                });
                parts.push(part);

                Parts(workflows, parts)
            }
        },
    );

    match end_state {
        Workflows(_) => panic!("Didn't reach Parts parsing stage"),
        Parts(workflows, parts) => Input { workflows, parts },
    }
}

trait RangeLen {
    fn range_len(&self) -> usize;
}

impl RangeLen for GenericRange<usize> {
    fn range_len(&self) -> usize {
        let start = match self.start_bound() {
            std::ops::Bound::Included(n) => *n,
            std::ops::Bound::Excluded(n) => n + 1,
            std::ops::Bound::Unbounded => {
                panic!("Length of range with unbounded start unsupported")
            }
        };

        let end = match self.end_bound() {
            std::ops::Bound::Included(n) => n + 1,
            std::ops::Bound::Excluded(n) => *n,
            std::ops::Bound::Unbounded => panic!("Length of range with unbounded end unsupported"),
        };

        if start >= end {
            0
        } else {
            end - start
        }
    }
}

impl RangeLen for Ranges<usize> {
    fn range_len(&self) -> usize {
        self.as_slice().iter().map(RangeLen::range_len).sum()
    }
}

fn part_1(input: &Input) -> usize {
    input
        .parts
        .iter()
        .filter(|part| {
            successors(
                Some(Destination::NextWorkflow(Input::START_WORKFLOW.to_string())),
                |prev| match prev {
                    Destination::Reject | Destination::Accept => None,
                    Destination::NextWorkflow(workflow_name) => {
                        Some(input.workflows[workflow_name].execute(part))
                    }
                },
            )
            .any(|destination| matches!(destination, Destination::Accept))
        })
        .map(Part::sum_of_ratings)
        .sum()
}

fn part_2(input: &Input) -> usize {
    let chains = log_run("Determine chains", || input.get_chains_leading_to_accept());

    type Possibilities = [Ranges<usize>; 4];

    fn cardinality(possibilities: &Possibilities) -> usize {
        possibilities
            .iter()
            .map(RangeLen::range_len)
            .product::<usize>()
    }

    chains
        .iter()
        .map(|chain| {
            chain.iter().fold(
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
        })
        .map(|range| cardinality(&range))
        .sum::<usize>()
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

        assert_eq!(result, 19114);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 167409079868000);
    }
}

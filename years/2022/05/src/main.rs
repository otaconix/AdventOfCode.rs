use std::io;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Crate {
    identifier: char,
}

impl FromStr for Crate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char_slice = &s.chars().collect::<Vec<_>>()[..];

        match char_slice {
            ['[', id, ']', ..] => Ok(Crate { identifier: *id }),
            [' ', ' ', ' ', ..] => Ok(Crate { identifier: ' ' }),
            _ => Err(format!("Couldn't parse crate: {}", s)),
        }
    }
}

#[derive(Debug)]
struct RearrangementStep {
    count: usize,
    from_stack: usize,
    to_stack: usize,
}

impl FromStr for RearrangementStep {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [count, from_stack, to_stack] = s
            .split_ascii_whitespace()
            .filter_map(|word| word.parse::<usize>().ok())
            .collect::<Vec<_>>()
            .as_slice()
        {
            Ok(RearrangementStep {
                count: *count,
                from_stack: *from_stack,
                to_stack: *to_stack,
            })
        } else {
            Err(format!("Couldn't parse rearrangement step: {}", s))
        }
    }
}

enum InputParsing {
    Start,
    Crates(Vec<Vec<Crate>>),
    RearrangementSteps(Vec<Vec<Crate>>, Vec<RearrangementStep>),
}

impl Into<Input> for InputParsing {
    fn into(self) -> Input {
        match self {
            InputParsing::RearrangementSteps(crates, steps) => Input {
                stacks: crates
                    .into_iter()
                    .map(|stack| stack.into_iter().rev().collect())
                    .collect(),
                rearrangement_procedure: steps,
            },
            _ => panic!("`into` called on wrong stage of the parsing state"),
        }
    }
}

type Stack = Vec<Crate>;

#[derive(Debug)]
struct Input {
    stacks: Vec<Stack>,
    rearrangement_procedure: Vec<RearrangementStep>,
}

fn apply_rearrangement_step<F>(
    stacks: &mut Vec<Stack>,
    rearrangement_step: &RearrangementStep,
    f: F,
) where
    F: FnOnce(Vec<Crate>) -> Vec<Crate>,
{
    let from_stack = stacks
        .get_mut(rearrangement_step.from_stack - 1)
        .expect("Invalid 'from' stack");
    let mut stripped = f(from_stack.split_off(from_stack.len() - rearrangement_step.count));
    let to_stack = stacks
        .get_mut(rearrangement_step.to_stack - 1)
        .expect("Invalid 'to' stack");

    to_stack.append(&mut stripped);
}

fn main() {
    let input: Input = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .filter(|line| !line.is_empty())
        .fold(InputParsing::Start, |parsing, line| match parsing {
            InputParsing::Start => InputParsing::Crates(
                line.chars()
                    .collect::<Vec<_>>()
                    .chunks(4)
                    .map(|raw_crate| {
                        if let Ok(crate_) =
                            raw_crate.into_iter().collect::<String>().parse::<Crate>()
                        {
                            vec![crate_]
                        } else {
                            vec![]
                        }
                    })
                    .collect(),
            ),
            InputParsing::Crates(crates) => {
                if let Ok(new_crates) = line
                    .chars()
                    .collect::<Vec<_>>()
                    .chunks(4)
                    .map(|raw_crate| raw_crate.into_iter().collect::<String>().parse::<Crate>())
                    .collect::<Result<Vec<_>, _>>()
                {
                    InputParsing::Crates(
                        crates
                            .into_iter()
                            .zip(new_crates)
                            .map(|(mut stack, new_crate)| {
                                stack.push(new_crate);
                                stack
                            })
                            .collect(),
                    )
                } else {
                    InputParsing::RearrangementSteps(crates, vec![])
                }
            }
            InputParsing::RearrangementSteps(crates, mut steps) => {
                InputParsing::RearrangementSteps(crates, {
                    steps.push(line.parse().unwrap());
                    steps
                })
            }
        })
        .into();

    let mut stacks = input.stacks.clone();
    for step in input.rearrangement_procedure.iter() {
        apply_rearrangement_step(&mut stacks, &step, |crates| {
            crates.into_iter().rev().collect()
        });
    }

    let part_1 = stacks
        .into_iter()
        .filter_map(|stack| stack.into_iter().last())
        .map(|crate_| crate_.identifier)
        .collect::<String>();

    println!("Part 1: {}", part_1);

    let mut stacks = input.stacks.clone();
    for step in input.rearrangement_procedure.iter() {
        apply_rearrangement_step(&mut stacks, &step, |x| x);
    }

    let part_2 = stacks
        .into_iter()
        .filter_map(|stack| stack.into_iter().last())
        .map(|crate_| crate_.identifier)
        .collect::<String>();

    println!("Part 2: {}", part_2);
}
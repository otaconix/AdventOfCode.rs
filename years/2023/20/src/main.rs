use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fmt::{Debug, Display};
use std::io;
use std::ops::ControlFlow;
use std::rc::Rc;

use aoc_timing::trace::log_run;
use log::debug;

type Input = HashMap<String, (Module, Vec<String>)>;

struct DependencyTree<'a> {
    name: &'a str,
    root: &'a Module,
    dependencies: Vec<Rc<DependencyTree<'a>>>,
}

impl Display for DependencyTree<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.name)
            .field(self.root)
            .finish()?;

        Ok(())
    }
}

const RX_NAME: &str = "rx";

fn construct_dependency_tree(input: &Input) -> DependencyTree<'_> {
    fn inner<'a>(input: &'a Input, name: &'a str) -> DependencyTree<'a> {
        let (root, destinations) = &input[name];
        DependencyTree {
            name,
            root,
            dependencies: input
                .iter()
                .filter(|(_, (_, destinations))| destinations.contains(&name.to_string()))
                .map(|(name, _)| Rc::new(inner(input, name)))
                .collect(),
        }
    }

    DependencyTree {
        name: RX_NAME,
        root: &Module::Broadcast,
        dependencies: input
            .iter()
            .filter(|(name, (module, destinations))| destinations.contains(&RX_NAME.to_string()))
            .map(|(name, _)| Rc::new(inner(input, name)))
            .collect(),
    }
}

#[derive(Hash, Clone, Copy, Debug)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Hash, Debug)]
enum Module {
    Broadcast,
    FlipFlop {
        on: bool,
    },
    Conjunction {
        last_inputs: BTreeMap<String, Pulse>,
    },
}

impl Module {
    /// Process a pulse
    fn process_pulse(&mut self, source: &str, pulse: Pulse) -> Option<Pulse> {
        match self {
            Module::Broadcast => Some(pulse),
            Module::FlipFlop { ref mut on } => {
                if matches!(pulse, Pulse::Low) {
                    *on = !*on;
                    let output_pulse = if *on { Pulse::High } else { Pulse::Low };

                    Some(output_pulse)
                } else {
                    None
                }
            }
            Module::Conjunction {
                ref mut last_inputs,
            } => {
                last_inputs.insert(source.to_string(), pulse);

                let output_pulse = if last_inputs
                    .values()
                    .all(|remembered_pulse| matches!(remembered_pulse, Pulse::High))
                {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                Some(output_pulse)
            }
        }
    }

    /// Register a new source.
    fn register_source(&mut self, source: &str) {
        if let Module::Conjunction { last_inputs } = self {
            last_inputs.insert(source.to_string(), Pulse::Low);
        }
    }
}

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Input {
    let mut input: Input = input
        .map(|line| line.to_string())
        .map(|line| {
            let (name, destinations) = line.split_once(" -> ").unwrap();
            let destinations = destinations
                .split(", ")
                .map(|name| name.to_string())
                .collect::<Vec<_>>();

            let (name, module) = match name.chars().next() {
                Some('&') => (
                    name[1..].to_string(),
                    Module::Conjunction {
                        last_inputs: BTreeMap::new(),
                    },
                ),
                Some('%') => (name[1..].to_string(), Module::FlipFlop { on: false }),
                _ => (name.to_string(), Module::Broadcast),
            };

            (name, (module, destinations))
        })
        .collect();

    let source_to_destination = input
        .iter()
        .flat_map(|(name, (_, destinations))| {
            destinations
                .iter()
                .map(|destination| (name.clone(), destination.clone()))
        })
        .collect::<Vec<_>>();

    for (source, destination) in source_to_destination {
        if let Some(ref mut module) = input.get_mut(&destination) {
            module.0.register_source(&source);
        }
    }

    input
}

fn push_button<S, F: Fn(S, (Pulse, &Vec<String>)) -> (S, bool)>(
    input: &mut Input,
    state: S,
    update_state: F,
) -> S {
    let mut pulse_queue = VecDeque::new();
    pulse_queue.push_back((
        "button".to_string(),
        Pulse::Low,
        vec!["broadcaster".to_string()],
    ));

    let mut state = state;

    while let Some((source, pulse, destinations)) = pulse_queue.pop_front() {
        let (new_state, do_break) = update_state(state, (pulse, &destinations));
        state = new_state;
        if do_break {
            break;
        }

        pulse_queue.extend(destinations.into_iter().filter_map(|destination| {
            debug!("{source} -{pulse:?}-> {destination}");
            if let Some((ref mut module, destinations)) = input.get_mut(&destination) {
                module
                    .process_pulse(&source, pulse)
                    .map(|new_pulse| (destination.to_owned(), new_pulse, destinations.clone()))
            } else {
                None
            }
        }));
    }

    state
}

fn part_1(input: &Input) -> usize {
    let mut input: Input = input.clone();

    let (sent_low, sent_high) = (0..1000).fold((0, 0), |state, _| {
        push_button(
            &mut input,
            state,
            |(sent_low, sent_high), (pulse, destinations)| {
                (
                    match pulse {
                        Pulse::Low => (sent_low + destinations.len(), sent_high),
                        Pulse::High => (sent_low, sent_high + destinations.len()),
                    },
                    false,
                )
            },
        )
    });

    sent_low * sent_high
}

fn part_2(input: &Input) -> usize {
    let dependency_tree = construct_dependency_tree(input);

    println!("Dependency tree:");
    println!("{dependency_tree}");

    0
    //
    // let mut input: Input = input.clone();
    //
    // let rx: String = String::from("rx");
    //
    // use ControlFlow::*;
    // let button_presses = (1..).try_fold(false, |do_break, button_presses| {
    //     let do_break = push_button(&mut input, do_break, |_, (pulse, destinations)| {
    //         let result = matches!(pulse, Pulse::Low) && destinations.contains(&rx);
    //         (result, result)
    //     });
    //
    //     if do_break {
    //         Break(button_presses)
    //     } else {
    //         Continue(do_break)
    //     }
    // });
    //
    // match button_presses {
    //     Break(button_presses) => button_presses,
    //     _ => panic!(),
    // }
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

        assert_eq!(result, 32000000);
    }

    // #[test]
    // fn test_part_2() {
    //     let input = parse(INPUT.lines());
    //     let result = part_2(&input);
    //
    //     assert_eq!(result, 0);
    // }
}

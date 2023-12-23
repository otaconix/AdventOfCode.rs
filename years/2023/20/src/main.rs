use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter, Write};
use std::io;
use std::rc::Rc;

use aoc_timing::trace::log_run;
use log::debug;

type Input = HashMap<String, (Module, Vec<String>)>;

struct DependencyTree<'a> {
    name: &'a str,
    root: &'a Module,
    dependencies: Vec<Rc<RefCell<DependencyTree<'a>>>>,
}

impl Display for DependencyTree<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn inner<'a>(
            tree: &DependencyTree<'a>,
            depth: usize,
            already_seen: &mut HashSet<&'a str>,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            let width = depth * 2;
            let prefix = "";

            f.write_fmt(format_args!("{:>width$}Name: {}\n", prefix, tree.name,))?;
            f.write_fmt(format_args!("{:>width$}Module: {}\n", "", tree.root))?;

            if already_seen.contains(tree.name) {
                f.write_fmt(format_args!("{:>width$}Dependencies already seen", ""))?;
            } else if tree.dependencies.is_empty() {
                f.write_fmt(format_args!("{:>width$}No dependencies", ""))?;
            } else {
                already_seen.insert(tree.name);
                f.write_fmt(format_args!("{:>width$}Dependencies:", ""))?;
                for dependency in &tree.dependencies {
                    f.write_char('\n')?;
                    inner(&dependency.borrow(), depth + 1, already_seen, f)?;
                }
            }

            Ok(())
        }

        let mut already_seen = HashSet::new();
        inner(self, 0, &mut already_seen, f)?;

        Ok(())
    }
}

const RX_NAME: &str = "rx";

fn construct_dependency_tree(input: &Input) -> DependencyTree<'_> {
    fn inner<'a>(
        input: &'a Input,
        name: &'a str,
        already_seen: &mut HashMap<String, Rc<RefCell<DependencyTree<'a>>>>,
    ) -> Rc<RefCell<DependencyTree<'a>>> {
        if let Some(already_seen) = already_seen.get(name) {
            already_seen.clone()
        } else {
            let (root, _) = &input[name];
            let result = Rc::new(RefCell::new(DependencyTree {
                name,
                root,
                dependencies: vec![],
            }));

            already_seen.insert(name.to_string(), result.clone());
            result.borrow_mut().dependencies.extend(
                input
                    .iter()
                    .filter(|(_, (_, destinations))| destinations.contains(&name.to_string()))
                    .map(|(name, _)| inner(input, name, already_seen)),
            );

            if matches!(root, Module::Broadcast) {
                result
                    .borrow_mut()
                    .dependencies
                    .push(Rc::new(RefCell::new(DependencyTree {
                        name: "button",
                        root: &Module::Button,
                        dependencies: vec![],
                    })));
            }

            result
        }
    }

    let mut already_seen = HashMap::new();

    DependencyTree {
        name: RX_NAME,
        root: &Module::FinalDestination,
        dependencies: input
            .iter()
            .filter(|(_, (_, destinations))| destinations.contains(&RX_NAME.to_string()))
            .map(|(name, _)| inner(input, name, &mut already_seen))
            .collect(),
    }
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
enum Module {
    Button,
    FinalDestination,
    Broadcast,
    FlipFlop {
        on: bool,
    },
    Conjunction {
        last_inputs: BTreeMap<String, Pulse>,
    },
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Module::Button => "button",
            Module::FinalDestination => "final destination",
            Module::Broadcast => "broadcast",
            Module::FlipFlop { on: _ } => "flip-flop",
            Module::Conjunction { last_inputs: _ } => "conjunction",
        })
    }
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
            _ => None,
        }
    }

    /// Register a new source.
    fn register_source(&mut self, source: &str) {
        if let Module::Conjunction { last_inputs } = self {
            last_inputs.insert(source.to_string(), Pulse::Low);
        }
    }
}

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let mut input: Input = input
        .map(|line| {
            let (name, destinations) = line.as_ref().split_once(" -> ").unwrap();
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

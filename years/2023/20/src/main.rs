use std::cell::OnceCell;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::io;

use aoc_timing::trace::log_run;
use fxhash::FxHashMap;
use log::debug;

#[derive(Debug, Clone)]
struct Input {
    links: HashMap<String, Vec<String>>,
    modules: HashMap<String, Module>,
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Module {
    Broadcast,
    FlipFlop {
        on: bool,
    },
    Conjunction {
        last_inputs: FxHashMap<String, Pulse>,
    },
}

impl Module {
    #[allow(dead_code)]
    fn symbol(&self) -> char {
        match self {
            Module::Broadcast => '!',
            Module::FlipFlop { .. } => '%',
            Module::Conjunction { .. } => '&',
        }
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
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
    let (mut modules, links) = input.fold(
        (
            HashMap::<String, Module>::default(),
            HashMap::<String, Vec<String>>::default(),
        ),
        |(mut modules, mut links), line| {
            let (name, destinations) = line.as_ref().split_once(" -> ").unwrap();
            let destinations = destinations
                .split(", ")
                .map(|name| name.to_string())
                .collect::<Vec<_>>();

            let (name, module) = match name.chars().next() {
                Some('&') => (
                    name[1..].to_string(),
                    Module::Conjunction {
                        last_inputs: FxHashMap::default(),
                    },
                ),
                Some('%') => (name[1..].to_string(), Module::FlipFlop { on: false }),
                _ => (name.to_string(), Module::Broadcast),
            };

            modules.entry(name.clone()).or_insert(module);
            links.entry(name).or_default().extend(destinations);

            (modules, links)
        },
    );

    for (source, destinations) in &links {
        for destination in destinations {
            if let Some(module) = modules.get_mut(destination) {
                module.register_source(source);
            }
        }
    }

    Input { modules, links }
}

const BUTTON_DESTINATIONS: OnceCell<Vec<String>> = OnceCell::new();

fn push_button<S, F: Fn(S, (Pulse, &[String])) -> S>(
    input: &mut Input,
    state: S,
    update_state: F,
) -> S {
    debug!("== Pressing button ===");
    let mut pulse_queue = VecDeque::new();
    let binding = BUTTON_DESTINATIONS;
    pulse_queue.push_back((
        "button".to_string(),
        Pulse::Low,
        binding.get_or_init(|| vec!["broadcaster".to_string()]),
    ));

    let mut state = state;

    while let Some((source, pulse, destinations)) = pulse_queue.pop_front() {
        state = update_state(state, (pulse, destinations));

        pulse_queue.extend(destinations.iter().filter_map(|destination| {
            debug!("{source} -{pulse:?}-> {destination}");
            if let Some(module) = input.modules.get_mut(destination) {
                module.process_pulse(&source, pulse).and_then(|new_pulse| {
                    input
                        .links
                        .get(destination)
                        .map(|destinations| (destination.to_string(), new_pulse, destinations))
                })
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
            |(sent_low, sent_high), (pulse, destinations)| match pulse {
                Pulse::Low => (sent_low + destinations.len(), sent_high),
                Pulse::High => (sent_low, sent_high + destinations.len()),
            },
        )
    });

    debug!("Sent: low={sent_low}; high={sent_high}");
    sent_low * sent_high
}

/// Ugly solution. Basically, I looked at the input, and concluded that:
/// 1. `rx` has only a single input: a conjunction module
/// 2. That conjunction module has a bunch of inputs, all of which are also conjunctions
///
/// So, let's figure out how many button presses are needed to get a _low_ pulse out of the
/// conjunctions twice removed from `rx`, and multiply them together, to find the lowest number at
/// which all of them output a _low_ pulse.
fn part_2(input: &Input) -> usize {
    let mut input: Input = input.clone();
    let rx_input = input
        .links
        .iter()
        .find_map(|(module, destinations)| {
            if destinations.contains(&"rx".to_string()) {
                Some(module)
            } else {
                None
            }
        })
        .expect("No 'rx' module found");
    let rx_input_inputs = input
        .links
        .iter()
        .filter_map(|(module, destinations)| {
            if destinations.contains(rx_input) {
                Some(module)
            } else {
                None
            }
        })
        .cloned()
        .collect::<Vec<_>>();

    use std::ops::ControlFlow::*;

    let input_inputs_lows = (1..).try_fold(FxHashMap::default(), |state, button_presses| {
        let state = push_button(&mut input, state, |mut state, (pulse, destinations)| {
            if pulse == Pulse::Low {
                rx_input_inputs
                    .iter()
                    .filter(|input_inputs| destinations.contains(*input_inputs))
                    .for_each(|input_input| {
                        state.insert(input_input, button_presses);
                    });
            }

            state
        });

        if state.len() == rx_input_inputs.len() {
            Break(state)
        } else {
            Continue(state)
        }
    });

    match input_inputs_lows {
        Break(map) => map.into_values().product(),
        _ => panic!("Broke out of loop?!"),
    }
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        // log_run("Writing dot for input", || {
        //     println!("digraph {{");
        //     for (name, module) in &input.modules {
        //         println!("  {name} [label=\"{}{name}\"]", module.symbol());
        //
        //         for destination in input.links.get(name).unwrap_or(&vec![]) {
        //             println!("  {name} -> {destination}")
        //         }
        //     }
        //     println!("}}");
        // });

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

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_part_1() {
        init();
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 32000000);
    }
}

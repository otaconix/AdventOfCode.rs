use std::collections::{HashMap, HashSet};
use std::io;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct DestinationValve {
    steps: u32,
    name: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Valve {
    name: String,
    flow_rate: i32,
    pipe_destinations: Vec<DestinationValve>,
}

impl FromStr for Valve {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use pom::char_class::*;
        use pom::parser::*;

        let number = is_a(digit).repeat(1..).map(|digits| {
            digits
                .iter()
                .fold(0i32, |result, digit| result * 10 + (digit - b'0') as i32)
        });
        let valve_name = || is_a(alpha).repeat(1..).convert(String::from_utf8);
        let parser = ((seq(b"Valve ") * valve_name())
            + (seq(b" has flow rate=") * number)
            + (seq(b"; tunnel")
                * sym(b's').opt()
                * seq(b" lead")
                * sym(b's').opt()
                * seq(b" to valve")
                * sym(b's').opt()
                * sym(b' ')
                * list(valve_name(), seq(b", "))))
        .map(|((name, flow_rate), pipe_destinations)| Valve {
            name,
            flow_rate,
            pipe_destinations: pipe_destinations
                .iter()
                .map(|name| DestinationValve {
                    steps: 1,
                    name: name.clone(),
                })
                .collect(),
        });

        parser.parse(s.as_bytes()).map_err(|e| e.to_string())
    }
}

fn maximum_possible_pressure_release<'a>(
    start_valve: &'a str,
    valves: &'a HashMap<String, Valve>,
    time_left: u32,
) -> i32 {
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Path<'a> {
        current: &'a Valve,
        opened: Vec<&'a String>,
        released: i32,
        flow_rate: i32,
    }

    let mut paths = vec![Path {
        current: &valves[start_valve],
        opened: vec![],
        released: 0,
        flow_rate: 0,
    }];
    let max_flow_rate = valves.values().map(|valve| valve.flow_rate).max().unwrap();

    for _ in 1..=time_left {
        paths = paths
            .into_iter()
            .flat_map(|path| {
                let mut new_paths = path
                    .current
                    .pipe_destinations
                    .iter()
                    .map(|destination| Path {
                        current: &valves[&destination.name],
                        opened: path.opened.clone(),
                        released: path.released + path.flow_rate,
                        flow_rate: path.flow_rate,
                    })
                    .collect::<Vec<_>>();

                if !path.opened.contains(&&path.current.name) && path.current.flow_rate > 0 {
                    let mut opened = path.opened.clone();
                    opened.push(&path.current.name);
                    new_paths.push(Path {
                        current: path.current,
                        released: path.released + path.flow_rate,
                        flow_rate: path.flow_rate + path.current.flow_rate,
                        opened,
                    });
                }

                new_paths
            })
            .collect::<Vec<_>>();

        let max_released = paths.iter().map(|path| path.released).max().unwrap();

        paths = paths
            .into_iter()
            .filter(|path| path.released > max_released - max_flow_rate)
            .collect::<Vec<_>>();
    }

    println!("Possible paths count: {}", paths.len());
    paths.into_iter().map(|path| path.released).max().unwrap()
}

fn simplify_graph(
    start_valve_name: &str,
    mut valves: HashMap<String, Valve>,
) -> HashMap<String, Valve> {
    fn remove_valve(valve_to_remove_name: &str, valves: &mut HashMap<String, Valve>) -> Valve {
        let removed_valve = valves.remove(valve_to_remove_name).unwrap();
        let new_destinations = removed_valve.pipe_destinations.clone();
        valves.values_mut().for_each(|to_update| {
            to_update.pipe_destinations = to_update
                .pipe_destinations
                .iter()
                .flat_map(|dest| {
                    if dest.name == valve_to_remove_name {
                        new_destinations
                            .iter()
                            .filter(|new_destination| new_destination.name != to_update.name)
                            .cloned()
                            .map(|new_destination| DestinationValve {
                                steps: new_destination.steps + dest.steps,
                                ..new_destination
                            })
                            .collect()
                    } else {
                        vec![dest.clone()]
                    }
                })
                .collect();
        });

        removed_valve
    }

    if let Some(zero_flow_rate) = valves
        .values()
        .find(|valve| valve.name != start_valve_name && valve.flow_rate == 0)
        .map(|valve| valve.name.clone())
    {
        remove_valve(&zero_flow_rate, &mut valves);
        simplify_graph(start_valve_name, valves)
    } else {
        valves
    }
}

fn main() {
    // let valves = simplify_graph(
    // "AA",
    let valves = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| {
            line.parse::<Valve>()
                .map_err(|e| format!("{}: {}", line, e))
                .unwrap()
        })
        .map(|valve| (valve.name.clone(), valve))
        .collect::<HashMap<_, _>>();
    // .collect(),
    // );

    let part_1 = maximum_possible_pressure_release("AA", &valves, 30);
    println!("Part 1: {part_1}");
}

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
    flow_rate: u32,
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
                .fold(0u32, |result, digit| result * 10 + (digit - b'0') as u32)
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
    start_valve: &'a Valve,
    valves: &'a HashMap<String, Valve>,
    time_left: u32,
) -> u32 {
    fn inner<'a>(
        valves: &'a HashMap<String, Valve>,
        released_pressure: u32,
        mut maximum: u32,
        opened_valves: &mut HashSet<&'a Valve>,
        current_valve: &'a Valve,
        time_left: u32,
    ) -> u32 {
        // println!(
        //     "Current valve: {}, maximum: {maximum}, released_pressure: {released_pressure}, time_left: {time_left}",
        //     current_valve.name
        // );
        if time_left == 0 || opened_valves.len() == valves.len() {
            return released_pressure;
        }

        for next_destination in current_valve.pipe_destinations.iter() {
            let next_valve = &valves[&next_destination.name];
            if !opened_valves.contains(current_valve) && time_left > next_destination.steps {
                opened_valves.insert(current_valve);
                maximum = inner(
                    valves,
                    released_pressure + (current_valve.flow_rate * (time_left - 1)),
                    maximum,
                    opened_valves,
                    next_valve,
                    time_left - next_destination.steps - 1,
                )
                .max(maximum);
                opened_valves.remove(current_valve);
            } else if time_left > next_destination.steps {
                maximum = inner(
                    valves,
                    released_pressure,
                    maximum,
                    opened_valves,
                    next_valve,
                    time_left - next_destination.steps,
                )
                .max(maximum);
            }
        }

        if time_left > 1 && !opened_valves.contains(current_valve) {
            maximum = maximum.max(released_pressure + current_valve.flow_rate * (time_left - 1))
        }

        maximum.max(released_pressure)
    }

    let mut opened_valves = HashSet::new();
    opened_valves.insert(start_valve);

    inner(valves, 0, 0, &mut opened_valves, start_valve, time_left)
}

fn simplify_graph(
    start_valve_name: &str,
    mut valves: HashMap<String, Valve>,
    // ) -> (Vec<DestinationValve>, HashMap<String, Valve>) {
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
        // let start_valve_destinations =
        // remove_valve(start_valve_name, &mut valves).pipe_destinations;
        // (start_valve_destinations, valves)
        valves
    }
}

fn main() {
    // let (start_destinations, valves) = simplify_graph(
    let valves = simplify_graph(
        "AA",
        // let valves: HashMap<String, Valve> = io::stdin()
        io::stdin()
            .lines()
            .map(|result| result.expect("I/O error"))
            .map(|line| {
                line.parse::<Valve>()
                    .map_err(|e| format!("{}: {}", line, e))
                    .unwrap()
            })
            .map(|valve| (valve.name.clone(), valve))
            // .collect();
            .collect(),
    );

    // let part_1 = start_destinations
    //     .iter()
    //     .map(|dest| {
    //         maximum_possible_pressure_release(&valves[&dest.name], &valves, 30 - dest.steps)
    //     })
    //     .max()
    //     .unwrap();
    let part_1 = maximum_possible_pressure_release(&valves["AA"], &valves, 30);
    println!("Part 1: {part_1}");
}

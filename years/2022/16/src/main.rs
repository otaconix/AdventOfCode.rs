use std::collections::HashMap;
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
            .collect();

        let max_released = paths.iter().map(|path| path.released).max().unwrap();

        paths.retain(|path| path.released > max_released - max_flow_rate);
    }

    paths.into_iter().map(|path| path.released).max().unwrap()
}

fn maximum_possible_pressure_release_with_elephant<'a>(
    start_valve: &'a str,
    valves: &'a HashMap<String, Valve>,
    time_left: u32,
) -> i32 {
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Path<'a> {
        me: &'a Valve,
        elephant: &'a Valve,
        opened: Vec<&'a String>,
        released: i32,
        flow_rate: i32,
    }

    let mut paths = vec![Path {
        me: &valves[start_valve],
        elephant: &valves[start_valve],
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
                    .me
                    .pipe_destinations
                    .iter()
                    .map(|destination| Path {
                        me: &valves[&destination.name],
                        elephant: path.elephant,
                        opened: path.opened.clone(),
                        released: path.released + path.flow_rate,
                        flow_rate: path.flow_rate,
                    })
                    .collect::<Vec<_>>();

                if !path.opened.contains(&&path.me.name) && path.me.flow_rate > 0 {
                    let mut opened = path.opened.clone();
                    opened.push(&path.me.name);
                    new_paths.push(Path {
                        me: path.me,
                        elephant: path.elephant,
                        released: path.released + path.flow_rate,
                        flow_rate: path.flow_rate + path.me.flow_rate,
                        opened,
                    });
                }

                let max_released = new_paths.iter().map(|path| path.released).max().unwrap();

                new_paths.retain(|path| path.released > max_released - max_flow_rate);

                let new_paths = new_paths
                    .iter()
                    .flat_map(|new_path| {
                        let mut new_elephant_paths = new_path
                            .elephant
                            .pipe_destinations
                            .iter()
                            .map(|destination| Path {
                                me: new_path.me,
                                elephant: &valves[&destination.name],
                                opened: new_path.opened.clone(),
                                released: new_path.released,
                                flow_rate: new_path.flow_rate,
                            })
                            .collect::<Vec<_>>();

                        if !new_path.opened.contains(&&new_path.elephant.name)
                            && new_path.elephant.flow_rate > 0
                        {
                            let mut opened = new_path.opened.clone();
                            opened.push(&new_path.elephant.name);
                            new_elephant_paths.push(Path {
                                opened,
                                flow_rate: new_path.flow_rate + new_path.elephant.flow_rate,
                                ..*new_path
                            })
                        }

                        new_elephant_paths
                    })
                    .collect::<Vec<_>>();

                new_paths
            })
            .collect::<Vec<_>>();

        let max_released = paths.iter().map(|path| path.released).max().unwrap();

        paths = paths
            .into_iter()
            .filter(|path| path.released > max_released - max_flow_rate)
            .collect::<Vec<_>>();
    }

    paths.into_iter().map(|path| path.released).max().unwrap()
}

fn main() {
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

    let part_1 = maximum_possible_pressure_release("AA", &valves, 30);
    println!("Part 1: {part_1}");

    let part_2 = maximum_possible_pressure_release_with_elephant("AA", &valves, 26);
    println!("Part 2: {part_2}");
}

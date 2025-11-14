use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
};

use aoc_timing::trace::log_run;
use dijkstra::dijkstra;
use intcode::{Computer, OpCode, SplitIO};

type Input = Computer;
type Output1 = (usize, (i64, i64), HashSet<(i64, i64)>);
type Output2 = usize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(mut input: I) -> Input {
    let line = input.next().expect("No input line!");

    Computer::parse(line.as_ref())
}

/// Finds the oxygen system, then the shortest path there.
///
/// ## Finding the oxygen system
///
/// To do this, we maintain a list of known walls, and what neighbors of each coordinate we've
/// already visited. Whenever we're at a particular coordinate, do the following:
/// 1. Make a set of potential coordinates (north, south, east, west) to move to
/// 2. Filter known walls from the set of potential next coordinates
/// 3. Remove the previous coordinate the robot was at from the set of potential coordinates
/// 4. Of the remaining coordinates, pick the one with the fewest visited neighbors, and go there.
/// 5. If no remaining coordinates were found, go back.
///
/// The issue then is that we can only update our position after the robot gives us a status code.
/// So we need to keep track of where we are, and where we told the robot to move to. If:
/// 1. it tells us it hit a wall, add the position it tried to go to to the set of known walls
/// 2. it tells us it moved, update our current coordinate, and add it to the set of coordinates
///    that are known _not_ to be walls.
/// 3. If we've found the oxygen system, stop!
///
/// Now we can find the shortest path there.
///
/// ## Finding the shortest path
///
/// This is a little tricky, and the way it's implemented here probably doesn't work in the general
/// case, but instead of using our set of known walls, we use our set of coordinates that are _not_
/// walls. Starting from our origin, take every neighbor from that set, and find the shortest path
/// that way (in our case: using Dijkstra's algorithm).
///
/// The reason we don't use the set of known walls: we probably haven't visited every wall, so our
/// shortest path algorithm may find a path that goes through walls we haven't found yet.
fn part_1(input: &Input) -> Output1 {
    let mut computer = input.clone();
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();
    let mut current_x = 0i64;
    let mut current_y = 0i64;
    let mut walls = HashSet::new();
    let mut not_walls = HashSet::from([(0, 0)]);
    let mut visited_neighbors: HashMap<_, HashSet<i64>> = HashMap::new();
    let mut last_position = (0, 0);
    let mut next_movement = (0, (0, 0));
    let mut oxygen_system = (0, 0);

    loop {
        match computer.step(&mut SplitIO::new(&mut input, &mut output)) {
            Some(OpCode::Input) => {
                let potential_movements = [
                    (1, (current_x, current_y - 1)), // North
                    (2, (current_x, current_y + 1)), // South
                    (3, (current_x - 1, current_y)), // West
                    (4, (current_x + 1, current_y)), // East
                ];
                let opposite_of_last_movement = potential_movements
                    .iter()
                    .find(|(_, position)| &last_position == position)
                    .copied();

                let (direction, movement) = potential_movements
                    .into_iter()
                    // Filter out the opposite of our last movement
                    .filter(|movement| {
                        if let Some(opposite) = opposite_of_last_movement {
                            &opposite != movement
                        } else {
                            true
                        }
                    })
                    // Filter out positions we know are walls
                    .filter(|(_, coord)| !walls.contains(coord))
                    .min_by_key(|(_, coord)| {
                        visited_neighbors
                            .get(coord)
                            .map(|neighbors| neighbors.len())
                            .unwrap_or(0)
                    })
                    .or(opposite_of_last_movement)
                    .unwrap();

                next_movement = (direction, movement);

                println!("Trying to move to {movement:?}");

                input.push_back(direction);

                continue;
            }
            Some(OpCode::Terminate) => break,
            _ => {}
        }

        if let Some(status) = output.pop_front() {
            match status {
                0 => {
                    println!("Hit wall at {next_movement:?}, staying at current position");
                    walls.insert(next_movement.1);
                }
                1 => {
                    visited_neighbors
                        .entry((current_x, current_y))
                        .or_default()
                        .insert(next_movement.0);
                    last_position = (current_x, current_y);
                    (current_x, current_y) = next_movement.1;

                    not_walls.insert(next_movement.1);

                    println!("Current position: {current_x},{current_y}");
                }
                2 => {
                    oxygen_system = next_movement.1;
                    not_walls.insert(oxygen_system);
                    break;
                }
                _ => panic!("Unknown status received from robot: {status}"),
            }
        }
    }

    println!("Computer exited. Oxygen system is at {oxygen_system:?}.");
    println!("Found {} walls", walls.len());

    (
        dijkstra(
            (0, 0),
            |coord| coord == &oxygen_system,
            |(x, y)| {
                [
                    (*x, y - 1), // North
                    (*x, y + 1), // South
                    (x - 1, *y), // West
                    (x + 1, *y), // East
                ]
                .into_iter()
                .filter(|coord| not_walls.contains(coord))
                .map(|coord| (coord, 1))
            },
        )
        .unwrap()
        .into_iter()
        .find(|(node, _)| node == &oxygen_system)
        .unwrap()
        .1,
        oxygen_system,
        not_walls,
    )
}

/// Flood fill, where we keep track of how many steps we've taken.
/// Inputs:
/// - `oxygen_system`: the position of the oxygen system, as found in part 1
/// - `not_walls`: all coordinates that are known not to be walls, as found in part 1
fn part_2(oxygen_system: &(i64, i64), not_walls: &HashSet<(i64, i64)>) -> Output2 {
    let mut queue = vec![(*oxygen_system, 0)];
    let mut max_steps = 0;
    let mut unvisited = not_walls.clone();

    while let Some((next, steps)) = queue.pop() {
        if unvisited.take(&next).is_some() {
            max_steps = max_steps.max(steps);
            let next_steps = steps + 1;
            queue.push(((next.0, next.1 - 1), next_steps));
            queue.push(((next.0, next.1 + 1), next_steps));
            queue.push(((next.0 - 1, next.1), next_steps));
            queue.push(((next.0 + 1, next.1), next_steps));
        }
    }

    max_steps
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        let part_1 = log_run("Part 1", || part_1(&input));
        println!("Part 1: {}", part_1.0);

        let part_2 = log_run("Part 2", || part_2(&part_1.1, &part_1.2));
        println!("Part 2: {part_2}");
    });
}

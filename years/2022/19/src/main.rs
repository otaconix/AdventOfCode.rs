mod solution;

use solution::Blueprint;
use std::io;

use crate::solution::Factory;

fn main() {
    env_logger::init();

    let blueprints: Vec<Blueprint> = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| line.parse().expect("Failed to parse blueprint"))
        .collect();

    let part_1: u32 = blueprints
        .iter()
        .map(|blueprint| blueprint.run_simulation(Factory::initial(24)))
        .zip(1..)
        .map(|(i, max_geodes)| i * max_geodes)
        .sum();
    println!("Part 1: {part_1}");

    let part_2: u32 = blueprints
        .iter()
        .take(3)
        .map(|blueprint| blueprint.run_simulation(Factory::initial(32)))
        .product();
    println!("Part 2: {part_2}");
}

use std::io;

use aoc_timing::trace::log_run;
use itertools::Itertools;
use rapidhash::RapidHashMap;
use rapidhash::RapidHashSet;

struct Input {
    computers: RapidHashSet<String>,
    original_map: RapidHashMap<String, RapidHashSet<String>>,
    adjacency_map: RapidHashMap<String, RapidHashSet<String>>,
}
type Output1 = usize;
type Output2 = String;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let (computers, adjacency_map, original_map) = input.fold(
        (
            RapidHashSet::default(),
            RapidHashMap::<String, RapidHashSet<String>>::default(),
            RapidHashMap::<String, RapidHashSet<String>>::default(),
        ),
        |(mut computers, mut adjacency_map, mut original_map), line| {
            let (left, right) = line.as_ref().split_once('-').unwrap();

            adjacency_map
                .entry(left.to_string())
                .or_default()
                .insert(right.to_string());
            adjacency_map
                .entry(right.to_string())
                .or_default()
                .insert(left.to_string());
            original_map
                .entry(left.to_string())
                .or_default()
                .insert(right.to_lowercase());
            computers.insert(left.to_string());
            computers.insert(right.to_string());

            (computers, adjacency_map, original_map)
        },
    );

    Input {
        computers,
        adjacency_map,
        original_map,
    }
}

fn clusters_of_three<'a>(
    computer: &'a str,
    input: &'a Input,
) -> impl Iterator<Item = Vec<String>> + use<'a> {
    input
        .adjacency_map
        .get(computer)
        .unwrap()
        .iter()
        .combinations(2)
        .filter(|neighbors| {
            input
                .adjacency_map
                .get(neighbors[0])
                .unwrap()
                .contains(neighbors[1])
        })
        .map(|neighbors| {
            let mut cluster = vec![
                computer.to_string(),
                neighbors[0].clone(),
                neighbors[1].clone(),
            ];
            cluster.sort();

            cluster
        })
        .unique()
}

fn part_1(input: &Input) -> Output1 {
    input
        .computers
        .iter()
        .filter(|computer| computer.starts_with('t'))
        .flat_map(|computer| clusters_of_three(computer, input))
        .unique()
        .count()
}

fn find_largest_mesh(
    computer: &String,
    seen_computers: &mut RapidHashSet<String>,
    input: &Input,
) -> Vec<String> {
    // println!("Seen computers: {}", seen_computers.len());
    seen_computers.insert(computer.to_string());

    let next_in_mesh = input
        .original_map
        .get(computer)
        .unwrap()
        .iter()
        .filter(|neighbor| {
            !seen_computers.contains(*neighbor)
                && input
                    .adjacency_map
                    .get(*neighbor)
                    .unwrap()
                    .is_superset(seen_computers)
        })
        .collect_vec();

    let result = if next_in_mesh.is_empty() {
        seen_computers.iter().cloned().collect_vec()
    } else {
        next_in_mesh
            .iter()
            .map(|neighbor| find_largest_mesh(neighbor, seen_computers, input))
            .max_by_key(|cluster| cluster.len())
            .unwrap_or(seen_computers.iter().cloned().collect_vec())
    };

    seen_computers.remove(computer);

    result
}

fn part_2(input: &Input) -> Output2 {
    let mut largest_cluster = input
        .computers
        .iter()
        .inspect(|computer| println!("=== Starting {computer} ==="))
        .map(|computer| find_largest_mesh(computer, &mut RapidHashSet::default(), input))
        .max_by_key(|cluster| cluster.len())
        .unwrap();

    println!("Largest cluster: {largest_cluster:#?}");

    largest_cluster.sort();

    largest_cluster.join(",")
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

        assert_eq!(result, 7);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, "co,de,ka,ta");
    }
}

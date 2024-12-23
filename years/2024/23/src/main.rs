use std::io;

use aoc_timing::trace::log_run;
use itertools::Itertools;
use rapidhash::RapidHashMap;
use rapidhash::RapidHashSet;

struct Input {
    computers: RapidHashSet<String>,
    adjacency_map: RapidHashMap<String, RapidHashSet<String>>,
}
type Output1 = usize;
type Output2 = String;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let (computers, adjacency_map) = input.fold(
        (
            RapidHashSet::default(),
            RapidHashMap::<String, RapidHashSet<String>>::default(),
        ),
        |(mut computers, mut adjacency_map), line| {
            let (left, right) = line.as_ref().split_once('-').unwrap();

            adjacency_map
                .entry(left.to_string())
                .or_default()
                .insert(right.to_string());
            adjacency_map
                .entry(right.to_string())
                .or_default()
                .insert(left.to_string());
            computers.insert(left.to_string());
            computers.insert(right.to_string());

            (computers, adjacency_map)
        },
    );

    Input {
        computers,
        adjacency_map,
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

/// Bron-Kerbosch algorithm to find maximal cliques
fn maximal_cliques(
    r: RapidHashSet<String>,
    mut p: RapidHashSet<String>,
    mut x: RapidHashSet<String>,
    input: &Input,
    result: &mut Vec<RapidHashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        result.push(r.clone());
    } else {
        while let Some(computer) = p.iter().next().cloned() {
            let mut new_r = r.clone();
            new_r.insert(computer.clone());
            maximal_cliques(
                new_r,
                p.intersection(input.adjacency_map.get(&computer).unwrap())
                    .cloned()
                    .collect(),
                x.intersection(input.adjacency_map.get(&computer).unwrap())
                    .cloned()
                    .collect(),
                input,
                result,
            );

            p.remove(&computer);
            x.insert(computer.to_string());
        }
    }
}

fn part_2(input: &Input) -> Output2 {
    let mut result = vec![];
    maximal_cliques(
        RapidHashSet::default(),
        input.computers.clone(),
        RapidHashSet::default(),
        input,
        &mut result,
    );

    let mut maximum_clique = result
        .iter()
        .max_by_key(|clique| clique.len())
        .unwrap()
        .iter()
        .collect_vec();
    maximum_clique.sort();

    maximum_clique.iter().join(",")
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

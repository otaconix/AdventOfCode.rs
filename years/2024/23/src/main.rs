use std::io;

use aoc_timing::trace::log_run;
use itertools::Itertools;
use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;
use rapidhash::RapidHashMap;

struct Input {
    computer_indices: RapidHashMap<String, NodeIndex>,
    graph: UnGraph<String, ()>,
}
type Output1 = usize;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let (computer_indices, graph) = input.fold(
        (RapidHashMap::default(), UnGraph::new_undirected()),
        |(mut computer_indices, mut graph), line| {
            let line = line.as_ref();

            let (left, right) = line.split_once('-').unwrap();
            let left = left.to_string();
            let left_index = *computer_indices
                .entry(left.clone())
                .or_insert_with(|| graph.add_node(left));
            let right = right.to_string();
            let right_index = *computer_indices
                .entry(right.clone())
                .or_insert_with(|| graph.add_node(right));

            graph.add_edge(left_index, right_index, ());

            (computer_indices, graph)
        },
    );

    Input {
        computer_indices,
        graph,
    }
}

fn part_1(input: &Input) -> Output1 {
    input
        .computer_indices
        .iter()
        .filter(|(computer, _)| computer.starts_with("t"))
        .flat_map(|(historian_computer, historian_index)| {
            input
                .graph
                .neighbors(*historian_index)
                .combinations(2)
                .filter(|pair_of_neighbors| {
                    input
                        .graph
                        .neighbors(pair_of_neighbors[0])
                        .contains(&pair_of_neighbors[1])
                })
                .map(|pair_of_neighbors| {
                    let mut cluster = vec![
                        historian_computer.clone(),
                        input.graph[pair_of_neighbors[0]].clone(),
                        input.graph[pair_of_neighbors[1]].clone(),
                    ];
                    cluster.sort();

                    cluster
                })
        })
        .unique()
        .count()
}

fn part_2(input: &Input) -> Output2 {
    todo!()
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

        assert_eq!(result, 0);
    }
}

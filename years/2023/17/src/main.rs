use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    io,
    iter::successors,
};

use aoc_timing::trace::log_run;
use grid::Grid;

type Input = Grid<u8>;
type Coord = (usize, usize);

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            line.as_ref()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect()
}

#[derive(PartialEq, Eq)]
struct DijkstraVertex<T, P: Ord> {
    priority: P,
    value: T,
}

impl<T: Eq, P: Ord> PartialOrd for DijkstraVertex<T, P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq, P: Ord> Ord for DijkstraVertex<T, P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

fn shortest_path(grid: &Input, start: Coord, end: Coord) -> Option<Vec<Coord>> {
    type Current = (Coord, Vec<Coord>);
    let mut queue: BinaryHeap<DijkstraVertex<Current, usize>> = BinaryHeap::new();
    queue.push(DijkstraVertex {
        priority: 0,
        value: (start, vec![start]), // Let's start at the end so we don't have to reverse the path at the end
    });

    let mut prevs: HashMap<Coord, Coord> = HashMap::new();
    let mut heat_losses: HashMap<Coord, usize> = HashMap::new();
    heat_losses.insert(start, 0);

    while let Some(DijkstraVertex {
        value: (current @ (column, row), predecessors),
        priority: current_heat_loss,
    }) = queue.pop()
    {
        if current == end {
            // We're done.
            break;
        }

        for neighbor @ (ncolumn, nrow) in grid.get_neighbors(column, row) {
            if predecessors.len() < 4
                || !(predecessors.iter().all(|(pcolumn, _)| pcolumn == &ncolumn)
                    || predecessors.iter().all(|(_, prow)| prow == &nrow))
            {
                let new_heat_loss = current_heat_loss + *grid.get(ncolumn, nrow).unwrap() as usize;
                let existing_heat_loss = *heat_losses.get(&neighbor).unwrap_or(&usize::MAX);

                if new_heat_loss < existing_heat_loss {
                    heat_losses.insert(neighbor, new_heat_loss);
                    prevs.insert(neighbor, current);
                    queue.push(DijkstraVertex {
                        value: (
                            neighbor,
                            predecessors
                                .iter()
                                .skip(predecessors.len().saturating_sub(3))
                                .copied()
                                .chain(Some(neighbor))
                                .collect(),
                        ),
                        priority: new_heat_loss,
                    });
                }
            }
        }
    }

    if heat_losses.contains_key(&end) {
        let mut path =
            successors(Some(end), |current| prevs.get(current).copied()).collect::<Vec<_>>();
        path.reverse();

        Some(path)
    } else {
        None
    }
}

fn part_1(input: &Input) -> usize {
    let shortest = shortest_path(input, (0, 0), (input.width() - 1, input.height() - 1)).unwrap();
    println!(
        "{}",
        (0..input.height())
            .map(|row| (0..input.width())
                .map(|column| {
                    if let Some(index) =
                        shortest.iter().position(|(x, y)| x == &column && y == &row)
                    {
                        if index == 0 {
                            (input.get(shortest[0].0, shortest[0].1).unwrap() + b'0') as char
                        } else {
                            let current = shortest[index];
                            let prev = shortest[index - 1];
                            match (current.0.cmp(&prev.0), current.1.cmp(&prev.1)) {
                                (Ordering::Greater, _) => '>',
                                (Ordering::Less, _) => '<',
                                (_, Ordering::Greater) => 'v',
                                (_, Ordering::Less) => '^',
                                _ => panic!("Can't go diagonally!"),
                            }
                        }
                    } else {
                        (input.get(column, row).unwrap() + b'0') as char
                    }
                })
                .collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );
    shortest
        .into_iter()
        .skip(1)
        .map(|(column, row)| *input.get(column, row).unwrap() as usize)
        .sum()
}

// fn part_2(input: &Input) -> usize {
//     todo!()
// }

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        let part_1 = log_run("Part 1", || part_1(&input));
        println!("Part 1: {part_1}");

        // let part_2 = log_run("Part 2", || part_2(&input));
        // println!("Part 2: {part_2}");
    });
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const INPUT: &str = include_str!("test-input");
//
//     #[test]
//     fn test_part_1() {
//         let input = parse(INPUT.lines());
//         let result = part_1(&input);
//
//         assert_eq!(result, 102);
//     }
//
//     #[test]
//     fn test_part_2() {
//         let input = parse(INPUT.lines());
//         let result = part_2(&input);
//
//         assert_eq!(result, 0);
//     }
// }

use grid::*;
use std::collections::{HashMap, HashSet};
use std::io;
use std::iter::successors;

#[derive(Debug)]
enum Cell {
    Height(u32),
    Start,
    End,
}

impl Cell {
    fn height(&self) -> u32 {
        match self {
            Cell::Height(h) => *h,
            Cell::Start => 0,
            Cell::End => 25,
        }
    }
}

fn main() {
    let input: Grid<Cell> = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'S' => Cell::Start,
                    'E' => Cell::End,
                    _ if char.is_ascii_lowercase() => Cell::Height(char.to_digit(36).unwrap() - 10),
                    _ => panic!("Unexpected character: {char}"),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let part_1 = shortest_path(
        &input,
        input
            .coordinates()
            .find(|(x, y)| {
                if let Some(Cell::Start) = input.get(*x, *y) {
                    true
                } else {
                    false
                }
            })
            .unwrap(),
    )
    .expect("No path found...");
    print_grid_path(&input, &part_1);
    println!("Part 1: {}", part_1.len() - 1);

    let part_2 = input
        .coordinates()
        .filter(|(x, y)| input.get(*x, *y).unwrap().height() == 0)
        .filter_map(|start| shortest_path(&input, start))
        .min_by_key(|path| path.len())
        .expect("Couldn't find shortest path for part 2");
    print_grid_path(&input, &part_2);
    println!("Part 2: {}", part_2.len() - 1)
}

fn shortest_path(grid: &Grid<Cell>, start: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let end = grid
        .coordinates()
        .find(|(x, y)| {
            if let Some(Cell::End) = grid.get(*x, *y) {
                true
            } else {
                false
            }
        })
        .expect("No 'end' cell found.");

    let mut distances: HashMap<(usize, usize), Option<usize>> = grid
        .coordinates()
        .map(|c| (c, if c == start { Some(0) } else { None }))
        .collect();
    let mut unvisited: HashSet<(usize, usize)> = grid.coordinates().collect();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    while !unvisited.is_empty() {
        let current = unvisited
            .iter()
            .filter(|coord| distances[coord].is_some())
            .min_by_key(|coord| distances[coord])?
            .clone();
        unvisited.remove(&current);

        if current == end {
            break;
        }

        let current_height = grid.get(current.0, current.1).unwrap().height();

        [
            (current.0.checked_sub(1), Some(current.1)),
            (Some(current.0 + 1), Some(current.1)),
            (Some(current.0), current.1.checked_sub(1)),
            (Some(current.0), Some(current.1 + 1)),
        ]
        .iter()
        .filter_map(|(x, y)| {
            x.zip(*y)
                // Only cells with non-negative coordinates
                .map(|coord @ (x, y)| {
                    if unvisited.contains(&coord) // Only unvisited
                        && (0..=current_height + 1).contains(&grid.get(x, y).unwrap().height())
                    {
                        // Only if height is at most one higher than current
                        Some(coord)
                    } else {
                        None
                    }
                })
                .unwrap_or(None)
        })
        .for_each(|neighbor| {
            let distance = distances[&current].unwrap() + 1;
            distances.entry(neighbor).and_modify(|old_distance| {
                *old_distance = if old_distance.is_some() && old_distance.unwrap() >= distance {
                    *old_distance
                } else {
                    prev.entry(neighbor)
                        .and_modify(|prev| *prev = current)
                        .or_insert(current);
                    Some(distance)
                }
            });
        });
    }

    let mut prevs =
        successors(Some(end), |curr| prev.get(curr).map(|x| x.clone())).collect::<Vec<_>>();
    prevs.reverse();

    Some(prevs)
}

fn print_grid_path(grid: &Grid<Cell>, path: &Vec<(usize, usize)>) {
    (0..grid.height()).for_each(|y| {
        let row = (0..grid.width())
            .map(|x| {
                path.windows(2)
                    .map(|w| (w[0], w[1]))
                    .find(|(v, _)| v == &(x, y))
                    .map(|(_, next)| {
                        let delta_x = next.0.cmp(&x);
                        let delta_y = next.1.cmp(&y);

                        use std::cmp::Ordering::*;
                        match (delta_x, delta_y) {
                            (Less, _) => '<',
                            (Greater, _) => '>',
                            (_, Less) => '^',
                            (_, Greater) => 'v',
                            _ => panic!(),
                        }
                    })
                    .unwrap_or_else(|| match grid.get(x, y).unwrap() {
                        Cell::End => 'E',
                        _ => '.',
                    })
            })
            .collect::<String>();
        println!("{row}");
    });
}

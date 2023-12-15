use aoc_timing::trace::log_run;
use grid::*;
use std::collections::{BinaryHeap, HashMap};
use std::io;
use std::iter::{once, successors};

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

struct DijkstraVertex<T> {
    priority: usize,
    value: T,
}

impl<T> Eq for DijkstraVertex<T> {}
impl<T> PartialEq for DijkstraVertex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}
impl<T> PartialOrd for DijkstraVertex<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for DijkstraVertex<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority).reverse()
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

    let part_1 = log_run("Part 1", || {
        shortest_path(
            &input,
            input
                .coordinates()
                .find(|(x, y)| matches!(input.get(*x, *y), Some(Cell::Start)))
                .unwrap(),
        )
        .expect("No path found...")
    });
    // print_grid_path(&input, &part_1);
    println!("Part 1: {}", part_1.len() - 1);

    let part_2 = log_run("Part 2", || {
        input
            .coordinates()
            .filter(|(x, y)| input.get(*x, *y).unwrap().height() == 0)
            .filter_map(|start| shortest_path(&input, start))
            .min_by_key(|path| path.len())
            .expect("Couldn't find shortest path for part 2")
    });
    // print_grid_path(&input, &part_2);
    println!("Part 2: {}", part_2.len() - 1)
}

fn shortest_path(grid: &Grid<Cell>, start: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let end = grid
        .coordinates()
        .find(|(x, y)| matches!(grid.get(*x, *y), Some(Cell::End)))
        .expect("No 'end' cell found.");

    let mut distances: HashMap<(usize, usize), Option<usize>> = grid
        .coordinates()
        .map(|c| (c, if c == start { Some(0) } else { None }))
        .collect();
    let mut queue: BinaryHeap<DijkstraVertex<(usize, usize)>> = once(DijkstraVertex {
        priority: 0,
        value: start,
    })
    .collect();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    while let Some(DijkstraVertex {
        value: current,
        priority,
    }) = queue.pop()
    {
        if distances[&current]
            .map(|distance| distance != priority)
            .unwrap_or(true)
        {
            continue;
        }

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
                    if grid
                        .get(x, y)
                        .map(|cell| cell.height() <= current_height + 1)
                        .unwrap_or(false)
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
                if old_distance.is_none() || old_distance.unwrap() > distance {
                    *old_distance = Some(distance);
                    prev.insert(neighbor, current);
                    queue.push(DijkstraVertex {
                        priority: distance,
                        value: neighbor,
                    });
                }
            });
        });
    }

    distances[&end].map(|_| {
        let mut prevs = successors(Some(end), |curr| prev.get(curr).copied()).collect::<Vec<_>>();
        prevs.reverse();

        prevs
    })
}

#[allow(dead_code)]
fn print_grid_path(grid: &Grid<Cell>, path: &[(usize, usize)]) {
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

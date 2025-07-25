use aoc_timing::trace::log_run;
use grid::{Grid, LineOfSightNeighbors};
use std::io;

trait Day8Grid {
    fn is_cell_visible(&self, column: usize, row: usize) -> bool;
    fn scenic_score(&self, column: usize, row: usize) -> usize;
}

impl<T: PartialOrd> Day8Grid for Grid<T> {
    fn is_cell_visible(&self, column: usize, row: usize) -> bool {
        let LineOfSightNeighbors {
            left,
            right,
            up,
            down,
        } = self.get_line_of_sight_neighbors(column, row);

        [up, left, right, down]
            .into_iter()
            .any(|potential_blockers| {
                potential_blockers
                    .into_iter()
                    .all(|value| value < self.get(column, row).unwrap())
            })
    }

    fn scenic_score(&self, column: usize, row: usize) -> usize {
        let cell_value = self.get(column, row).unwrap();

        let LineOfSightNeighbors {
            mut left,
            right,
            mut up,
            down,
        } = self.get_line_of_sight_neighbors(column, row);

        left.reverse();
        up.reverse();

        [up, left, right, down]
            .into_iter()
            .map(|neighbors| {
                let neighbor_count = neighbors.len();

                neighbors
                    .into_iter()
                    .enumerate()
                    .find(|(_, value)| value >= &cell_value)
                    .map_or(neighbor_count, |(i, _)| i + 1)
            })
            .product()
    }
}

fn main() {
    env_logger::init();

    let grid: Grid<u8> = Grid::new(
        io::stdin()
            .lines()
            .map(|result| result.expect("I/O error"))
            .map(|line| {
                line.chars()
                    .map(|n| {
                        n.to_string()
                            .parse()
                            .unwrap_or_else(|_| panic!("{n} could not be parsed as a number"))
                    })
                    .collect()
            })
            .collect(),
    )
    .unwrap();

    let part_1 = log_run("Part 1", || {
        grid.coordinates()
            .filter(|(column, row)| grid.is_cell_visible(*column, *row))
            .count()
    });

    println!("Part 1: {part_1}");

    let part_2 = log_run("Part 2", || {
        grid.coordinates()
            .map(|(column, row)| grid.scenic_score(column, row))
            .max()
            .unwrap()
    });

    println!("Part 2: {part_2}");
}

use std::collections::BTreeSet;
use std::io;

use aoc_timing::trace::log_run;
use fxhash::FxHashMap;
use fxhash::FxHashSet;
use grid::Grid;
use rayon::prelude::*;

type Coord = (usize, usize);
type Input = FxHashMap<char, Vec<FxHashSet<Coord>>>;
type Output = usize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let grid = Grid::new(
        input
            .map(|line| {
                let line = line.as_ref();

                line.chars().collect()
            })
            .collect(),
    )
    .unwrap();

    flood_group(&grid)
}

/// Do a flood fill for all cells in the grid.
///
/// Basically, pick any cell, flood to find all other cells part of that region, and remove them
/// all from the grid. Continue until the grid is empty.
fn flood_group(grid: &Grid<char>) -> FxHashMap<char, Vec<FxHashSet<Coord>>> {
    let mut cells: BTreeSet<_> = grid
        .coordinates()
        .map(|coord @ (column, row)| (grid.get(column, row).unwrap(), coord))
        .collect();
    let mut result: FxHashMap<char, Vec<FxHashSet<Coord>>> =
        FxHashMap::with_hasher(Default::default());

    while let Some(current) = cells.pop_first() {
        let plant = current.0;
        let mut region = FxHashSet::with_hasher(Default::default());
        let mut queue = vec![current];

        while let Some(cell) = queue.pop() {
            if cell.0 == plant {
                cells.remove(&cell);
                region.insert(cell.1);

                queue.extend(
                    orthogonal_neighbors(&cell.1)
                        .into_iter()
                        .filter(|other| !region.contains(other))
                        .flat_map(|other| {
                            grid.get(other.0, other.1)
                                .map(|other_plant| (other_plant, other))
                        }),
                );
            }
        }

        if let Some(regions) = result.get_mut(plant) {
            regions.push(region);
        } else {
            result.insert(*plant, vec![region]);
        }
    }

    result
}

fn orthogonal_neighbors(cell: &Coord) -> Vec<Coord> {
    let mut result = Vec::with_capacity(4);
    result.push((cell.0 + 1, cell.1));
    result.push((cell.0, cell.1 + 1));
    if cell.0 > 0 {
        result.push((cell.0 - 1, cell.1));
    }
    if cell.1 > 0 {
        result.push((cell.0, cell.1 - 1));
    }

    result
}

/// Basically, the perimeter of a cell is 4 minus the count of neighboring cells in the same
/// region.
fn region_perimeter(region: &FxHashSet<Coord>) -> usize {
    region
        .iter()
        .map(|cell| {
            4 - region
                .iter()
                .filter(|other| {
                    (other.0.abs_diff(cell.0) == 1 && other.1 == cell.1)
                        || (other.0 == cell.0 && other.1.abs_diff(cell.1) == 1)
                })
                .count()
        })
        .sum()
}

/// The amount of sides for a region is equal to the amount of corners it has, so find the amount
/// of corners the region has.
///
/// This is probably a whole lot uglier than it has to be, but it comes down to the following,
/// where we're trying to figure out if there's a corner at the north-west of the cell in the
/// south-east (same letters indicate they're in the same region):
/// - ✅
///   ```
///   YY
///   YX
///   ```
/// - ✅
///   ```
///   XY
///   YX
///   ```
/// - ✅
///   ```
///   XY
///   YX
///   ```
/// - ❌
///   ```
///   XX
///   XX
///   ```
/// - ❌
///   ```
///   XX
///   YX
///   ```
/// - ❌
///   ```
///   YY
///   XX
///   ```
/// - ❌
///   ```
///   XY
///   XX
///   ```
/// - ❌
///   ```
///   YX
///   YX
///   ```
///
/// In a nutshell: either all three of north, north-west and west are not part of the same region,
/// or north and west are part of the same region and north-west isn't, or north-west is part of
/// the
fn region_sides(region: &FxHashSet<Coord>) -> usize {
    region
        .par_iter()
        .map(|cell| {
            let north = cell
                .1
                .checked_sub(1)
                .map(|row| region.contains(&(cell.0, row)))
                .unwrap_or(false);
            let north_east = cell
                .1
                .checked_sub(1)
                .map(|row| region.contains(&(cell.0 + 1, row)))
                .unwrap_or(false);
            let east = region.contains(&(cell.0 + 1, cell.1));
            let south_east = region.contains(&(cell.0 + 1, cell.1 + 1));
            let south = region.contains(&(cell.0, cell.1 + 1));
            let south_west = cell
                .0
                .checked_sub(1)
                .map(|column| region.contains(&(column, cell.1 + 1)))
                .unwrap_or(false);
            let west = cell
                .0
                .checked_sub(1)
                .map(|column| region.contains(&(column, cell.1)))
                .unwrap_or(false);
            let north_west = cell
                .0
                .checked_sub(1)
                .and_then(|column| {
                    cell.1
                        .checked_sub(1)
                        .map(|row| region.contains(&(column, row)))
                })
                .unwrap_or(false);

            [
                [!north, !north_east, !east],
                [north, !north_east, east],
                [!north, north_east, !east],
                [!east, !south_east, !south],
                [east, !south_east, south],
                [!east, south_east, !south],
                [!south, !south_west, !west],
                [south, !south_west, west],
                [!south, south_west, !west],
                [!west, !north_west, !north],
                [west, !north_west, north],
                [!west, north_west, !north],
            ]
            .into_iter()
            .filter(|corner| corner.iter().all(|b| *b))
            .count()
        })
        .sum()
}

fn part_1(input: &Input) -> Output {
    input
        .par_iter()
        .flat_map(|(_, regions)| {
            regions
                .par_iter()
                .map(|region| region.len() * region_perimeter(region))
        })
        .sum()
}

fn part_2(input: &Input) -> Output {
    input
        .par_iter()
        .flat_map(|(_, regions)| {
            regions
                .par_iter()
                .map(|region| region.len() * region_sides(region))
        })
        .sum()
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

        assert_eq!(result, 1930);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 1206);
    }
}

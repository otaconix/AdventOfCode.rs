use grid::Grid;
use rapidhash::HashMapExt;
use rapidhash::RapidHashMap;
use rapidhash::RapidHashSet;

#[derive(PartialEq)]
pub enum Cell {
    Empty,
    Splitter,
}

type Input = (usize, Grid<Cell>);
type Output1 = usize;
type Output2 = Output1;

pub fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let mut start = 0;
    let grid = input
        .map(|line| {
            let line = line.as_ref();

            line.chars()
                .enumerate()
                .map(|(column, c)| match c {
                    '.' => Cell::Empty,
                    '^' => Cell::Splitter,
                    'S' => {
                        start = column;
                        Cell::Empty
                    }
                    _ => panic!("Invalid cell {c}"),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    (start, grid)
}

pub fn part_1((start, grid): &Input) -> Output1 {
    let mut splits = 0;
    let mut beams = RapidHashSet::default();
    beams.insert(*start);

    for row in 0..grid.height() {
        beams = beams
            .into_iter()
            .flat_map(|beam| {
                let mut new_beams = vec![beam];
                if grid.get(beam, row).unwrap() == &Cell::Splitter {
                    splits += 1;
                    new_beams = vec![beam - 1, beam + 1];
                }

                new_beams.into_iter()
            })
            .collect::<RapidHashSet<_>>()
    }

    splits
}

pub fn part_2((start, grid): &Input) -> Output2 {
    fn inner(
        column: usize,
        row: usize,
        grid: &Grid<Cell>,
        memo: &mut RapidHashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(memoized) = memo.get(&(column, row)) {
            *memoized
        } else if row == grid.height() {
            1
        } else if grid.get(column, row).unwrap() == &Cell::Splitter {
            let left = inner(column - 1, row + 1, grid, memo);
            memo.insert((column - 1, row + 1), left);
            let right = inner(column + 1, row + 1, grid, memo);
            memo.insert((column + 1, row + 1), right);

            left + right
        } else {
            inner(column, row + 1, grid, memo)
        }
    }

    inner(*start, 0, grid, &mut RapidHashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 40);
    }
}

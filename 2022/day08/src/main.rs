use std::io;

#[derive(Debug)]
struct Grid<T> {
    width: usize,
    rows: Vec<Vec<T>>,
}

struct GridRow<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
    index: usize,
}

impl<'a, T> Iterator for GridRow<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;

        self.grid.rows.get(self.row).and_then(|row| row.get(index))
    }
}

struct GridColumn<'a, T> {
    grid: &'a Grid<T>,
    column: usize,
    index: usize,
}

impl<'a, T> Iterator for GridColumn<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;

        self.grid
            .rows
            .get(index)
            .and_then(|row| row.get(self.column))
    }
}

struct GridCoordinates<'a, T> {
    grid: &'a Grid<T>,
    column: usize,
    row: usize,
}

impl<'a, T> Iterator for GridCoordinates<'a, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.grid.rows.len() {
            None
        } else {
            let coord = (self.column, self.row);
            self.column = (self.column + 1) % self.grid.width;
            if self.column == 0 {
                self.row += 1;
            }

            Some(coord)
        }
    }
}

#[derive(Debug)]
enum GridCreationError {
    UnequalRowLengths,
}

struct LineOfSightNeighbors<'a, T> {
    left: Vec<&'a T>,
    right: Vec<&'a T>,
    up: Vec<&'a T>,
    down: Vec<&'a T>,
}

impl<T> Grid<T> {
    fn new(rows: Vec<Vec<T>>) -> Result<Grid<T>, GridCreationError> {
        let width = rows.first().map(|firstrow| firstrow.len()).unwrap_or(0);

        if rows.iter().any(|row| row.len() != width) {
            Err(GridCreationError::UnequalRowLengths)
        } else {
            Ok(Grid { rows, width })
        }
    }

    fn get(&self, column: usize, row: usize) -> Option<&T> {
        self.rows.get(row).and_then(|row| row.get(column))
    }

    fn coordinates(&self) -> GridCoordinates<T> {
        GridCoordinates {
            grid: self,
            row: 0,
            column: 0,
        }
    }

    fn row(&self, row: usize) -> GridRow<T> {
        GridRow {
            grid: self,
            row,
            index: 0,
        }
    }

    fn column(&self, column: usize) -> GridColumn<T> {
        GridColumn {
            grid: self,
            column,
            index: 0,
        }
    }

    fn get_line_of_sight_neighbors(&self, column: usize, row: usize) -> LineOfSightNeighbors<T> {
        let (left, right): (Vec<_>, Vec<_>) = self
            .row(row)
            .enumerate()
            .filter(|(i, _)| i != &column)
            .partition(|(i, _)| i < &column);

        let (up, down): (Vec<_>, Vec<_>) = self
            .column(column)
            .enumerate()
            .filter(|(i, _)| i != &row)
            .partition(|(i, _)| i < &row);

        fn remove_indices<U>(v: Vec<(usize, U)>) -> Vec<U> {
            v.into_iter().map(|(_, t)| t).collect()
        }

        LineOfSightNeighbors {
            up: remove_indices(up),
            left: remove_indices(left),
            right: remove_indices(right),
            down: remove_indices(down),
        }
    }
}

impl<T: PartialOrd> Grid<T> {
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
                    .all(|value| value < &self.get(column, row).unwrap())
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
                    .map(|(i, _)| i + 1)
                    .unwrap_or(neighbor_count)
            })
            .product()
    }
}

fn main() {
    let grid: Grid<u8> = Grid::new(
        io::stdin()
            .lines()
            .map(|result| result.expect("I/O error"))
            .map(|line| {
                line.chars()
                    .map(|n| {
                        n.to_string()
                            .parse()
                            .expect(&format!("{} could not be parsed as a number", n))
                    })
                    .collect()
            })
            .collect(),
    )
    .unwrap();

    let part_1 = grid
        .coordinates()
        .filter(|(column, row)| grid.is_cell_visible(*column, *row))
        .count();

    println!("Part 1: {}", part_1);

    let part_2 = grid
        .coordinates()
        .map(|(column, row)| grid.scenic_score(column, row))
        .max()
        .unwrap();

    println!("Part 2: {}", part_2);
}

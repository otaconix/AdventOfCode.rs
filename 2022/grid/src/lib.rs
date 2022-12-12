#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    rows: Vec<Vec<T>>,
}

pub struct GridRow<'a, T> {
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

pub struct GridColumn<'a, T> {
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

pub struct GridCoordinates<'a, T> {
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
pub enum GridCreationError {
    UnequalRowLengths,
}

pub struct LineOfSightNeighbors<'a, T> {
    pub left: Vec<&'a T>,
    pub right: Vec<&'a T>,
    pub up: Vec<&'a T>,
    pub down: Vec<&'a T>,
}

impl<T> Grid<T> {
    pub fn new(rows: Vec<Vec<T>>) -> Result<Grid<T>, GridCreationError> {
        let width = rows.first().map(|firstrow| firstrow.len()).unwrap_or(0);

        if rows.iter().any(|row| row.len() != width) {
            Err(GridCreationError::UnequalRowLengths)
        } else {
            Ok(Grid { rows, width })
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn get(&self, column: usize, row: usize) -> Option<&T> {
        self.rows.get(row).and_then(|row| row.get(column))
    }

    pub fn coordinates(&self) -> GridCoordinates<T> {
        GridCoordinates {
            grid: self,
            row: 0,
            column: 0,
        }
    }

    pub fn row(&self, row: usize) -> GridRow<T> {
        GridRow {
            grid: self,
            row,
            index: 0,
        }
    }

    pub fn column(&self, column: usize) -> GridColumn<T> {
        GridColumn {
            grid: self,
            column,
            index: 0,
        }
    }

    pub fn get_line_of_sight_neighbors(
        &self,
        column: usize,
        row: usize,
    ) -> LineOfSightNeighbors<T> {
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

impl<T> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        let rows = iter.into_iter().collect();

        Grid::new(rows).unwrap()
    }
}

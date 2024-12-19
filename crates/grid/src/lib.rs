use std::iter::successors;
use std::{fmt::Debug, hash::Hash};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    width: usize,
    rows: Vec<Vec<T>>,
}

pub struct GridRow<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
    index: usize,
    index_back: usize,
}

impl<'a, T> Iterator for GridRow<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;

        self.grid.rows.get(self.row).and_then(|row| row.get(index))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();

        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for GridRow<'_, T> {
    fn len(&self) -> usize {
        self.index_back - self.index
    }
}

impl<T> DoubleEndedIterator for GridRow<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index == self.index_back {
            None
        } else {
            self.index_back -= 1;

            self.grid
                .rows
                .get(self.row)
                .and_then(|row| row.get(self.index_back))
        }
    }
}

pub struct GridColumn<'a, T> {
    grid: &'a Grid<T>,
    column: usize,
    index: usize,
    index_back: usize,
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for GridColumn<'_, T> {
    fn len(&self) -> usize {
        self.index_back - self.index
    }
}

impl<T> DoubleEndedIterator for GridColumn<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index == self.index_back {
            None
        } else {
            self.index_back -= 1;

            self.grid
                .rows
                .get(self.index_back)
                .and_then(|row| row.get(self.column))
        }
    }
}

#[derive(Clone)]
pub struct GridCoordinates<'a, T> {
    grid: &'a Grid<T>,
    column: usize,
    row: usize,
}

impl<T> Iterator for GridCoordinates<'_, T> {
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

    pub fn with_size(width: usize, height: usize) -> Self
    where
        T: Sized + Default,
    {
        Self::new(
            (0..height)
                .map(|_| {
                    successors(Some(T::default()), |_| Some(T::default()))
                        .take(width)
                        .collect()
                })
                .collect(),
        )
        .unwrap()
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
            index_back: self.width(),
        }
    }

    pub fn column(&self, column: usize) -> GridColumn<T> {
        GridColumn {
            grid: self,
            column,
            index: 0,
            index_back: self.height(),
        }
    }

    pub fn update(&mut self, column: usize, row: usize, value: T) {
        self.rows[row][column] = value;
    }

    pub fn get_neighbors(&self, column: usize, row: usize) -> Vec<(usize, usize)> {
        let left = column.checked_sub(1).map(|x| (x, row));
        let right = Some((column + 1, row)).filter(|(x, _)| x < &self.width());
        let up = row.checked_sub(1).map(|y| (column, y));
        let down = Some((column, row + 1)).filter(|(_, y)| y < &self.height());

        [left, right, up, down].into_iter().flatten().collect()
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

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height() {
            writeln!(f, "{:?}", self.rows[row])?;
        }

        Ok(())
    }
}

impl<T> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        let rows = iter.into_iter().collect();

        Grid::new(rows).unwrap()
    }
}

impl<T: Clone> Grid<T> {
    pub fn transpose(&self) -> Self {
        (0..self.width())
            .map(|column| self.column(column).cloned().collect::<Vec<_>>())
            .collect()
    }
}

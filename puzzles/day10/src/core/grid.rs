use nalgebra::{
    iter::{ColumnIter, RowIter},
    DMatrix, Dyn, VecStorage,
};

use std::fmt::Debug;
use std::ops::{Index, IndexMut};

use super::{Direction, Location};

pub type Matrix<T> = DMatrix<T>;
pub type Cell<'a, T> = (Location, &'a T);

fn neighbor((row, column): Location, direction: Direction) -> Option<Location> {
    let (vertical, horizontal) = direction.offset();
    row.checked_add_signed(vertical)
        .zip(column.checked_add_signed(horizontal))
}

#[derive(Debug, Clone)]
pub struct Grid<T>(Matrix<T>);

impl<T> Grid<T> {
    pub fn size(&self) -> (usize, usize) {
        self.0.shape()
    }

    pub fn get(&self, location: Location) -> Option<&T> {
        self.0.get(location)
    }

    pub fn get_mut(&mut self, location: Location) -> Option<&mut T> {
        self.0.get_mut(location)
    }

    pub fn row_iter(&self) -> RowIter<T, Dyn, Dyn, VecStorage<T, Dyn, Dyn>> {
        self.0.row_iter()
    }

    pub fn column_iter(&self) -> ColumnIter<T, Dyn, Dyn, VecStorage<T, Dyn, Dyn>> {
        self.0.column_iter()
    }

    pub fn neighbor(&self, location: Location, direction: Direction) -> Option<Cell<T>> {
        neighbor(location, direction)
            .and_then(|adjacent| self.0.get(adjacent).map(|value| (adjacent, value)))
    }
}

impl<T> From<Matrix<T>> for Grid<T> {
    fn from(matrix: Matrix<T>) -> Self {
        Self(matrix)
    }
}

impl<T> Index<Location> for Grid<T> {
    type Output = T;

    fn index(&self, location: Location) -> &Self::Output {
        &self.0[location]
    }
}

impl<T> IndexMut<Location> for Grid<T> {
    fn index_mut(&mut self, location: Location) -> &mut Self::Output {
        &mut self.0[location]
    }
}

impl<T: Debug + Clone + PartialEq + 'static> Grid<T> {
    pub fn from_iter(height: usize, width: usize, iter: impl IntoIterator<Item = T>) -> Self {
        Self(DMatrix::from_row_iterator(height, width, iter))
    }
}

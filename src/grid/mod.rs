use std::{borrow::Borrow, collections::HashMap, ops::Add};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct Extent {
    north: isize,
    south: isize,
    east: isize,
    west: isize,
}

impl Extent {
    pub fn contain<C: Borrow<Cell>>(&mut self, loc: &C) {
        let (x, y) = loc.borrow().as_tuple();
        if x > self.east {
            self.east = x;
        }
        if x < self.west {
            self.west = x;
        }
        if y > self.north {
            self.north = y;
        }
        if y < self.south {
            self.south = y;
        }
    }
}

pub(crate) struct Grid<T: Clone + Default> {
    data: HashMap<Cell, T>,
    extent: Extent,
}

impl<V: Clone + Default> Grid<V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            extent: Default::default(),
        }
    }

    pub fn insert<C: Borrow<Cell>>(&mut self, cell: &C, data: V) -> Option<V> {
        let cell = cell.borrow();
        self.extent.contain(&cell);
        self.data.insert(cell.clone(), data)
    }

    pub fn get<C: Borrow<Cell>>(&self, cell: &C) -> Option<&V> {
        let cell = cell.borrow();
        self.data.get(cell)
    }

    pub fn rows_iter(&self) -> RowsIter<V> {
        RowsIter {
            grid: &self,
            current_y: self.extent.north,
            end_y: self.extent.south,
            west: self.extent.west,
            east: self.extent.east,
        }
    }
}

pub(crate) struct RowsIter<'a, V: Clone + Default> {
    grid: &'a Grid<V>,
    current_y: isize,
    end_y: isize,
    west: isize,
    east: isize,
}

impl<'a, V: Clone + Default> Iterator for RowsIter<'a, V> {
    type Item = RowIter<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y >= self.end_y {
            let row = RowIter {
                grid: self.grid,
                y: self.current_y,
                current_x: self.west,
                end_x: self.east,
            };
            self.current_y -= 1;
            Some(row)
        } else {
            None
        }
    }
}

pub(crate) struct RowIter<'a, V: Clone + Default> {
    grid: &'a Grid<V>,
    y: isize,
    current_x: isize,
    end_x: isize,
}

impl<'a, V: Clone + Default> RowIter<'a, V> {
    pub fn enumerate(self) -> RowIterWithCell<'a, V> {
        RowIterWithCell { inner: self }
    }
}

impl<'a, V: Clone + Default> Iterator for RowIter<'a, V> {
    type Item = Option<&'a V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_x <= self.end_x {
            let cell = Cell(self.current_x, self.y);
            let value = self.grid.get(&cell);
            self.current_x += 1;
            Some(value)
        } else {
            None
        }
    }
}

pub(crate) struct RowIterWithCell<'a, V: Clone + Default> {
    inner: RowIter<'a, V>,
}

impl<'a, V: Clone + Default> Iterator for RowIterWithCell<'a, V> {
    type Item = (Cell, Option<&'a V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|value| {
            let cell = Cell(self.inner.current_x - 1, self.inner.y);
            (cell, value)
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub(crate) struct Cell(pub isize, pub isize);

impl Cell {
    pub fn as_tuple(&self) -> (isize, isize) {
        (self.0, self.1)
    }
}

impl<C: Borrow<Cell>> Add<C> for Cell {
    type Output = Cell;

    fn add(self, rhs: C) -> Self::Output {
        Cell(self.0 + rhs.borrow().0, self.1 + rhs.borrow().1)
    }
}

impl From<(isize, isize)> for Cell {
    fn from(value: (isize, isize)) -> Self {
        Self(value.0, value.1)
    }
}

impl Borrow<Cell> for (isize, isize) {
    fn borrow(&self) -> &Cell {
        // This is safe because Cell and (isize, isize) have the same memory layout
        unsafe { &*(self as *const (isize, isize) as *const Cell) }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum Turn {
    Left,
    Right,
}

impl Direction {
    pub fn turn(&self, t: Turn) -> Self {
        match t {
            Turn::Left => match self {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            },
            Turn::Right => match self {
                Direction::North => Direction::East,
                Direction::West => Direction::North,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
            },
        }
    }
}

impl From<Direction> for Cell {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => (0, 1),
            Direction::South => (0, -1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn test_grid() {
        let mut grid: Grid<usize> = Grid::new();
        grid.insert(&(3, 5), 100);
        assert_eq!(grid.get(&(3, 5)), Some(&100))
    }
}

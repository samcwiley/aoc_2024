use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Result<Self, &'static str> {
        if data.is_empty() {
            return Ok(Self {
                data: Vec::new(),
                width: 0,
                height: 0,
            });
        }
        let width = data[0].len();
        if data.iter().any(|row| row.len() != width) {
            return Err("Rows must have equal length");
        }
        let height = data.len();
        Ok(Self {
            data,
            width,
            height,
        })
    }

    /// Returns a vector of vectors representing all directions (up, down, left, right, and diagonals)
    /// starting from `(x, y)` with `n` elements in each direction.
    pub fn search(&self, x: usize, y: usize, n: usize) -> Vec<Vec<T>> {
        let mut directions = Vec::new();

        let deltas = [
            (0, -1),  // Up
            (0, 1),   // Down
            (-1, 0),  // Left
            (1, 0),   // Right
            (-1, -1), // Up-left
            (1, -1),  // Up-right
            (-1, 1),  // Down-left
            (1, 1),   // Down-right
        ];

        for &(dx, dy) in &deltas {
            let mut path = Vec::new();
            for i in 0..n {
                let nx = x as isize + i as isize * dx;
                let ny = y as isize + i as isize * dy;

                if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
                    path.push(self[(nx as usize, ny as usize)].clone());
                } else {
                    break;
                }
            }
            directions.push(path);
        }

        directions
    }
}

impl Grid<u8> {
    /// Parses a &str into a Grid<u8>, where each character in the string is converted to its ascii byte value
    pub fn parse_grid(input: &str) -> Result<Self, &'static str> {
        let data: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

        let width = data.get(0).map_or(0, |row| row.len());
        if data.iter().any(|row| row.len() != width) {
            return Err("All lines must have the same length");
        }

        Grid::new(data)
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for (i, item) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", item)?;
            }
            writeln!(f)?; // Move to the next line after each row
        }
        Ok(())
    }
}

// for indexing the grid, using a tuple of coordinates, like grid_ex[(x, y)]
impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[y][x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[y][x]
    }
}

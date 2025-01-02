use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone + PartialEq> Grid<T> {
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
    pub fn search_grid(&self, value: &T) -> Option<(usize, usize)> {
        for (i, row) in self.data.iter().enumerate() {
            for (j, point) in row.iter().enumerate() {
                if point == value {
                    return Some((j, i));
                }
            }
        }
        None
    }

    /// Returns a vector of vectors representing all directions (up, down, left, right, and diagonals)
    /// starting from `(x, y)` with `n` elements in each direction.
    pub fn check_directions(&self, x: usize, y: usize, n: usize) -> Vec<Vec<T>> {
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

impl Grid<Point> {
    pub fn parse_grid(input: &str) -> Result<Self, &'static str> {
        let input: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
        let mut data: Vec<Vec<Point>> = Vec::new();

        for row in input.iter() {
            let mut grid_row = Vec::new();
            for &datum in row.iter() {
                let point = match datum {
                    b'.' => Point::Empty,
                    b'#' => Point::Obstacle,
                    b'^' => Point::Guy(Direction::Up),
                    _ => Point::Empty,
                };
                grid_row.push(point);
            }
            data.push(grid_row);
        }

        let height = data.len();
        let width = data.get(0).map_or(0, |row| row.len());
        if data.iter().any(|row| row.len() != width) {
            return Err("All lines must have the same length");
        }

        Ok(Self {
            data,
            width,
            height,
        })
    }

    pub fn grid_step(
        &mut self,
        coords: &mut (usize, usize),
    ) -> Option<(&mut Self, (usize, usize), bool)> {
        let mut visited = false;
        let (guy_x, guy_y) = *coords;
        println!("Guy is at {}, {}", guy_x, guy_y);
        let current_point = self[(guy_x, guy_y)];
        let current_direction: Direction = if let Point::Guy(direction) = current_point {
            direction
        } else {
            Direction::Up
        };
        dbg!(current_direction);

        let delta: (isize, isize) = match current_direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let (new_x, new_y) = (guy_x as isize + delta.0, guy_y as isize + delta.1);

        if new_x < 0 || new_y < 0 || new_x as usize >= self.width || new_y as usize >= self.height {
            return None; // Elvis has left the building
        }

        let (new_x, new_y) = (new_x as usize, new_y as usize);
        println!("We are going to: {}, {}", new_x, new_y);

        if self[(new_x, new_y)] == Point::Visited {
            self[(new_x, new_y)] = Point::Guy(current_direction.clone());
            self[(guy_x, guy_y)] = Point::Visited;
            visited = true;
        } else if self[(new_x, new_y)] == Point::Empty {
            println!("Time to move!");
            println!("{guy_x}, {guy_y} will be visited, {new_x}, {new_y} is our new location");
            self[(new_x, new_y)] = Point::Guy(current_direction.clone());
            self[(guy_x, guy_y)] = Point::Visited;
        } else if self[(new_x, new_y)] == Point::Obstacle {
            let new_direction = match current_direction {
                Direction::Up => Direction::Right,   // turn right
                Direction::Right => Direction::Down, // turn down
                Direction::Down => Direction::Left,  // turn left
                Direction::Left => Direction::Up,    // turn up
            };
            self[(guy_x, guy_y)] = Point::Guy(new_direction);
        }
        println!("The new grid will look like {}", self);
        Some((self, (new_x, new_y), visited))
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

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum Point {
    Guy(Direction),
    Obstacle,
    Empty,
    Visited,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let representation = match self {
            Point::Guy(direction) => match direction {
                Direction::Up => "^",
                Direction::Down => "v",
                Direction::Left => "<",
                Direction::Right => ">",
            },
            Point::Obstacle => "#",
            Point::Empty => ".",
            Point::Visited => "*",
        };
        write!(f, "{}", representation)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

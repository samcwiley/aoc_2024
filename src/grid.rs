use std::collections::HashSet;
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
    pub fn find_first(&self, value: &T) -> Option<(usize, usize)> {
        for (i, row) in self.data.iter().enumerate() {
            for (j, point) in row.iter().enumerate() {
                if point == value {
                    return Some((j, i));
                }
            }
        }
        None
    }

    pub fn find_all(&self, value: &T) -> Option<Vec<(usize, usize)>> {
        let mut locs = Vec::<(usize, usize)>::new();
        for (i, row) in self.data.iter().enumerate() {
            for (j, point) in row.iter().enumerate() {
                if point == value {
                    locs.push((j, i));
                }
            }
        }
        if !locs.is_empty() {
            return Some(locs);
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

    pub fn is_valid_point(&self, point: (isize, isize)) -> bool {
        point.0 >= 0
            && point.0 < self.width as isize
            && point.1 >= 0
            && point.1 < self.height as isize
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

    pub fn find_unique_values(&self, exclusions: Option<Vec<u8>>) -> HashSet<u8> {
        let mut uniques = HashSet::<u8>::new();
        for row in &self.data {
            for val in row {
                uniques.insert(*val);
            }
        }
        if let Some(to_remove) = exclusions {
            for removal in to_remove {
                uniques.remove(&removal);
            }
        }
        uniques
    }

    pub fn print_as_chars(&self) {
        for row in &self.data {
            let s = String::from_utf8_lossy(row);
            println!("{}", s);
        }
    }

    /*impl fmt::Display for Grid<u8> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for row in &self.data {
                let s = String::from_utf8_lossy(row);
                writeln!(f, "{}", s)?;
            }
            Ok(())
        }
    }*/
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
                    b'#' => Point::Obstacle(Hit::new()),
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

    pub fn run_grid(&self, coords: (usize, usize)) -> Option<u32> {
        let mut grid: Grid<Point> = self.clone();
        let (mut guy_x, mut guy_y) = coords.clone();
        let mut spaces = 0;
        loop {
            let current_point = grid[(guy_x, guy_y)];
            let current_direction: Direction = if let Point::Guy(direction) = current_point {
                direction
            } else {
                Direction::Up
            };

            let delta: (isize, isize) = match current_direction {
                Direction::Up => (0, -1),
                Direction::Down => (0, 1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };

            let (new_x, new_y) = (guy_x as isize + delta.0, guy_y as isize + delta.1);

            if new_x < 0
                || new_y < 0
                || new_x as usize >= grid.width
                || new_y as usize >= grid.height
            {
                break; // Elvis has left the building
            }

            let (new_x, new_y) = (new_x as usize, new_y as usize);

            match &mut grid[(new_x, new_y)] {
                Point::Visited => {
                    grid[(new_x, new_y)] = Point::Guy(current_direction.clone());
                    grid[(guy_x, guy_y)] = Point::Visited;
                    (guy_x, guy_y) = (new_x, new_y);
                }
                Point::Empty => {
                    grid[(new_x, new_y)] = Point::Guy(current_direction.clone());
                    grid[(guy_x, guy_y)] = Point::Visited;
                    (guy_x, guy_y) = (new_x, new_y);
                    spaces += 1;
                }
                Point::Obstacle(hit_directions) => {
                    if hit_directions.hit(current_direction) {
                        return None; // we are in a loop!
                    } else {
                        let new_direction = match current_direction {
                            Direction::Up => Direction::Right,   // turn right
                            Direction::Right => Direction::Down, // turn down
                            Direction::Down => Direction::Left,  // turn left
                            Direction::Left => Direction::Up,    // turn up
                        };
                        grid[(guy_x, guy_y)] = Point::Guy(new_direction);
                    }
                }
                Point::Guy(_) => continue,
            }
        }
        Some(spaces)
    }
    pub fn part_2(&self, coords: (usize, usize)) -> u32 {
        let mut grid: Grid<Point> = self.clone();
        let (mut guy_x, mut guy_y) = coords.clone();
        let mut obstacles = 0;
        loop {
            let current_point = grid[(guy_x, guy_y)];
            let current_direction: Direction = if let Point::Guy(direction) = current_point {
                direction
            } else {
                Direction::Up
            };

            let delta: (isize, isize) = match current_direction {
                Direction::Up => (0, -1),
                Direction::Down => (0, 1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };

            let (new_x, new_y) = (guy_x as isize + delta.0, guy_y as isize + delta.1);

            if new_x < 0
                || new_y < 0
                || new_x as usize >= grid.width
                || new_y as usize >= grid.height
            {
                break; // Elvis has left the building
            }

            let (new_x, new_y) = (new_x as usize, new_y as usize);

            match &mut grid[(new_x, new_y)] {
                Point::Visited => {
                    grid[(new_x, new_y)] = Point::Guy(current_direction.clone());
                    grid[(guy_x, guy_y)] = Point::Visited;
                    (guy_x, guy_y) = (new_x, new_y);
                }
                Point::Empty => {
                    let mut new_grid = self.clone();
                    new_grid[(new_x, new_y)] = Point::Obstacle(Hit::new());
                    if let Some(_) = new_grid.run_grid(coords) {
                        ()
                    } else {
                        obstacles += 1;
                    }

                    grid[(new_x, new_y)] = Point::Guy(current_direction.clone());
                    grid[(guy_x, guy_y)] = Point::Visited;
                    (guy_x, guy_y) = (new_x, new_y);
                }
                Point::Obstacle(_) => {
                    let new_direction = match current_direction {
                        Direction::Up => Direction::Right,   // turn right
                        Direction::Right => Direction::Down, // turn down
                        Direction::Down => Direction::Left,  // turn left
                        Direction::Left => Direction::Up,    // turn up
                    };
                    grid[(guy_x, guy_y)] = Point::Guy(new_direction);
                }
                Point::Guy(_) => continue,
            }
        }
        obstacles
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
    Obstacle(Hit),
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
            Point::Obstacle(_) => "#",
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

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub struct Hit {
    up: bool,
    left: bool,
    right: bool,
    down: bool,
}

impl Hit {
    pub fn new() -> Self {
        Self {
            up: false,
            left: false,
            right: false,
            down: false,
        }
    }
    pub fn hit(&mut self, from: Direction) -> bool {
        //dbg!(&self);
        //dbg!(from);
        match from {
            Direction::Up => {
                if self.up {
                    return true;
                }
                self.up = true;
            }
            Direction::Left => {
                if self.left {
                    return true;
                }
                self.left = true;
            }
            Direction::Right => {
                if self.right {
                    return true;
                }
                self.right = true;
            }
            Direction::Down => {
                if self.down {
                    return true;
                }
                self.down = true;
            }
        }
        false
    }
}

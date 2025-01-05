#![allow(unused_variables, dead_code, unused_imports)]
use aoc_2024::grid::{Direction, Grid, Point};

fn main() {
    let input = include_str!("../.inputs/input06.txt");
    let (data, starting_coords) = parse_input(input);
    let part1 = part1(&data, starting_coords);
    dbg!(part1);
    let part2 = part2(&data, starting_coords);
    dbg!(part2);
}

fn parse_input(input: &str) -> (Grid<Point>, (usize, usize)) {
    let grid: Grid<Point> = Grid::<Point>::parse_grid(input).unwrap();
    let coords = grid.find_first(&Point::Guy(Direction::Up)).unwrap();
    (grid, coords)
}

fn part1(input: &Grid<Point>, coords: (usize, usize)) -> String {
    (input.run_grid(coords).unwrap() + 1).to_string() // adding 1 to account for the last space while leaving the warehouse
}

fn part2(input: &Grid<Point>, coords: (usize, usize)) -> String {
    input.part_2(coords).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ex_input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let ex_answer = "41";
        let (ex_data, starting_coords) = parse_input(ex_input);
        let result = part1(&ex_data, starting_coords);
        assert_eq!(result, ex_answer);
        let ex_answer2 = "6";
        let result2 = part2(&ex_data, starting_coords);
        assert_eq!(result2, ex_answer2);
    }
}

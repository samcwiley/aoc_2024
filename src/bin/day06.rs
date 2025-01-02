#![allow(unused_variables, dead_code, unused_imports)]
use aoc_2024::grid::{Direction, Grid, Point};

fn main() {
    //let input = include_str!("../.inputs/input06.txt");
    //let data = parse_input(input);
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
    //let ex_answer = "41";
    let (ex_data, starting_coords) = parse_input(ex_input);
    println!("{}, {}", starting_coords.0, starting_coords.1);
    let result = part1(&ex_data, starting_coords);
    //let part1 = part1(&data);
    //dbg!(part1);
    //let part2 = part2(&data);
    //dbg!(part2);
}

fn parse_input(input: &str) -> (Grid<Point>, (usize, usize)) {
    let grid: Grid<Point> = Grid::<Point>::parse_grid(input).unwrap();
    let coords = grid.search_grid(&Point::Guy(Direction::Up)).unwrap();
    (grid, coords)
}

fn part1(input: &Grid<Point>, coords: (usize, usize)) -> String {
    let mut grid = (*input).clone();
    let mut spaces = 0u32;
    let mut coords = coords;
    while let Some((grid, coords, visited)) = grid.grid_step(&mut coords) {
        if !visited {
            spaces += 1;
        }
    }
    spaces.to_string()
}

/*fn part2(input: &Vec<&str>) -> String {
    "todo!()".to_string()
}
*/

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
        /*let ex_answer2 = "";
        let result2 = part2(&ex_data);
        assert_eq!(result2, ex_answer2);
        */
    }
}

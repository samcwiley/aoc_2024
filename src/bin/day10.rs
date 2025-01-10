#![allow(unused_variables, dead_code, unused_imports)]
use aoc_2024::grid::{Grid, Point};
use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

fn main() {
    let input = include_str!("../.inputs/input10.txt");
    let data = parse_input(input);
    let part1 = part1(&data);
    dbg!(part1);
    let part2 = part2(&data);
    dbg!(part2);
}

fn full_trails(grid: &Grid<u8>, start_pos: Point) -> Vec<Point> {
    let mut points = VecDeque::from([start_pos]);
    let mut visited = Vec::new();
    while let Some(point) = points.pop_front() {
        if grid[point] == 9 {
            visited.push(point);
            continue;
        }
        for new_point in [
            point.move_by(1, 0),
            point.move_by(-1, 0),
            point.move_by(0, 1),
            point.move_by(0, -1),
        ] {
            if let Some(new_point) = new_point {
                if grid.is_valid_point(new_point) && grid[new_point] == grid[point] + 1 {
                    points.push_back(new_point);
                }
            }
        }
    }
    visited
}

type Input = (Grid<u8>, Vec<Point>);

fn parse_input(raw: &str) -> Input {
    let trailhead_marker = 0u8;
    let grid = Grid::parse_grid_nums(raw).unwrap();
    let trailheads = grid.find_all_points(&trailhead_marker).unwrap();
    (grid, trailheads)
}

fn part1(input: &Input) -> String {
    let (grid, trailheads) = input;
    let mut trail_count = 0;
    for trailhead in trailheads {
        let visited = full_trails(grid, *trailhead);
        trail_count += visited.iter().collect::<HashSet<_>>().len();
    }
    trail_count.to_string()
}

fn part2(input: &Input) -> String {
    let (grid, trailheads) = input;
    let mut trail_count = 0;
    for trailhead in trailheads {
        let visited = full_trails(grid, *trailhead);
        trail_count += visited.len();
    }
    trail_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    const EX_INPUT_2: &str = "8880888
8881888
8882888
6543456
7888887
8888888
9888889";

    #[test]
    fn test_part_1() {
        let ex_answer = "36";
        let ex_data = parse_input(EX_INPUT);
        let result = part1(&ex_data);
        assert_eq!(result, ex_answer);
    }

    #[test]
    fn test_part_1_easy() {
        let ex_answer = "2";
        let ex_data = parse_input(EX_INPUT_2);
        let result = part1(&ex_data);
        assert_eq!(result, ex_answer);
    }

    #[test]
    fn test_part_2() {
        let ex_answer_2 = "81";
        let ex_data = parse_input(EX_INPUT);
        let result2 = part2(&ex_data);
        assert_eq!(result2, ex_answer_2);
    }
}

#![allow(unused_variables, dead_code, unused_imports)]
use aoc_2024::grid::Grid;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../.inputs/input08.txt");
    let data = parse_input(input);
    let part1 = part1(&data);
    dbg!(part1);
    let part2 = part2(&data);
    dbg!(part2);
}

fn parse_input(input: &str) -> Grid<u8> {
    Grid::<u8>::parse_grid(input).unwrap()
}

fn part1(input: &Grid<u8>) -> String {
    let mut input = input.clone();
    let wavelengths = input.find_unique_values(Some(vec![b'.']));
    let mut result = 0;
    wavelengths.iter().for_each(|wavelength| {
        let locs = input.find_all(wavelength).unwrap();
        let wave_node_sum = locs
            .iter()
            .enumerate()
            .flat_map(|(i, &x)| locs[i..].iter().map(move |&y| (x, y)))
            .map(|(loc_1, loc_2)| {
                let mut antinodes = 0;
                let (dx, dy) = (
                    loc_1.0 as isize - loc_2.0 as isize,
                    loc_1.1 as isize - loc_2.1 as isize,
                );
                let (antinode_1, antinode_2) = (
                    (loc_1.0 as isize + dx, loc_1.1 as isize + dy),
                    (loc_2.0 as isize - dx, loc_2.1 as isize - dy),
                );

                if input.is_valid_point(antinode_1) {
                    let (x1, y1) = (antinode_1.0 as usize, antinode_1.1 as usize);
                    if input[(x1, y1)] == b'.' || input[(x1, y1)] == b'#' {
                        input[(x1, y1)] = b'#';
                        antinodes += 1;
                    }
                }

                if input.is_valid_point(antinode_2) {
                    let (x2, y2) = (antinode_2.0 as usize, antinode_2.1 as usize);
                    if input[(x2, y2)] == b'.' || input[(x2, y2)] == b'#' {
                        input[(x2, y2)] = b'#';
                        antinodes += 1;
                    }
                }
                antinodes
            })
            .sum::<i32>();
        result += wave_node_sum;
    });
    result.to_string()
}

fn part2(input: &Grid<u8>) -> String {
    "todo!()".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part_1() {
        let ex_answer = "14";
        let ex_data = parse_input(EX_INPUT);
        let result = part1(&ex_data);
        assert_eq!(result, ex_answer);
    }

    #[test]
    fn test_part_2() {
        let ex_answer_2 = "";
        let ex_data = parse_input(EX_INPUT);
        let result2 = part2(&ex_data);
        assert_eq!(result2, ex_answer_2);
    }
}

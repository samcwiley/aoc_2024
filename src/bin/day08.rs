#![allow(unused_variables, dead_code, unused_imports)]
use aoc_2024::grid::{Grid, Point};
use std::{collections::HashSet, hash::Hash};

fn main() {
    let input = include_str!("../.inputs/input08.txt");
    let data = parse_input(input);
    let part1 = part1(&data);
    dbg!(part1);
    let part2 = part2(&data);
    dbg!(part2);
}

type Input = Grid<u8>;

fn parse_input(input: &str) -> Input {
    Grid::<u8>::parse_grid_bytes(input).unwrap()
}

fn part1(input: &Input) -> String {
    let wavelengths = input.find_unique_values(Some(vec![b'.']));
    let mut antinode_set: HashSet<(usize, usize)> = HashSet::new();
    wavelengths.iter().for_each(|wavelength| {
        let locs = input.find_all(wavelength).unwrap();
        locs.iter()
            .enumerate()
            .flat_map(|(i, &x)| locs[i..].iter().map(move |&y| (x, y)))
            .for_each(|(loc_1, loc_2)| {
                let (dx, dy) = (
                    loc_1.0 as isize - loc_2.0 as isize,
                    loc_1.1 as isize - loc_2.1 as isize,
                );
                if dx != 0 || dy != 0 {
                    let (antinode_1, antinode_2) = (
                        (loc_1.0 as isize + dx, loc_1.1 as isize + dy),
                        (loc_2.0 as isize - dx, loc_2.1 as isize - dy),
                    );

                    if input.is_valid_boint(antinode_1) {
                        let (x1, y1) = (antinode_1.0 as usize, antinode_1.1 as usize);
                        antinode_set.insert((x1, y1));
                    }

                    if input.is_valid_boint(antinode_2) {
                        let (x2, y2) = (antinode_2.0 as usize, antinode_2.1 as usize);
                        antinode_set.insert((x2, y2));
                    }
                }
            });
    });
    antinode_set.len().to_string()
}

fn part2(input: &Input) -> String {
    let wavelengths = input.find_unique_values(Some(vec![b'.']));
    let mut antinode_set: HashSet<(usize, usize)> = HashSet::new();
    wavelengths.iter().for_each(|wavelength| {
        let locs = input.find_all(wavelength).unwrap();
        locs.iter()
            .enumerate()
            .flat_map(|(i, &x)| locs[i..].iter().map(move |&y| (x, y)))
            .for_each(|(loc_1, loc_2)| {
                antinode_set.insert(loc_1);
                antinode_set.insert(loc_2);
                let (dx, dy) = (
                    loc_1.0 as isize - loc_2.0 as isize,
                    loc_1.1 as isize - loc_2.1 as isize,
                );
                if dx != 0 || dy != 0 {
                    let (mut antinode_1, mut antinode_2) = (
                        (loc_1.0 as isize + dx, loc_1.1 as isize + dy),
                        (loc_2.0 as isize - dx, loc_2.1 as isize - dy),
                    );

                    while input.is_valid_boint(antinode_1) {
                        let (x1, y1) = (antinode_1.0 as usize, antinode_1.1 as usize);
                        antinode_set.insert((x1, y1));
                        antinode_1 = (antinode_1.0 as isize + dx, antinode_1.1 as isize + dy);
                    }

                    while input.is_valid_boint(antinode_2) {
                        let (x2, y2) = (antinode_2.0 as usize, antinode_2.1 as usize);
                        antinode_set.insert((x2, y2));
                        antinode_2 = (antinode_2.0 as isize - dx, antinode_2.1 as isize - dy);
                    }
                }
            });
    });
    antinode_set.len().to_string()
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
        let ex_answer_2 = "34";
        let ex_data = parse_input(EX_INPUT);
        let result2 = part2(&ex_data);
        assert_eq!(result2, ex_answer_2);
    }
}

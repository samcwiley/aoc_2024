#![allow(unused_variables, dead_code, unused_imports)]
use aoc_2024::grid::*;

fn main() {
    let input = include_str!("../.inputs/input04.txt");
    let data = Grid::<u8>::parse_grid(input).unwrap();
    let part1 = part1(&data);
    dbg!(part1);
    let part2 = part2(&data);
    dbg!(part2);
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part1(input: &Grid<u8>) -> String {
    let mut result = 0;

    for i in 0..input.width {
        for j in 0..input.height {
            result += input
                .check_directions(i, j, 4)
                .iter()
                .filter(|m| **m == vec![b'X', b'M', b'A', b'S'])
                .count();
        }
    }

    result.to_string()
}

fn part2(input: &Grid<u8>) -> String {
    let mut result = 0;
    for i in 1..input.width - 1 {
        for j in 1..input.height - 1 {
            if input[(i, j)] == b'A' {
                result += (input[(i - 1, j - 1)].abs_diff(input[(i + 1, j + 1)])
                    == b'S'.abs_diff(b'M')
                    && input[(i - 1, j + 1)].abs_diff(input[(i + 1, j - 1)]) == b'S'.abs_diff(b'M'))
                    as u32;
            }
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ex_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let ex_answer = "18";
        let ex_data = Grid::<u8>::parse_grid(ex_input).unwrap();
        let result = part1(&ex_data);
        assert_eq!(result, ex_answer);
        let ex_answer2 = "9";
        let result2 = part2(&ex_data);
        assert_eq!(result2, ex_answer2);
    }
}

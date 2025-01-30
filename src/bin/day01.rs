#![allow(unused_variables, dead_code, unused_imports)]

fn main() {
    let input = include_str!("../.inputs/input01.txt");
    let data = parse_input(input);
    let part1 = part1(&data);
    dbg!(part1);
    let part2 = part2(&data);
    dbg!(part2);
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part1(input: &Vec<&str>) -> String {
    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();
    input.iter().for_each(|s| {
        let mut s = s.split_whitespace();
        list_1.push(s.next().unwrap().parse::<u32>().unwrap());
        list_2.push(s.next().unwrap().parse::<u32>().unwrap());
    });
    list_1.sort();
    list_2.sort();
    list_1
        .iter()
        .zip(list_2.iter())
        .map(|(loc_1, loc_2)| u32::abs_diff(*loc_1, *loc_2))
        .sum::<u32>()
        .to_string()
}

fn part2(input: &Vec<&str>) -> String {
    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();
    input.iter().for_each(|s| {
        let mut s = s.split_whitespace();
        list_1.push(s.next().unwrap().parse::<u32>().unwrap());
        list_2.push(s.next().unwrap().parse::<u32>().unwrap());
    });
    list_1
        .iter()
        .map(|loc_1| loc_1 * list_2.iter().filter(|&loc_2| *loc_2 == *loc_1).count() as u32)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ex_input = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";
        let ex_answer = "11";
        let ex_data = parse_input(ex_input);
        let result = part1(&ex_data);
        assert_eq!(result, ex_answer);
        let ex_answer2 = "31";
        let result2 = part2(&ex_data);
        assert_eq!(result2, ex_answer2);
    }
}

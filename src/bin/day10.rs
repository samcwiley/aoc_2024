#![allow(unused_variables, dead_code, unused_imports)]

fn main() {
    let input = include_str!("../.inputs/input10.txt");
    let data = parse_input(input);
    let part1 = part1(&data);
    dbg!(part1);
    let part2 = part2(&data);
    dbg!(part2);
}

type Input<'a> = Vec<&'a str>;

fn parse_input(raw: &str) -> Input {
    raw.lines().collect()
}

fn part1(input: &Input) -> String {
    "todo!()".to_string()
}

fn part2(input: &Input) -> String {
    "todo!()".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX_INPUT: &str = "";

    #[test]
    fn test_part_1() {
        let ex_answer = "";
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

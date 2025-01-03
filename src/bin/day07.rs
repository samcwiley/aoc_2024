#![allow(unused_variables, dead_code, unused_imports)]

fn main() {
    let input = include_str!("../.inputs/input07.txt");
    let data = parse_input(input);
    let part1 = part1(&data);
    dbg!(part1);
    //let part2 = part2(&data);
    //dbg!(part2);
}

#[derive(Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}
impl Equation {
    fn is_valid(&self) -> bool {
        Self::check_operators(self.result, &self.operands[..])
    }
    fn is_valid_concat(&self) -> bool {
        Self::check_w_concat(self.result, &self.operands[..])
    }
    fn check_operators(result: u64, operands: &[u64]) -> bool {
        if let Some((last, ops)) = operands.split_last() {
            if ops.len() == 0 {
                return result == *last;
            }
            if result % last == 0 && Self::check_operators(result / last, ops) {
                return true;
            }
            if result >= *last && Self::check_operators(result - last, ops) {
                return true;
            }
        }
        false
    }
    fn check_w_concat(result: u64, operands: &[u64]) -> bool {
        if let Some((last, ops)) = operands.split_last() {
            if ops.is_empty() {
                return result == *last;
            }

            if result >= *last && Self::check_w_concat(result - last, ops) {
                return true;
            }

            if result % last == 0 && Self::check_w_concat(result / last, ops) {
                return true;
            }

            //10u64.pow(last.ilog10() + 1) + last;
            //if Self::check_w_concat(result, &new_ops) {
            //    return true;
            //}
        }

        false
    }
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (res, ops) = line.split_once(":").unwrap();
            Equation {
                result: res.parse::<u64>().unwrap(),
                operands: ops
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect(),
            }
        })
        .collect()
}

fn part1(input: &Vec<Equation>) -> String {
    input
        .iter()
        .filter(|eq| eq.is_valid())
        .map(|eq| eq.result)
        .sum::<u64>()
        .to_string()
}

fn part2(input: &Vec<Equation>) -> String {
    input
        .iter()
        .filter(|eq| eq.is_valid_concat())
        .map(|eq| eq.result)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ex_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let ex_answer = "3749";
        let ex_data = parse_input(ex_input);
        let result = part1(&ex_data);
        assert_eq!(result, ex_answer);
        let ex_answer2 = "11387";
        let result2 = part2(&ex_data);
        assert_eq!(result2, ex_answer2);
    }
}

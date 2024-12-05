#![allow(unused_variables, dead_code, unused_imports)]

fn main() {
    let input = include_str!("../.inputs/input02.txt");
    let data = parse_input(input);
    let part1 = part1(&data);
    dbg!(part1);
    let part2 = part2(&data);
    dbg!(part2);
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    reports
}

// Checks if the vector is either all increasing or all decreasing, as well as making sure it is
// "gradual" (each change is within the safety threshold)
fn check_safe(report: &Vec<u8>) -> bool {
    let safety_threshold = 3;
    let increasing = report[1] > report[0];

    for i in 1..report.len() {
        let diff = u8::abs_diff(report[i], report[i - 1]);
        if diff > safety_threshold || diff == 0 {
            return false;
        }
        if (increasing && report[i] < report[i - 1]) || (!increasing && report[i] > report[i - 1]) {
            return false;
        }
    }
    true
}

// One irregularity is allowed. So if we find a pair of unsafe levels, we try removing either one,
// then check each of the edited reports for safety.
fn check_safe_dampened(report: &Vec<u8>) -> bool {
    let safety_threshold = 3;

    for i in 1..report.len() {
        let diff = u8::abs_diff(report[i], report[i - 1]);
        if diff > safety_threshold || diff == 0 {
            let mut edit_1 = report.clone();
            edit_1.remove(i);
            let mut edit_2 = report.clone();
            edit_2.remove(i - 1);
            return check_safe(&edit_1) || check_safe(&edit_2);
        }
        // very dumb way to instead check for increasing/decreasing allowing mismatches
        let mut increasing_count = 0;
        let mut decreasing_count = 0;

        for j in 1..report.len() {
            if report[j] > report[j - 1] {
                increasing_count += 1;
            } else if report[j] < report[j - 1] {
                decreasing_count += 1;
            }
        }
        let increasing = increasing_count >= decreasing_count;

        if (increasing && report[i] < report[i - 1]) || (!increasing && report[i] > report[i - 1]) {
            let mut edit_1 = report.clone();
            edit_1.remove(i);
            let mut edit_2 = report.clone();
            edit_2.remove(i - 1);
            return check_safe(&edit_1) || check_safe(&edit_2);
        }
    }
    true
}

fn part1(input: &Vec<Vec<u8>>) -> String {
    input
        .iter()
        .filter(|report| check_safe(report))
        .count()
        .to_string()
}

fn part2(input: &Vec<Vec<u8>>) -> String {
    input
        .iter()
        .filter(|report| check_safe_dampened(report))
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ex_input = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";
        let ex_answer = "2";
        let ex_data = parse_input(ex_input);
        let result = part1(&ex_data);
        assert_eq!(result, ex_answer);
        let ex_answer2 = "4";
        let result2 = part2(&ex_data);
        assert_eq!(result2, ex_answer2);
    }
    #[test]
    fn test_safety() {
        let safe_vec = vec![50, 51, 52, 54];
        assert!(check_safe(&safe_vec) == true);
        let damp_safe = vec![51, 50, 51, 52, 54];
        assert!(check_safe_dampened(&damp_safe) == true);
    }
}

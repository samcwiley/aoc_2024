#![allow(unused_variables, dead_code, unused_imports)]

fn main() {
    let input = include_str!("../.inputs/input03.txt");
    let data = parse_input(input);
    let part1 = part1(&data);
    dbg!(part1);
    let part2 = part2(input);
    dbg!(part2);
}

// a regular expression would be nice here,,,
fn parse_input(input: &str) -> Vec<(i32, i32)> {
    let mut results = Vec::new();
    let mut start = 0;

    while let Some(pos) = input[start..].find("mul(") {
        let actual_pos = start + pos;
        start = actual_pos + 4;

        if let Some(end) = input[start..].find(')') {
            let end_pos = start + end;
            let content = &input[start..end_pos];

            if let Some((x, y)) = parse_coords(content) {
                results.push((x, y));
                start = end_pos + 1;
            }
        } else {
            break;
        }
    }

    results
}

fn parse_coords(content: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = content.split(',').collect();
    if parts.len() == 2 {
        if let (Ok(x), Ok(y)) = (parts[0].parse(), parts[1].parse()) {
            return Some((x, y));
        }
    }
    None
}

fn part1(input: &Vec<(i32, i32)>) -> String {
    input.iter().map(|(a, b)| a * b).sum::<i32>().to_string()
}

// the plan will be to run through and remove the "dead" code in between the don't() and the next do(),
// then repeat part 1
fn part2(input: &str) -> String {
    let mut current_code = input;
    let mut executable = String::new();
    let mut enabled = true;

    loop {
        if enabled {
            let Some(stop_index) = current_code.find("don't()") else {
                executable.push_str(current_code);
                break;
            };
            executable.push_str(&current_code[..stop_index]);
            current_code = &current_code[stop_index..];
            enabled = false;
        } else {
            let Some(start_index) = current_code.find("do()") else {
                break;
            };
            current_code = &current_code[start_index..];
            enabled = true;
        }
    }
    parse_input(&executable)
        .iter()
        .map(|(a, b)| a * b)
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ex_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let ex_answer = "161";
        let ex_data = parse_input(ex_input);
        let result = part1(&ex_data);
        assert_eq!(result, ex_answer);
        let ex_answer2 = "48";
        let result2 = part2(ex_input);
        assert_eq!(result2, ex_answer2);
    }
}

#![allow(unused_variables, dead_code, unused_imports)]

fn main() {
    let input = include_str!("../.inputs/input03.txt");
    let data = parse_input(input);
    let part1 = part1(&data);
    dbg!(part1);
    //let part2 = part2(&data);
    //dbg!(part2);
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

/*fn part2(input: &Vec<&str>) -> String {
    "todo!()".to_string()
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ex_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let ex_answer = "161";
        let ex_data = parse_input(ex_input);
        let result = part1(&ex_data);
        assert_eq!(result, ex_answer);
        /*let ex_answer2 = "";
        let result2 = part2(&ex_data);
        assert_eq!(result2, ex_answer2);
        */
    }
}

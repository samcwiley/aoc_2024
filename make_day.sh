#!/bin/bash

set -e

YEAR=2024

# Validate day argument
if [[ -z "$1" || "$1" =~ [^0-9] ]]; then
    echo "Error: Please provide a day as a number between 1 and 25 (inclusive)."
    exit 1
fi

if [ "$1" -lt 1 ] || [ "$1" -gt 25 ]; then
    echo "Error: Day argument must be between 1 and 25 (inclusive)."
    exit 1
fi

day_number="${1#0}"

if [ "${#1}" -eq 1 ]; then
    file_number="0$1"
else
    file_number="$1"
fi

# Check for session file
if ! [ -f ".session" ]; then
    echo "Error: You need to get the session cookie from the AoC website and put it in a file named .session"
    exit 1
fi

# Download input file if not already downloaded
if [ -e "src/.inputs/input$file_number.txt" ]; then
    echo "Day $day_number input file already exists! Skipping download."
else
    mkdir -p src/.inputs
    curl "https://adventofcode.com/$YEAR/day/$day_number/input" \
        --silent --fail --max-time 10 --cookie "session=$(cat .session)" \
        -o "src/.inputs/input$file_number.txt" || {
        echo "Error: Failed to download input file. Check your session or network."
        rm -f "src/.inputs/input$file_number.txt"
        exit 1
    }
fi

# Create Rust solution file if not already created
if [ -e "src/bin/day$file_number.rs" ]; then
    echo "Day $day_number solution file already exists; good luck!"
else
    mkdir -p src/bin
    cat <<EOF >src/bin/day"$file_number".rs
#![allow(unused_variables, dead_code, unused_imports)]

fn main() {
    let input = include_str!("../.inputs/input$file_number.txt");
    let data = parse_input(input);
    //let part1 = part1(data);
    //dbg!(part1);
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part1(input: Vec<&str>) -> String {
    "todo!()".to_string()
}

/*fn part2(input: Vec<&str>) -> String {
    "todo!()".to_string()
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ex_input = "";
        let ex_answer = "";
        let ex_data = parse_input(ex_input);
        let result = part1(ex_data);
        assert_eq!(result, ex_answer);
        /*let ex_answer2 = "";
        let result2 = part2(ex_data);
        assert_eq!(result2, ex_answer2);
        */
    }
}
EOF
    echo "Created solution template at src/bin/day$file_number.rs"
fi

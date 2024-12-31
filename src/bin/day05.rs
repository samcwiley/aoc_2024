#![allow(unused_variables, dead_code, unused_imports)]
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../.inputs/input05.txt");
    let data = parse_input(input);
    let (rules, books) = parse_input(input);
    let (part_1, part_2) = solution(rules, books);
    dbg!(part_1);
    dbg!(part_2);
}

#[derive(Debug)]
struct Page {
    number: u8,
    predecessors: HashSet<u8>,
}

fn parse_input(input: &str) -> (HashMap<u8, Page>, Vec<Vec<u8>>) {
    let mut book_rules: HashMap<u8, Page> = HashMap::new();
    let (in_1, in_2) = input.split_once("\n\n").unwrap();
    in_1.lines().for_each(|line| {
        let (num_1, num_2) = line.split_once("|").unwrap();
        let num_1 = num_1.parse::<u8>().unwrap();
        let num_2 = num_2.parse::<u8>().unwrap();
        if let Some(page) = book_rules.get_mut(&num_1) {
            page.predecessors.insert(num_2);
        } else {
            book_rules.insert(
                num_1,
                Page {
                    number: num_1,
                    predecessors: HashSet::from([num_2]),
                },
            );
        }
    });
    let in_2 = in_2
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    (book_rules, in_2)
}

fn solution(rules: HashMap<u8, Page>, books: Vec<Vec<u8>>) -> (String, String) {
    let (mut part_1, mut part_2) = (0, 0);

    for book in books.iter() {
        let mut is_sorted = true;
        for (i, &page) in book.iter().enumerate() {
            if let Some(page_rules) = rules.get(&page) {
                for &predecessor in &page_rules.predecessors {
                    if let Some(predecessor_index) = book.iter().position(|&p| p == predecessor) {
                        if predecessor_index < i {
                            is_sorted = false;
                            break;
                        }
                    }
                }
            }
            if !is_sorted {
                break;
            }
        }

        if is_sorted {
            let middle_index = book.len() / 2;
            part_1 += book[middle_index] as u32;
        } else {
            let mut sorted_book = book.clone();
            sorted_book.sort_by(|&a, &b| {
                if let Some(page_a) = rules.get(&a) {
                    if page_a.predecessors.contains(&b) {
                        return std::cmp::Ordering::Greater;
                    }
                }
                if let Some(page_b) = rules.get(&b) {
                    if page_b.predecessors.contains(&a) {
                        return std::cmp::Ordering::Less;
                    }
                }
                std::cmp::Ordering::Equal
            });

            let middle_index = sorted_book.len() / 2;
            part_2 += sorted_book[middle_index] as u32;
        }
    }

    (part_1.to_string(), part_2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ex_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let ex_answer = "143";
        let (rules, books) = parse_input(ex_input);
        let (part_1, part_2) = solution(rules, books);
        assert_eq!(part_1, ex_answer);
        let ex_answer2 = "123";
        assert_eq!(part_2, ex_answer2);
    }
}

#![allow(unused_variables, dead_code, unused_imports)]
use std::fmt;

fn main() {
    let input = include_str!("../.inputs/input09.txt");
    let data = parse_input(input);
    let part1 = part1(&data);
    dbg!(part1);
    let part2 = part2(&data);
    dbg!(part2);
}

type Input = Vec<u8>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Block {
    Empty,
    File(u16),
}
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Block::Empty => write!(f, "."),
            Block::File(id) => write!(f, "{}", id),
        }
    }
}

struct Disk {
    data: Vec<Block>,
}

impl Disk {
    fn compress(&mut self) {
        let (mut i, mut j) = (0usize, self.data.len() - 1);
        while i < j {
            // find next empty spot
            while self.data[i] != Block::Empty {
                i += 1;
            }
            // find next value that needs to move
            while self.data[j] == Block::Empty {
                j -= 1;
            }
            if i < j {
                self.data[i] = self.data[j];
                self.data[j] = Block::Empty;
            }
        }
    }

    fn find_empty_spans(&self) -> Vec<(usize, usize)> {
        let mut spans = Vec::new();
        let mut i = 0;

        while i < self.data.len() {
            if self.data[i] == Block::Empty {
                let start = i;
                while i < self.data.len() && self.data[i] == Block::Empty {
                    i += 1;
                }
                spans.push((start, i));
            } else {
                i += 1;
            }
        }

        spans
    }
    fn find_file_chunks(&self) -> Vec<(usize, usize, u16)> {
        let mut chunks = Vec::new();
        let mut i = 0;

        while i < self.data.len() {
            if let Block::File(id) = self.data[i] {
                let start = i;
                while i < self.data.len() && self.data[i] == Block::File(id) {
                    i += 1;
                }
                chunks.push((start, i, id));
            } else {
                i += 1;
            }
        }

        chunks
    }

    /// an assuredly slow way to do this
    fn compress_2(&mut self) {
        let mut empty_spans = self.find_empty_spans();
        let mut file_chunks = self.find_file_chunks();

        file_chunks.sort_by(|a, b| b.2.cmp(&a.2));

        for (start, end, id) in file_chunks {
            let chunk_size = end - start;

            if let Some((span_start, span_end)) =
                empty_spans.iter().find(|&&(span_start, span_end)| {
                    (span_end - span_start) >= chunk_size && span_start < start
                })
            {
                let span_start = *span_start;
                for k in 0..chunk_size {
                    self.data[span_start + k] = Block::File(id);
                }

                for k in start..end {
                    self.data[k] = Block::Empty;
                }
                empty_spans = self.find_empty_spans();
            }
        }
    }

    fn checksum(&self) -> u64 {
        let mut result = 0;
        for (i, block) in self.data.iter().enumerate() {
            if let Block::File(value) = block {
                result += i as u64 * *value as u64;
            }
        }
        result
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for block in &self.data {
            write!(f, "{}", block)?;
        }
        Ok(())
    }
}

fn parse_input(raw: &str) -> Input {
    raw.trim()
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect()
}

fn expand_input(input: &Input) -> Disk {
    let first = input[0];
    let mut id = 1u16;
    let mut expanded = Vec::<Block>::new();
    for _ in 0..first {
        expanded.push(Block::File(0));
    }
    input[1..].chunks(2).for_each(|chunk| {
        let (space_length, file_length) = (chunk[0], chunk[1]);
        for _ in 0..space_length {
            expanded.push(Block::Empty);
        }
        for _ in 0..file_length {
            expanded.push(Block::File(id));
        }
        id += 1;
    });
    Disk { data: expanded }
}

fn part1(input: &Input) -> String {
    let mut disk = expand_input(input);
    disk.compress();
    disk.checksum().to_string()
}

fn part2(input: &Input) -> String {
    let mut disk = expand_input(input);
    disk.compress_2();
    disk.checksum().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_1() {
        let ex_answer = "1928";
        let ex_data = parse_input(EX_INPUT);
        let result = part1(&ex_data);
        assert_eq!(result, ex_answer);
    }

    #[test]
    fn test_part_2() {
        let ex_answer_2 = "2858";
        let ex_data = parse_input(EX_INPUT);
        let result2 = part2(&ex_data);
        assert_eq!(result2, ex_answer_2);
    }
}

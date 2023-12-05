#![allow(unused_variables, dead_code, unused_imports)]

use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Map;
use std::path::Path;

#[derive(Debug)]
struct Num {
    value: u32,
    length: usize,
    start_x: usize,
    end_x: usize,
}
impl Num {
    fn look_around(&self, above: &[char], below: &[char], line: &[char]) -> u32 {
        if above.iter().any(|&c| !c.is_digit(10) && c != '.')
            || below.iter().any(|&c| !c.is_digit(10) && c != '.')
            || line.iter().any(|&c| !c.is_digit(10) && c != '.')
        {
            return self.value;
        }
        0
    }
}

fn main() {
    let pattern = Regex::new(r"\d+").unwrap();
    let mut input: Vec<(Vec<char>, Vec<Num>)> = read_lines("../inputs/input03.txt")
        .unwrap()
        .map(|line| {
            let text: String = format!(".{}..", line.unwrap());
            let row: Vec<char> = text.chars().collect();
            let mut row_nums: Vec<Num> = Vec::new();
            for mat in pattern.find_iter(&text) {
                let start_byte = mat.start();
                let end_byte = mat.end();
                let start_char: usize = text[..start_byte].chars().count();
                let end_char: usize = text[..end_byte].chars().count();
                let value: u32 = text[start_byte..end_byte].parse().unwrap();
                let number = Num {
                    value: value,
                    length: end_char - start_char,
                    start_x: start_char,
                    end_x: end_char,
                };
                row_nums.push(number);
            }
            (row, row_nums)
        })
        .collect();
    input.insert(0, (vec!['.'; input[0].0.len()], Vec::new()));
    input.push((vec!['.'; input[0].0.len()], Vec::new()));

    let part1: u32 = input
        .windows(3)
        .map(|window| {
            let current_row = window[1].0.clone();
            let row_nums = &window[1].1;
            let mut row_total = 0u32;
            for num in row_nums {
                let (left_ind, right_ind) = (num.start_x - 1, num.end_x + 1);
                let above = &window[0].0[left_ind..right_ind];
                let line = &window[1].0[left_ind..right_ind];
                let below = &window[2].0[left_ind..right_ind];
                row_total += num.look_around(&above, &below, &line);
            }
            //println!("{}", row_total);
            row_total
        })
        .sum();

    let part2: u32 = input
        .windows(3)
        .map(|window| {
            let current_row = window[1].0.clone();
            let asterisks: Vec<usize> = current_row
                .iter()
                .enumerate()
                .filter(|&(_, &c)| c == '*')
                .map(|(index, _)| index)
                .collect();
            let mut row_total: u32 = 0;
            for asterisk in asterisks {
                let mut bordering: u32 = 0;
                let mut values: Vec<u32> = Vec::new();
                for row in window {
                    for num in &row.1 {
                        if asterisk >= num.start_x - 1 && asterisk <= num.end_x {
                            bordering += 1;
                            values.push(num.value);
                        }
                    }
                }
                if bordering == 2 {
                    let ratio: u32 = values.iter().product();
                    row_total += ratio;
                }
            }
            row_total
        })
        .sum();

    println!("{part1}\n{part2}");
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

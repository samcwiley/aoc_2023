#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Map;
use std::path::Path;

fn main() {
    let (part1, part2): (u32, u32) = read_lines("../inputs/input1.txt")
        .unwrap()
        .map(|line| {
            let line = line.unwrap();
            get_digits(&line)
        })
        .fold((0, 0), |(sum1, sum2), (n1, n2)| (sum1 + n1, sum2 + n2));
    println!("{part1}\n{part2}");
}

fn get_digits(line: &str) -> (u32, u32) {
    let line2 = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
        .to_string();
    let num: u32 = find_first_digit(&line).unwrap() * 10 + find_last_digit(&line).unwrap();
    let num2: u32 = find_first_digit(&line2).unwrap() * 10 + find_last_digit(&line2).unwrap();
    (num, num2)
}

fn find_first_digit(string: &str) -> Option<u32> {
    for c in string.chars() {
        if let Some(digit) = c.to_digit(10) {
            return Some(digit);
        }
    }
    None
}

fn find_last_digit(string: &str) -> Option<u32> {
    for c in string.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            return Some(digit);
        }
    }
    None
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#![allow(unused_variables, dead_code, unused_imports)]

use std::iter::zip;

fn main() {
    let input = include_str!("../.inputs/input06.txt");
    let data = parse_input(input);
    let part1 = part1(data);
    let data2 = parse_input2(input);
    let part2 = part2(data2);
    println!("{}\n{}", part1, part2);
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let times: Vec<u32> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<u32> = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    (times, distances)
}

fn parse_input2(input: &str) -> (u64, u64) {
    let time: u64 = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();
    let distance: u64 = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();
    (time, distance)
}

fn part1((times, distances): (Vec<u32>, Vec<u32>)) -> u32 {
    let result: u32 = zip(times, distances)
        .map(|(time, distance)| {
            let mut possible_times: Vec<u32> = Vec::new();
            for i in 0..=time {
                if (time - i) * i > distance {
                    possible_times.push(i);
                }
            }
            possible_times.len() as u32
        })
        .product();
    result
}

fn part2((time, distance): (u64, u64)) -> u64 {
    let mut result: u64 = 0;
    for i in 0..=time {
        if (time - i) * i > distance {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ex_input = "Time:      7  15   30
            Distance:  9  40  200";
        let ex_answer = 288;
        let ex_data = parse_input(ex_input);
        let result = part1(ex_data);
        assert_eq!(result, ex_answer);
        let ex_answer2 = 71503;
        let ex_data2 = parse_input2(ex_input);
        let result2 = part2(ex_data2);
        assert_eq!(result2, ex_answer2);
    }
}

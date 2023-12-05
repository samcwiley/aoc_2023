#![allow(unused_variables, dead_code, unused_imports)]

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Map;
use std::path::Path;

fn get_score(matches: u32) -> u32 {
    match matches {
        0 => 0u32,
        _ => u32::pow(2, matches - 1),
    }
}

fn main() {
    let matches: Vec<u32> = read_lines("../inputs/input04.txt")
        .unwrap()
        .map(|line| {
            let line = line.unwrap();
            let nums: Vec<&str> = line.split(":").collect::<Vec<_>>()[1].split("|").collect();
            let winners: Vec<u8> = nums[0]
                .split_whitespace()
                .map(|num| num.parse::<u8>().unwrap())
                .collect();
            let my_nums: Vec<u8> = nums[1]
                .split_whitespace()
                .map(|num| num.parse::<u8>().unwrap())
                .collect();
            let matches: u32 = winners
                .iter()
                .filter(|num| my_nums.contains(num))
                .collect::<Vec<_>>()
                .len() as u32;
            //let score = get_score(matches);
            //score
            matches
        })
        .collect();
    let part1: u32 = matches.iter().map(|matches| get_score(*matches)).sum();
    let part2 = part2(matches);
    println!("{}\n{}", part1, part2);
    //let test: Vec<u32> = vec![4, 2, 2, 1, 0, 0];
    //let test_answer = part2(test);
    //println!("{}", test_answer)
}

fn part2(matches: Vec<u32>) -> u32 {
    let mut cards: Vec<u32> = vec![1; matches.len()];
    cards.insert(0, 0);
    //let mut cards: HashMap<u8, u32> = HashMap::new();

    for (index, num_matches) in matches.iter().enumerate() {
        let card_num = index + 1;
        for i in (card_num + 1)..=(card_num + *num_matches as usize) {
            cards[i] += cards[card_num];
        }
    }
    cards.iter().sum()
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

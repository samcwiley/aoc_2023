#![allow(unused_variables, dead_code, unused_imports)]

use regex::Regex;
use std::cmp::max;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Map;
use std::path::Path;

struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}
impl Game {
    fn is_legal(&self) -> bool {
        self.max_red <= 12 && self.max_green <= 13 && self.max_blue <= 14
    }
}

fn main() {
    let num_regex = Regex::new(r"Game (\d+)").unwrap(); //setting up regexes to find game id and colors
    let colors_regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let input: Vec<Game> = read_lines("../inputs/input2.txt")
        .unwrap()
        .map(|line| {
            let line = line.unwrap();
            let num: u32 = num_regex //create a Game struct for each line
                .captures(&line)
                .and_then(|captures| captures.get(1))
                .map(|m| m.as_str().parse().unwrap())
                .unwrap();
            let (mut red, mut green, mut blue): (u32, u32, u32) = (0, 0, 0);
            // loops through captured numbers and colors to find the maximum number for each color
            for captures in colors_regex.captures_iter(&line) {
                let count: u32 = captures[1].parse().unwrap();
                match &captures[2] {
                    "red" => red = max(red, count),
                    "green" => green = max(green, count),
                    "blue" => blue = max(blue, count),
                    _ => {}
                }
            }
            let game = Game {
                id: num,
                max_red: red,
                max_green: green,
                max_blue: blue,
            };
            game
        })
        .collect();
    // adds up the legal game ids!
    let answer: u32 = input
        .iter()
        .map(|game| match game.is_legal() {
            true => game.id,
            false => 0,
        })
        .sum();
    // adds up legal powers!
    let answer2: u32 = input
        .iter()
        .map(|game| game.max_red * game.max_green * game.max_blue)
        .sum();
    println!("{answer}\n{answer2}");
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

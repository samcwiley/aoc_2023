#![allow(unused_variables, dead_code, unused_imports)]

use std::cmp::min;

fn main() {
    let input = include_str!("../.inputs/input05.txt");
    let data = parse_input(input);
    //let part1 = part1(data);
    let part2: i64 = part2_brute_force(data);
    println!("{}", part2);
    //println!("{}\n{}", part1, part2);
}
/*
fn part1((seeds, maps): (Vec<i64>, Vec<Vec<((i64, i64), i64)>>)) -> i64 {
    let result = seeds
        .iter()
        .map(|seed| {
            let mut seed: i64 = seed.to_owned();
            for map_group in &maps {
                for map in map_group {
                    if map.0 .0 <= seed && seed <= map.0 .1 {
                        seed += map.1;
                        break;
                    }
                }
            }
            seed
        })
        .min()
        .unwrap();
    result
}
*/

fn part2_brute_force((seeds, maps): (Vec<i64>, Vec<Vec<((i64, i64), i64)>>)) -> i64 {
    let mut lowest: i64 = i64::MAX;
    for pair in seeds.chunks(2) {
        for mut seed in pair[0]..=pair[0] + pair[1] {
            for map_group in &maps {
                for map in map_group {
                    if map.0 .0 <= seed && seed <= map.0 .1 {
                        seed += map.1;
                        break;
                    }
                }
            }
            lowest = min(lowest, seed);
        }
    }
    lowest
}
/*
fn part2_brute_force_better((seeds, maps): (Vec<i64>, Vec<Vec<((i64, i64), i64)>>)) -> i64 {
    let mut lowest: i64 = i64::MAX;
    for pair in seeds.chunks(2) {
        let mut seeds: Vec<i64> = Vec::new();
        for i in pair[0]..=pair[0] + pair[1] {
            seeds.push(i);
        }
        let result = seeds
            .iter()
            .map(|seed| {
                let mut seed: i64 = seed.to_owned();
                for map_group in &maps {
                    for map in map_group {
                        if map.0 .0 <= seed && seed <= map.0 .1 {
                            seed += map.1;
                            break;
                        }
                    }
                }
                seed
            })
            .min()
            .unwrap();
        lowest = min(lowest, result);
    }
    lowest
}

fn part2_brute_force_take_1((seeds, maps): (Vec<i64>, Vec<Vec<((i64, i64), i64)>>)) -> i64 {
    let mut all_seeds: Vec<i64> = Vec::new();
    for pair in seeds.chunks(2) {
        for i in pair[0]..=pair[0] + pair[1] {
            all_seeds.push(i);
        }
    }
    println!("{}", all_seeds.len());
    let result = all_seeds
        .iter()
        .map(|seed| {
            let mut seed: i64 = seed.to_owned();
            for map_group in &maps {
                for map in map_group {
                    if map.0 .0 <= seed && seed <= map.0 .1 {
                        seed += map.1;
                        break;
                    }
                }
            }
            seed
        })
        .min()
        .unwrap();
    result
} */

fn parse_input(input: &str) -> (Vec<i64>, Vec<Vec<((i64, i64), i64)>>) {
    let mut data = input.split("\n\n");
    let seeds: Vec<i64> = data
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse().unwrap())
        .collect();
    let maps = data.map(|map| parse_map(map)).collect();
    (seeds, maps)
}

fn parse_map(input: &str) -> Vec<((i64, i64), i64)> {
    let result: Vec<((i64, i64), i64)> = input
        .lines()
        .skip(1)
        .map(|line| {
            let line: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            ((line[1], line[1] + line[2]), line[0] - line[1])
        })
        .collect();
    result
}

/*fn part2(input: Vec<&str>) -> String {
    "todo!()".to_string()
}
*/
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ex_input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4";
        let ex_answer = 35;
        let ex_data = parse_input(ex_input);
        //let result = part1(ex_data);
        //assert_eq!(result, ex_answer);
        let ex_answer2 = 46;
        let result2 = part2_brute_force(ex_data);
        assert_eq!(result2, ex_answer2);
    }
}
*/

#![allow(unused_variables, dead_code, unused_imports)]

use num::integer::lcm;
use std::{collections::HashMap, thread::current};

fn main() {
    let input = include_str!("../.inputs/input08.txt");
    let input1 = "RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";
    let input2 = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    let data = parse_input(input);
    //let part1 = part1(data);
    let part2: u64 = part2(data);
    println!("{}", part2);
    //let part1 = part1(data);
    //dbg!(part1);
}

fn parse_input(input: &str) -> (Vec<u8>, HashMap<String, (String, String)>) {
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let (lr, locs): (&str, &str) = input.split_once("\n\n").unwrap();
    let lr: Vec<u8> = lr.as_bytes().iter().cloned().collect();
    for line in locs.lines() {
        let (loc, left_right) = line.split_once(" = ").unwrap();
        let (left, right) = left_right.split_once(", ").unwrap();
        let left = left.replace("(", "");
        let right = right.replace(")", "");
        //println!("{} {} {}", loc, left, right);
        map.insert(loc.to_string(), (left, right));
    }
    (lr, map)
}

fn part1((lr, map): (Vec<u8>, HashMap<String, (String, String)>)) -> u64 {
    get_steps("AAA", (&lr, &map))
}

fn part2((lr, map): (Vec<u8>, HashMap<String, (String, String)>)) -> u64 {
    //println!("{:?}", map);
    //println!("{:?}", map.get("11A").unwrap());
    let steps: u64 = map
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|key| get_steps(key, (&lr, &map)))
        .fold(1, lcm);
    steps
}

fn get_steps(start_loc: &str, (lr, map): (&Vec<u8>, &HashMap<String, (String, String)>)) -> u64 {
    let mut current_loc = start_loc;
    let mut move_count: u64 = 0;
    let mut instructions: Vec<u8> = lr.iter().cloned().rev().collect();
    //while current_loc != "ZZZ".to_string() {
    while !current_loc.ends_with("Z") {
        //println!("{current_loc}");
        if let Some(next_move) = instructions.pop() {
            //println!("{next_move} {}", b'L');
            if let Some((left, right)) = map.get(current_loc) {
                if next_move == b'L' {
                    current_loc = left;
                } else {
                    current_loc = right;
                }
            }
            move_count += 1;
        } else {
            instructions = lr.iter().cloned().rev().collect(); //renew instructions if we run out
        }
    }
    move_count
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
        let ex_input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let ex_answer = 2;
        let ex_data = parse_input(ex_input);
        let result = part1(ex_data);
        assert_eq!(result, ex_answer);
        /*let ex_answer2 = "";
        let result2 = part2(ex_data);
        assert_eq!(result2, ex_answer2);
        */
    }
}

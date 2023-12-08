#![allow(unused_variables, dead_code, unused_imports)]

use std::collections::HashMap;
//use std::marker::Tuple;

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: String,
    bid: u32,
    hand_type: u8,
}

const CARD_ORDER: &str = "AKQJT98765432";
const PART2_CARD_ORDER: &str = "AKQT98765432J";

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let part2: bool = true;
        // First, compare by hand_type
        let type_cmp = self.hand_type.cmp(&other.hand_type);

        if type_cmp == std::cmp::Ordering::Equal {
            // If hand_type is the same, compare cards based on custom order
            let cards_cmp = self
                .cards
                .chars()
                .zip(other.cards.chars())
                .find(|&(c1, c2)| {
                    if !part2 {
                        let index1 = CARD_ORDER.find(c1);
                        let index2 = CARD_ORDER.find(c2);
                        index1 != index2
                    } else {
                        let index1 = PART2_CARD_ORDER.find(c1);
                        let index2 = PART2_CARD_ORDER.find(c2);
                        index1 != index2
                    }
                });

            match cards_cmp {
                Some((c1, c2)) => {
                    if !part2 {
                        let index1 = CARD_ORDER.find(c1).unwrap();
                        let index2 = CARD_ORDER.find(c2).unwrap();
                        index2.cmp(&index1)
                    } else {
                        let index1 = PART2_CARD_ORDER.find(c1).unwrap();
                        let index2 = PART2_CARD_ORDER.find(c2).unwrap();
                        index2.cmp(&index1)
                    }
                    //index1.cmp(&index2)
                }
                None => std::cmp::Ordering::Equal, // Cards are the same
            }
            .then_with(|| self.bid.cmp(&other.bid))
        } else {
            // If hand_type is different, return the comparison result
            type_cmp
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_hand_type(hand: &str) -> u8 {
    print!("{hand} ");
    let part2 = true;
    let mut card_counts: HashMap<char, u8> = HashMap::new();
    if hand == "JJJJJ" {
        return 7;
    }
    let jokers = hand.chars().filter(|&c| c == 'J').count() as u8;
    let hand: String = if part2 {
        hand.chars().filter(|&c| c != 'J').collect()
    } else {
        hand.to_string()
    };

    for c in hand.chars() {
        let count = card_counts.entry(c).or_insert(0);
        *count += 1;
    }
    let mut max_counts: Vec<u8> = card_counts.values().cloned().collect::<Vec<_>>();
    max_counts.sort_by(|a, b| b.cmp(a));
    if part2 {
        max_counts[0] += jokers;
    }
    let hand_type: u8 = match max_counts[0] {
        //max_counts is just a vector of character counts, sorted by highest count
        5 => 7, //5 of a kind
        4 => 6, //4 of a kind
        3 => {
            if max_counts[1] == 2 {
                5 //full house
            } else {
                4 //3 of a kind
            }
        }
        2 => {
            if max_counts[1] == 2 {
                3 // 2 pair
            } else {
                2 // 1 pair
            }
        }
        1 => 1, //high card
        _ => 0,
    };
    println!("{hand_type}");
    hand_type
}

fn main() {
    let input = include_str!("../.inputs/input07.txt");
    let input2 = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";
    let data = parse_input(input);
    //let data = data.iter().take(20).to_owned().collect();
    let part1 = part1(data);
    let part2 = part1;
    dbg!(part2);
}

fn parse_input(input: &str) -> Vec<Hand> {
    input
        .lines()
        //.filter(|hand| hand.contains('J'))
        .map(|line| {
            let mut line = line.split_whitespace();
            let cards: &str = line.next().unwrap();
            let bid: u32 = line.next().unwrap().parse().unwrap();
            let hand = Hand {
                cards: cards.to_string(),
                bid: bid,
                hand_type: get_hand_type(&cards),
            };
            hand
        })
        .collect()
}

fn part1(mut input: Vec<Hand>) -> u32 {
    input.sort();
    //for hand in &input {
    //    println!("{:?}", hand);
    //}
    let result = input
        .iter()
        .enumerate()
        .map(|(rank, hand)| {
            let rank: u32 = 1 + rank as u32;
            rank * hand.bid
        })
        .sum();
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
        let ex_input = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";
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
*/

#![allow(unused_variables, dead_code, unused_imports)]

fn main() {
    let input = include_str!("../.inputs/input09.txt");
    let data = parse_input(input);
    let part2 = part2(data);
    dbg!(part2);
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let input: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    input
}

fn find_next(vector: Vec<i32>) -> i32 {
    let mut next: i32 = 0;
    let mut vector = vector;
    //vector.reverse();

    while vector != vec![0i32; vector.len()] {
        next += vector.iter().last().unwrap();
        vector = vector.windows(2).map(|nums| nums[1] - nums[0]).collect();
    }
    next
}

fn find_next2(vector: Vec<i32>) -> i32 {
    let mut next: i32 = 0;
    let mut vector = vector;
    vector.reverse();

    while vector != vec![0i32; vector.len()] {
        next += vector.iter().last().unwrap();
        vector = vector.windows(2).map(|nums| nums[1] - nums[0]).collect();
    }
    next
}

fn part1(input: Vec<Vec<i32>>) -> i32 {
    let answer = input.iter().map(|line| find_next(line.to_owned())).sum();
    answer
}

fn part2(input: Vec<Vec<i32>>) -> i32 {
    let answer = input.iter().map(|line| find_next2(line.to_owned())).sum();
    answer
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
        let ex_input = "
        0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        //let ex_answer = 114;
        let ex_data = parse_input(ex_input);
        //let result = part1(ex_data);
        //assert_eq!(result, ex_answer);
        let ex_answer2 = 2;
        let result2 = part2(ex_data);
        assert_eq!(result2, ex_answer2);
    }
}

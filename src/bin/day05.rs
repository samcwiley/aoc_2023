#![allow(unused_variables, dead_code, unused_imports)]

fn main() {
    let input = include_str!("../.inputs/input1.txt");
    let data = parse_input(input);
    //let part1 = part1(data);
    //dbg!(part);
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part1(input: Vec<&str>) -> String {
    "todo!()".to_string()
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
        let ex_input = "";
        let ex_answer = "";
        let ex_data = parse_input(ex_input);
        let result = part1(ex_data);
        assert_eq!(result, ex_answer);
        /*let ex_answer2 = "";
        let result2 = part2(ex_data);
        assert_eq!(part2(ex_data), ex_answer);
        */
    }
}

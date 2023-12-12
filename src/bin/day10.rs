#![allow(unused_variables, dead_code, unused_imports)]

fn main() {
    let input = include_str!("../.inputs/input10.txt");
    let data = parse_input(input);
    //let part1 = part1(data);
    //dbg!(part1);
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let matrix = input
        .lines()
        .map(|line| line.as_bytes().iter().map(|x| x.clone()).collect())
        .collect();
    matrix
}

fn find_animal(matrix: &Vec<Vec<u8>>, target: u8) -> Option<(usize, usize)> {
    for (i, row) in matrix.iter().enumerate() {
        for (j, &element) in row.iter().enumerate() {
            if element == target {
                return Some((i, j));
            }
        }
    }
    None
}
/*
| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this
*/

fn find_step_2(start: (usize, usize), matrix: &Vec<Vec<u8>>) -> (usize, usize) {
    let matrix = matrix.clone();

    let left = [b'F', b'L', b'-'];
    let right = [b'7', b'J', b'-'];
    let up = [b'F', b'7', b'|'];
    let down = [b'J', b'L', b'|'];

    if up.contains(&matrix[start.0 - 1][start.1]) {
        return (start.0 - 1, start.1);
    }
    if down.contains(&matrix[start.0 + 1][start.1]) {
        return (start.0 + 1, start.1);
    }
    if left.contains(&matrix[start.0][start.1 - 1]) {
        return (start.0, start.1 - 1);
    }
    if right.contains(&matrix[start.0][start.1 + 1]) {
        return (start.0, start.1 + 1);
    }
    println!("ah shit there's nowhere to go");
    start
}

fn traverse_matrix(start: (usize, usize), matrix: &Vec<Vec<u8>>) -> u32 {
    let matrix = matrix.clone();
    let mut num_steps: u32 = 0;
    let mut current_location: (usize, usize) = start.clone();
    let mut current_direction: (usize, usize) = (1, 1); //down and right
                                                        // (1, 1) is down right, (1, -1) is down left, (-1, 1) is up right, (-1, -1) is up left
    while current_location != start {}

    3
}

fn part1(matrix: &Vec<Vec<u8>>) -> u32 {
    let matrix = matrix.clone();
    let cords: (usize, usize) = find_animal(&matrix, b'S').unwrap();
    3
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
        let ex_input = "";
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

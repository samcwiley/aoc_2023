#!/bin/sh

set -e

YEAR=2023

case $1 in
'' | *[!0-9]*)
    echo "Error, a day argument must be passed as a number between 1-25"
    exit 1
    ;;
esac

if [ "$1" -le 1 ] || [ "$1" -ge 25 ]; then
    echo "Error, make sure the day  argument is between 1-25"
    exit 1
fi

site_num=$(echo "$1" | sed 's/^0*//')

if [ "${#1}" -eq 1 ]; then
    file_num="0$1"
else
    file_num="$1"
fi

if ! [ -e ".session" ]; then
    echo "Error, you need to get the session cookie from the AoC website and put it in a file .session"
    exit 1
fi

if [ -e "src/.inputs/input$file_num.txt" ]; then
    echo "Day $site_num input file already exists! Skipping download"
else
    mkdir -p src/.inputs
    curl "https://adventofcode.com/$YEAR/day/$site_num/input" \
        --silent --max-time 10 --cookie "session=$(cat .session)" >"src/.inputs/input$file_num.txt"
fi

if [ -e "/src/bin/day$file_num.rs" ]; then
    echo "Day $site_num solution file already exists; good luck!"
else
    cat <<'EOF' >src/bin/day"$file_num".rs
#![allow(unused_variables, dead_code, unused_imports)]

fn main() {
    let input = include_str!("../.inputs/input$file_num.txt");
    let data = parse_input(input);
    //let part1 = part1(data);
    //dbg!(part1);
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
        assert_eq!(result2, ex_answer2);
        */
    }
}
EOF
    echo "Created solution template at src/bin/day$file_num.rs"

fi

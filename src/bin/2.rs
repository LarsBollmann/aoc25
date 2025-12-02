use core::num;

use aoc25::input::get_input;
use nom::number;

type Number = usize;
type Range = (Number, Number);

fn parse_input(input: &str) -> Vec<Range> {
    let ranges = input.trim_end().split_terminator(',');
    ranges
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            (
                start
                    .parse()
                    .unwrap_or_else(|_| panic!("Error while parsing start number: {}", start)),
                end.parse()
                    .unwrap_or_else(|_| panic!("Error while parsing end number: {}", end)),
            )
        })
        .collect::<Vec<Range>>()
}

fn check_if_repeats(number_string: &str, seq_len: usize) -> bool {
    if  seq_len == 0
        || number_string.is_empty()
        || number_string.len() % seq_len != 0
    {
        return false;
    }

    let (first_seq, mut rest) = number_string.split_at(seq_len);

    for _ in 0..(rest.len() / seq_len) {
        let (current_seq, new_rest) = rest.split_at(seq_len);
        rest = new_rest;
        if current_seq != first_seq {
            return false;
        }
    }
    true
}

fn part_one(ranges: Vec<Range>) -> Number {
    ranges
        .iter()
        .map(|range| {
            (range.0..=range.1)
                .filter(|i| {
                    let number_str = i.to_string();
                    number_str.len() % 2 == 0 && check_if_repeats(&number_str, number_str.len() / 2)
                })
                .sum::<Number>()
        })
        .sum::<Number>()
}

fn part_two(ranges: Vec<Range>) -> usize {
    ranges
        .iter()
        .map(|range| {
            (range.0..=range.1)
                .filter(|i| {
                    let number_str = i.to_string();
                    for i in 1..=(number_str.len() / 2) {
                        if check_if_repeats(&number_str, i) {
                            return true;
                        }
                    }
                    false
                })
                .sum::<Number>()
        })
        .sum::<Number>()
}

fn main() {
    let input = get_input(2);
    let input = parse_input(&input);
    println!("Part one: {}", part_one(input.clone()));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc25::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(2);
        let input = parse_input(&input);
        assert_eq!(part_one(input), 1227775554);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(2);
        let input = parse_input(&input);
        assert_eq!(part_two(input), 4174379265);
    }
}

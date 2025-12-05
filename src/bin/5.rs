use std::{collections::HashSet, ops::Range};

use aoc25::input::get_input;

const DAY: usize = 5;
type Input = (Vec<Range<usize>>, Vec<usize>);

fn parse_input(input: &str) -> Input {
    let fresh_ranges_str = input.lines().take_while(|line| !line.is_empty());

    let fresh_ranges = fresh_ranges_str
        .map(|fresh_range| {
            let parts: Vec<&str> = fresh_range.split('-').collect();
            let start: usize = parts[0].parse().unwrap();
            let end: usize = parts[1].parse().unwrap();
            start..end + 1
        })
        .collect::<Vec<Range<usize>>>();

    let items = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    (fresh_ranges, items)
}

fn part_one(input: Input) -> usize {
    let (fresh_ranges, items) = input;
    items
        .iter()
        .filter(|&&item| {
            fresh_ranges.iter().any(|fresh_range| {
                if fresh_range.contains(&item) {
                    return true;
                }
                false
            })
        })
        .count()
}

fn ranges_do_overlap(r1: &Range<usize>, r2: &Range<usize>) -> bool {
    r1.start < r2.end && r2.start < r1.end
}

fn part_two(input: Input) -> usize {
    let (mut fresh_ranges, _) = input;
    fresh_ranges.sort_by_key(|r| r.start);

    let mut merged_ranges: Vec<Range<usize>> = Vec::new();

    for fresh_range in fresh_ranges {
        if let Some(last_range) = merged_ranges.last_mut() {
            if ranges_do_overlap(last_range, &fresh_range) || last_range.end == fresh_range.start {
                last_range.end = last_range.end.max(fresh_range.end);
                continue;
            }
        }
        merged_ranges.push(fresh_range);
    }

    merged_ranges.iter().map(|range| range.len()).sum()
}

fn main() {
    let input_string = get_input(DAY);
    let input = parse_input(&input_string);

    let result_1 = part_one(input.clone());
    println!("Part one: {}", result_1);

    let result_2 = part_two(input);
    println!("Part two: {}", result_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc25::input::get_example;

    #[test]
    fn test_part_one() {
        let input_string = get_example(DAY);
        let input = parse_input(&input_string);
        let result = part_one(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_two() {
        let input_string = get_example(DAY);
        let input = parse_input(&input_string);
        let result = part_two(input);
        assert_eq!(result, 14);
    }
}

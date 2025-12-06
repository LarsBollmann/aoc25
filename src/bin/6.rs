use core::{num, panic};
use std::str::FromStr;

use aoc25::input::get_input;

const DAY: usize = 6;
type Input = Vec<Problem>;

#[derive(Debug, Clone)]
enum Operation {
    None,
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Operation::Multiply),
            "+" => Ok(Operation::Add),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    pub numbers: Vec<usize>,
    pub operation: Operation,
}

impl Problem {
    pub fn solve(&self) -> usize {
        match self.operation {
            Operation::Add => self.numbers.iter().sum::<usize>(),
            Operation::Multiply => self.numbers.iter().product::<usize>(),
            Operation::None => panic!("No operation found"),
        }
    }
}

fn parse_input(input_str: &str) -> Input {
    let mut input = Vec::new();

    for line in input_str.lines() {
        for (i, text) in line.split_whitespace().enumerate() {
            if input.len() < i + 1 {
                input.push(Problem {
                    numbers: vec![],
                    operation: Operation::None,
                });
            }

            if let Ok(number) = text.parse::<usize>() {
                input.get_mut(i).unwrap().numbers.push(number);
            } else {
                input.get_mut(i).unwrap().operation = Operation::from_str(text).unwrap();
            }
        }
    }

    input
}

fn part_one(input: Input) -> usize {
    input.iter().map(|problem| problem.solve()).sum()
}

fn part_two(mut input: Input) -> usize {
    1
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
        println!("{:?}", input);
        let result = part_one(input);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part_two() {
        let input_string = get_example(DAY);
        let input = parse_input(&input_string);
        let result = part_two(input);
        assert_eq!(result, 3263827);
    }
}

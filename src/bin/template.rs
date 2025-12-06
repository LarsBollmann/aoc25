use aoc25::input::get_input;

const DAY: usize = 1; 
type Input = ();

fn parse_input(input_str: &str) -> Input {
    
}

fn part_one(input: Input) -> usize {
   1
}

fn part_two(input: Input) -> usize {
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
        let result = part_one(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_two() {
        let input_string = get_example(DAY);
        let input = parse_input(&input_string);
        let result = part_two(input);
        assert_eq!(result, 0);

    }
}

use aoc25::input::get_input;

const DAY: usize = 1; 

fn parse_input(input: &str) {
    
}

fn part_one() -> usize {
   1
}

fn part_two() -> usize {
    1
}

fn main() {
    let input_string = get_input(DAY);
    let input = parse_input(&input_string);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc25::input::get_example;

    #[test]
    fn test_part_one() {
        let input_string = get_example(DAY);
        let input = parse_input(&input_string);
        //let result = part_one(input);
        //assert_eq!(result, )
    }

    #[test]
    fn test_part_two() {
        let input_string = get_example(DAY);
        let input = parse_input(&input_string);
        //let result = part_two(input);
        //assert_eq!(result, )

    }
}

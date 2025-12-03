use aoc25::input::get_input;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let lines = input.trim().lines();
    lines.map(|line| {
        line.chars().collect::<Vec<char>>()
    }).collect()
}

fn part_one(input: Vec<Vec<char>>) -> usize {
    input.iter().map(|row| {
        let length = row.len();
        (0..length).map(|start_index| {
            (start_index+1..length).map(|end_index| {
                format!("{}{}", row[start_index], row[end_index]).parse::<usize>().unwrap()
            }).max().unwrap_or(0)
        }).max().unwrap()
    }).sum()
}

// Returns the index and value of the first maximum element
fn get_leftmost_max_element(batteries: &[char]) -> (usize, &char) {
    batteries
        .iter()
        .enumerate()
        .rev()
        .max_by(|(_, a), (_, b)| a.to_string().parse::<usize>().unwrap().partial_cmp(&b.to_string().parse::<usize>().unwrap()).unwrap())
        .unwrap()
}

fn part_two(input: Vec<Vec<char>>) -> usize {
    input.iter().map(|row| {
        let mut result = String::new();
        let mut last_index = 0;
        for i in (1..=12).rev() {
            // Take only the first n elements so that there are enough left to get to a total of 12
            let candidates = &row[last_index..=row.len()-i];
            // Get the index and value of the first maximum number
            let (index, number) = get_leftmost_max_element(candidates);
            // Move the new start of the search window 1 behind the last picked number
            last_index = last_index+index+1;
            // Add the highest number to the final result
            result.push(*number);
        }
        result.parse::<usize>().unwrap()
    }).sum()
}

fn main() {
    let input_string = get_input(3);
    let input = parse_input(&input_string);
    let result = part_one(input.clone());
    println!("Part one: {}", result);
    let result2 = part_two(input);
    println!("Part two: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc25::input::get_example;

    #[test]
    fn test_part_one() {
        let input_string = get_example(3);
        let input = parse_input(&input_string);
        let result = part_one(input);
        assert_eq!(result, 357)
    }

    #[test]
    fn test_part_two() {
        let input_string = get_example(3);
        let input = parse_input(&input_string);
        let result = part_two(input);
        assert_eq!(result, 3121910778619);
    }
}

use std::collections::HashSet;

use aoc25::input::get_input;

const DAY: usize = 4;
type Input = Vec<Vec<char>>;

fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_adjacent_coordinates(x: usize, y: usize, len_x: usize, len_y: usize) -> Vec<(usize, usize)> {
    let mut adjacent_coords = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;
            if new_x >= 0 && new_x < len_x as isize && new_y >= 0 && new_y < len_y as isize {
                adjacent_coords.push((new_x as usize, new_y as usize));
            }
        }
    }
    adjacent_coords
}

fn part_one(input: Input) -> usize {
    let mut rolls = 0;
    let len_x = input.len();
    let len_y = input[0].iter().len();

    for x in 0..len_x {
        for y in 0..len_y {
            if input[x][y] != '@' {
                continue;
            }

            let mut adjacent_rolls = 0;

            let adjacent_coordinates = get_adjacent_coordinates(x, y, len_x, len_y);
            for (adj_x, adj_y) in adjacent_coordinates {
                if input[adj_x][adj_y] == '@' {
                    adjacent_rolls += 1;
                }
            }

            if adjacent_rolls < 4 {
                rolls += 1;
            }
        }
    }

    rolls
}

fn part_two(mut input: Input) -> usize {
    let mut rolls = 0;
    let len_x = input.len();
    let len_y = input[0].iter().len();

    let mut coordinates_to_look_at: HashSet<(usize, usize)> = (0..len_x).flat_map(|x| (0..len_y).map(move |y| (x, y))).collect();

    while !coordinates_to_look_at.is_empty() {
        coordinates_to_look_at = coordinates_to_look_at.into_iter().fold(
            HashSet::new(),
            |mut acc: HashSet<(usize, usize)>, (x, y)| {
                if input[x][y] != '@' {
                    return acc;
                }

                let mut adjacent_rolls = 0;

                let adjacent_coordinates = get_adjacent_coordinates(x, y, len_x, len_y);
                for (adj_x, adj_y) in &adjacent_coordinates {
                    if input[*adj_x][*adj_y] == '@' {
                        adjacent_rolls += 1;
                    }
                }

                if adjacent_rolls < 4 {
                    rolls += 1;
                    input[x][y] = '#';
                    acc.extend(adjacent_coordinates);
                }
                acc
            },
        );
    }

    rolls
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
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_two() {
        let input_string = get_example(DAY);
        let input = parse_input(&input_string);
        let result = part_two(input);
        assert_eq!(result, 43);
    }
}

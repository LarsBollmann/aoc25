use std::str::FromStr;
use aoc25::input::get_input;

#[derive(Debug)]
enum Rotation {
    Left(u32),
    Right(u32),
}

#[derive(Debug)]
struct Dial {
    position: u32,
    max_position: u32,
    lands_on_0: u32,
    pointed_at_0: u32,
}

impl FromStr for Rotation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, num_str) = s.split_at(1);
        let number: u32 = num_str.parse::<u32>().map_err(|_| ())?;
        match dir {
            "L" => Ok(Rotation::Left(number)),
            "R" => Ok(Rotation::Right(number)),
            _ => Err(()),
        }
    }
}

impl Dial {
    fn from_position(position: u32) -> Self {
        Self {
            position,
            max_position: 99,
            lands_on_0: 0,
            pointed_at_0: 0,
        }
    }

    fn rotate(&mut self, rotation: &Rotation) {
        match rotation {
            Rotation::Left(degrees) => {
                let modulo_rotation = degrees % (self.max_position + 1);
                let additional_wraps = degrees / (self.max_position + 1);

                if modulo_rotation > self.position {
                    if self.position != 0 {
                        self.pointed_at_0 += 1;
                    }
                    self.position = self.max_position + 1 - (modulo_rotation - self.position);
                    self.pointed_at_0 += additional_wraps;
                } else {
                    self.position -= modulo_rotation;
                    self.pointed_at_0 += additional_wraps;
                }
            }
            Rotation::Right(degrees) => {
                let modulo_rotation = degrees % (self.max_position + 1);
                let additional_wraps = degrees / (self.max_position + 1);

                if modulo_rotation > self.max_position - self.position {
                    self.position = self.position + modulo_rotation - self.max_position - 1;
                    self.pointed_at_0 += additional_wraps;
                    if self.position != 0 {
                        self.pointed_at_0 += 1;
                    }
                } else {
                    self.position += modulo_rotation;
                    self.pointed_at_0 += additional_wraps;
                }
            }
        }
        if self.position == 0 {
            self.lands_on_0 += 1;
            self.pointed_at_0 += 1;
        }
    }
}

fn part_one_and_two(input: &str) -> (u32, u32) {
    let rotations = parse_input(input);
    let mut dial = Dial::from_position(50);

    for rotation in rotations.iter() {
        dial.rotate(rotation);
    }

    println!("Part 1: Times landed on 0: {}", dial.lands_on_0);
    println!("Part 2: Times pointed to 0: {}", dial.pointed_at_0);

    (dial.lands_on_0, dial.pointed_at_0)
}

fn parse_input(input: &str) -> Vec<Rotation> {
    let lines = input.lines();
    let rotations: Vec<Rotation> = lines
        .map(Rotation::from_str)
        .filter_map(Result::ok)
        .collect();

    rotations
}

fn main() {
    let input = get_input(1);
    part_one_and_two(&input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc25::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(1);
        let (s1, _) = part_one_and_two(&input);
        assert_eq!(s1, 3);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(1);
        let (_, s2) = part_one_and_two(&input);
        assert_eq!(s2, 6);    
    }
}

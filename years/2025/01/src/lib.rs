type Input = Vec<Turn>;
type Output1 = usize;
type Output2 = Output1;

pub enum Turn {
    Left(u64),
    Right(u64),
}

impl Turn {
    fn to_number_to_add(&self) -> i32 {
        match self {
            Turn::Left(n) => -(*n as i32),
            Turn::Right(n) => *n as i32,
        }
    }
}

const DIAL_POSITIONS: i32 = 100;
const DIAL_START_POSITION: i32 = 50;

pub fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();
            let (direction, count) = line.split_at(1);
            let direction = direction.chars().next().unwrap();
            let count = count.parse().expect("Invalid number of turns");

            match direction {
                'L' => Turn::Left(count),
                'R' => Turn::Right(count),
                _ => panic!("Unknown turn direction"),
            }
        })
        .collect()
}

pub fn part_1(input: &Input) -> Output1 {
    input
        .iter()
        .scan(DIAL_START_POSITION, |current_position, turn| {
            *current_position += turn.to_number_to_add();
            *current_position = current_position.rem_euclid(DIAL_POSITIONS);
            Some(*current_position)
        })
        .filter(|&position| position == 0)
        .count()
}

pub fn part_2(input: &Input) -> Output2 {
    input
        .iter()
        .fold(
            (DIAL_START_POSITION, 0usize),
            |(mut current_position, zeroes), turn| {
                let new_position = current_position + turn.to_number_to_add();
                let zeroes_passed = if new_position == 0 {
                    1
                } else {
                    (new_position / 100).unsigned_abs() as usize
                        + if current_position != 0
                            && new_position.signum() != current_position.signum()
                        {
                            1
                        } else {
                            0
                        }
                };
                current_position = new_position.rem_euclid(DIAL_POSITIONS);
                (current_position, zeroes + zeroes_passed)
            },
        )
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 6);
    }
}

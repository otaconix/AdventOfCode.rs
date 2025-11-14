use std::{
    cmp::Reverse,
    collections::{HashSet, VecDeque},
    io,
};

use aoc_timing::trace::log_run;
use grid::Grid;
use intcode::{Computer, OpCode, SplitIO};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pixel {
    Scaffold,
    OpenSpace,
    Robot(char),
}

type Input = (Computer, Grid<Pixel>);
type Output1 = usize;
type Output2 = i64;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(mut input: I) -> Input {
    let line = input.next().expect("No input line!");
    let original_computer = Computer::parse(line.as_ref());
    let mut computer = original_computer.clone();
    let mut io = VecDeque::new();

    assert_eq!(computer.run(&mut io), OpCode::Terminate);

    (
        original_computer,
        io.into_iter()
            .map(|c| c as u8 as char)
            .collect::<String>()
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|pixel| match pixel {
                        '#' => Pixel::Scaffold,
                        '.' => Pixel::OpenSpace,
                        '>' | '<' | '^' | 'v' => Pixel::Robot(pixel),
                        _ => panic!("Unknown pixel {pixel}"),
                    })
                    .collect::<Vec<_>>()
            })
            .filter(|row| !row.is_empty())
            .collect(),
    )
}

fn part_1((_, input): &Input) -> Output1 {
    let picture = (0..input.height())
        .map(|row_num| input.row(row_num))
        .map(|row| {
            row.map(|pixel| match pixel {
                Pixel::Scaffold => '#',
                Pixel::OpenSpace => '.',
                Pixel::Robot(direction) => match direction {
                    '>' => '→',
                    '<' => '←',
                    '^' => '↑',
                    'v' => '↓',
                    _ => '?',
                },
            })
            .collect::<String>()
        })
        .join("\n");

    println!("{picture}");

    input
        .coordinates()
        .filter(|coord| {
            let pixel = input.get_coord(*coord).unwrap();
            pixel == &Pixel::Scaffold
                && input
                    .get_neighbors(coord.0, coord.1)
                    .into_iter()
                    .map(|neighbor| input.get_coord(neighbor).unwrap())
                    .filter(|pixel| pixel == &&Pixel::Scaffold)
                    .count()
                    == 4
        })
        // .inspect(|scaffold_intersection| {
        //     println!("Scaffold intersection found at {scaffold_intersection:?}")
        // })
        .map(|scaffold_intersection| scaffold_intersection.0 * scaffold_intersection.1)
        .sum()
}

const NUMBER_REPLACEMENTS: [&str; 26] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z",
];

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            '^' => Self::Up,
            'v' => Self::Down,
            _ => panic!("Unknown direction {c}"),
        }
    }

    fn turn_from(&self, from: &Self) -> Option<String> {
        match (from, self) {
            (Direction::Left, Direction::Down)
            | (Direction::Right, Direction::Up)
            | (Direction::Up, Direction::Left)
            | (Direction::Down, Direction::Right) => Some("L"),
            (Direction::Left, Direction::Up)
            | (Direction::Right, Direction::Down)
            | (Direction::Up, Direction::Right)
            | (Direction::Down, Direction::Left) => Some("R"),
            (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up) => {
                unimplemented!("Implement multiple turns if necessary")
            }
            (Direction::Left, Direction::Left)
            | (Direction::Right, Direction::Right)
            | (Direction::Up, Direction::Up)
            | (Direction::Down, Direction::Down) => None,
        }
        .map(|turn| turn.to_string())
    }
}

fn part_2((computer, picture): &Input) -> Output2 {
    let (mut current_x, mut current_y) = picture
        .coordinates()
        .find(|coord| matches!(picture.get_coord(*coord).unwrap(), Pixel::Robot(_)))
        .unwrap();
    let mut robot_direction = Direction::from_char(
        *picture
            .get(current_x, current_y)
            .map(|robot| match robot {
                Pixel::Robot(direction) => direction,
                _ => panic!(),
            })
            .unwrap(),
    );
    let mut visited_scaffolds: HashSet<(usize, usize)> = HashSet::new();
    let mut instructions = vec![];

    loop {
        let mut line_of_sight_neighbors = picture.get_line_of_sight_neighbors(current_x, current_y);
        if let Some((next_direction, _, line_length)) = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .map(|direction| {
            (
                direction,
                match direction {
                    Direction::Left => (current_x - 1, current_y),
                    Direction::Right => (current_x + 1, current_y),
                    Direction::Down => (current_x, current_y + 1),
                    Direction::Up => (current_x, current_y - 1),
                },
                match direction {
                    Direction::Left => {
                        line_of_sight_neighbors.left.reverse();
                        &line_of_sight_neighbors.left
                    }
                    Direction::Right => &line_of_sight_neighbors.right,
                    Direction::Down => &line_of_sight_neighbors.down,
                    Direction::Up => {
                        line_of_sight_neighbors.up.reverse();
                        &line_of_sight_neighbors.up
                    }
                }
                .iter()
                .take_while(|pixel| pixel == &&&Pixel::Scaffold)
                .count(),
            )
        })
        .find(|(_, coord, length)| *length > 0 && !visited_scaffolds.contains(coord))
        {
            if let Some(turn) = next_direction.turn_from(&robot_direction) {
                robot_direction = next_direction;
                instructions.push(turn);
            }

            instructions.push(NUMBER_REPLACEMENTS[line_length].to_string());
            // instructions.push(line_length.to_string());

            visited_scaffolds.extend(match next_direction {
                Direction::Left => (current_x - line_length..current_x)
                    .map(|x| (x, current_y))
                    .collect_vec(),
                Direction::Right => (current_x + 1..=current_x + line_length)
                    .map(|x| (x, current_y))
                    .collect_vec(),
                Direction::Up => (current_y - line_length..current_y)
                    .map(|y| (current_x, y))
                    .collect_vec(),
                Direction::Down => (current_y + 1..=current_y + line_length)
                    .map(|y| (current_x, y))
                    .collect_vec(),
            });

            match next_direction {
                Direction::Left => {
                    current_x -= line_length;
                }
                Direction::Right => current_x += line_length,
                Direction::Up => current_y -= line_length,
                Direction::Down => current_y += line_length,
            }
        } else {
            break;
        }
    }

    let instructions = instructions.join("");
    println!("Instructions: {instructions}");

    let (main_routine, functions) =
        determine_movement_functions(&instructions, &["A", "B", "C"]).unwrap();

    let main_routine = main_routine.chars().map(|c| c.to_string()).join(",");
    let functions = functions
        .into_iter()
        .map(|(function, _)| function)
        .map(|function| {
            function
                .chars()
                .map(|c| {
                    if c.is_ascii_lowercase() {
                        NUMBER_REPLACEMENTS
                            .iter()
                            .position(|n| &c.to_string() == n)
                            .unwrap()
                            .to_string()
                    } else {
                        c.to_string()
                    }
                })
                .join(",")
        })
        .collect_vec();

    println!("Main routine: {main_routine}; functions: {functions:#?}");

    let mut computer = computer.clone();
    computer.write(0, 2);
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();

    main_routine.chars().for_each(|c| input.push_back(c as i64));
    input.push_back(b'\n' as i64);
    for function in functions {
        function.chars().for_each(|c| input.push_back(c as i64));
        input.push_back(b'\n' as i64);
    }
    input.push_back(b'n' as i64);
    input.push_back(b'\n' as i64);

    computer.run(&mut SplitIO::new(&mut input, &mut output));

    println!("Output size: {}", output.len());

    output
        .pop_back()
        .expect("Robot didn't return dust collected.")
}

fn assemble_main_routine(program: &str, movement_functions: &[(&str, &str)]) -> String {
    movement_functions
        .iter()
        .fold(program.to_string(), |result, (function, name)| {
            result.replace(function, name)
        })
}

fn determine_movement_functions<'a, 'b>(
    program: &'a str,
    available_function_names: &[&'b str],
) -> Option<(String, Vec<(&'a str, &'b str)>)> {
    let mut movement_functions = vec![];

    fn inner<'a, 'b>(
        program: &'a str,
        substrings_sorted_by_occurrences: &[&'a str],
        remaining_chunks: &[&'a str],
        available_function_names: &[&'b str],
        movement_functions: &mut Vec<(&'a str, &'b str)>,
    ) -> Option<String> {
        if available_function_names.is_empty() {
            if !remaining_chunks.is_empty() {
                None
            } else {
                // println!("Trying functions {movement_functions:?}");
                Some(assemble_main_routine(program, movement_functions))
                    .filter(|main_routine| main_routine.len() <= 20)
            }
        } else {
            for substring in substrings_sorted_by_occurrences {
                let new_remaining_chunks: Vec<_> = remaining_chunks
                    .iter()
                    .flat_map(|to_split| to_split.split(substring))
                    .filter(|chunk| !chunk.is_empty())
                    .collect();

                movement_functions.push((substring, available_function_names[0]));

                let inner_result = inner(
                    program,
                    substrings_sorted_by_occurrences,
                    &new_remaining_chunks,
                    &available_function_names[1..],
                    movement_functions,
                );

                if inner_result.is_some() {
                    return inner_result;
                } else {
                    movement_functions.pop();
                }
            }

            None
        }
    }

    let unique_substring_occurrences: Vec<_> = (0..program.len())
        .flat_map(|start| {
            (start + 2..program.len().min(start + 20))
                .step_by(2)
                .map(move |end| &program[start..end])
        })
        .unique()
        .sorted_by_key(|substring| {
            Reverse((
                program
                    .as_bytes()
                    .windows(substring.len())
                    .filter(|&w| w == substring.as_bytes())
                    .count(),
                substring.len(),
            ))
        })
        .collect();

    inner(
        program,
        &unique_substring_occurrences,
        &[program],
        available_function_names,
        &mut movement_functions,
    )
    .map(move |main_routine| (main_routine, movement_functions))
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        let part_1 = log_run("Part 1", || part_1(&input));
        println!("Part 1: {part_1}");

        let part_2 = log_run("Part 2", || part_2(&input));
        println!("Part 2: {part_2}");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOVEMENT_LIST: &str = "R8R8R4R4R8L6L2R4R4R8R8R8L6L2";
    const EXPECTED_MAIN_ROUTINE: &str = "ABCBAC";
    const EXPECTED_MOVEMENT_FUNCTIONS: [(&str, &str); 3] =
        [("R4R4R8", "B"), ("R8R8", "A"), ("L6L2", "C")];

    #[test]
    fn assemble_main_routine_test() {
        let result = assemble_main_routine(MOVEMENT_LIST, &EXPECTED_MOVEMENT_FUNCTIONS);

        assert_eq!(result, EXPECTED_MAIN_ROUTINE.to_string());
    }

    #[test]
    fn determine_routines_short_test() {
        let (main_routine, functions) =
            determine_movement_functions(MOVEMENT_LIST, &["A", "B", "C"]).unwrap();

        assert_eq!(
            MOVEMENT_LIST,
            functions
                .into_iter()
                .fold(main_routine, |program, (function, name)| {
                    program.replace(name, function)
                })
        );
    }
}

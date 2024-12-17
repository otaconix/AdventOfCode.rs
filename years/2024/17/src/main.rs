use std::io;
use std::ops::Index;
use std::ops::IndexMut;

use aoc_timing::trace::log_run;
use itertools::Itertools;
use log::debug;

#[derive(Default, Debug, Clone, Copy)]
struct Registers([usize; 3]);

enum Register {
    A,
    B,
    C,
}

impl Index<Register> for Registers {
    type Output = usize;

    fn index(&self, index: Register) -> &Self::Output {
        &self.0[match index {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
        }]
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.0[match index {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
        }]
    }
}

struct Computer {
    registers: Registers,
    instructions: Vec<usize>,
    instruction_pointer: usize,
    output: Vec<usize>,
    instruction_counter: usize,
}

impl Computer {
    fn new(registers: Registers, instructions: Vec<usize>) -> Self {
        Self {
            registers,
            instructions,
            instruction_pointer: 0,
            instruction_counter: 0,
            output: vec![],
        }
    }

    fn run(&mut self) -> String {
        while self.instruction_pointer < self.instructions.len() {
            let instruction = self.instructions[self.instruction_pointer];
            let operand = self.instructions[self.instruction_pointer + 1];

            let mut jumped = false;
            match instruction {
                0 => self.registers[Register::A] >>= self.combo_value(operand),
                1 => self.registers[Register::B] ^= operand,
                2 => self.registers[Register::B] = self.combo_value(operand) % 8,
                3 => {
                    if self.registers[Register::A] != 0 {
                        self.instruction_pointer = operand as usize;
                        jumped = true;
                    }
                }
                4 => self.registers[Register::B] ^= self.registers[Register::C],
                5 => self.output.push(self.combo_value(operand) % 8),
                6 => {
                    self.registers[Register::B] =
                        self.registers[Register::A] >> self.combo_value(operand)
                }
                7 => {
                    self.registers[Register::C] =
                        self.registers[Register::A] >> self.combo_value(operand)
                }
                _ => {
                    panic!("Unknown instruction {instruction}")
                }
            }

            if !jumped {
                self.instruction_pointer += 2;
            }
            self.instruction_counter += 1;
        }

        debug!("Ran {} instructions.", self.instruction_counter);
        self.output.iter().join(",")
    }

    fn combo_value(&self, combo: usize) -> usize {
        match combo {
            0..=3 => combo,
            4 => self.registers[Register::A],
            5 => self.registers[Register::B],
            6 => self.registers[Register::C],
            _ => panic!("Invalid combo operand {}", combo),
        }
    }
}

#[derive(Debug)]
struct Input {
    registers: Registers,
    instructions: Vec<usize>,
}
type Output = String;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    enum State {
        Registers(Registers),
        Instructions(Registers, Vec<usize>),
    }
    let end_state = input.fold(State::Registers(Registers::default()), |state, line| {
        let line = line.as_ref();

        match state {
            State::Registers(mut registers) => {
                if line.is_empty() {
                    State::Instructions(registers, vec![])
                } else {
                    let (name, value) = line.split_once(": ").unwrap();
                    let (_, name) = name.split_once(' ').unwrap();
                    let value = value.parse().unwrap();

                    registers[match name {
                        "A" => Register::A,
                        "B" => Register::B,
                        "C" => Register::C,
                        _ => panic!("Unknown register named {name}"),
                    }] = value;

                    State::Registers(registers)
                }
            }
            State::Instructions(registers, mut instructions) => {
                let (_, raw_instructions) = line.split_once(": ").unwrap();
                instructions.extend(
                    raw_instructions
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap()),
                );

                State::Instructions(registers, instructions)
            }
        }
    });

    match end_state {
        State::Instructions(registers, instructions) => Input {
            registers,
            instructions,
        },
        _ => panic!("Invalid state when done parsing"),
    }
}

fn part_1(input: &Input) -> Output {
    Computer::new(input.registers, input.instructions.clone()).run()
}

fn part_2(input: &Input) -> Output {
    "".to_string()
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

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, "");
    }

    #[test]
    fn example_1() {
        let mut computer = Computer::new(Registers([0, 0, 9]), vec![2, 6]);
        computer.run();

        assert_eq!(computer.registers[Register::B], 1);
    }

    #[test]
    fn example_2() {
        let mut computer = Computer::new(Registers([10, 0, 0]), vec![5, 0, 5, 1, 5, 4]);
        let result = computer.run();

        assert_eq!(result, "0,1,2");
    }

    #[test]
    fn example_3() {
        let mut computer = Computer::new(Registers([2024, 0, 0]), vec![0, 1, 5, 4, 3, 0]);
        let result = computer.run();

        assert_eq!(result, "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(computer.registers[Register::A], 0);
    }

    #[test]
    fn example_4() {
        let mut computer = Computer::new(Registers([0, 29, 0]), vec![1, 7]);
        computer.run();

        assert_eq!(computer.registers[Register::B], 26);
    }

    #[test]
    fn example_5() {
        let mut computer = Computer::new(Registers([0, 2024, 43690]), vec![4, 0]);
        computer.run();

        assert_eq!(computer.registers[Register::B], 44354);
    }
}

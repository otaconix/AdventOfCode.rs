use std::io;
use std::ops::Index;
use std::ops::IndexMut;

use aoc_timing::trace::log_run;
use itertools::Itertools;
use log::debug;

#[derive(Default, Debug, Clone, Copy)]
struct Registers([isize; 3]);

enum Register {
    A,
    B,
    C,
}

impl Index<Register> for Registers {
    type Output = isize;

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

#[derive(Debug, Clone, Copy)]
struct ComboOperand(isize);

impl ComboOperand {
    fn value(&self, computer: &Computer) -> isize {
        match self.0 {
            0..=3 => self.0,
            4 => computer.registers[Register::A],
            5 => computer.registers[Register::B],
            6 => computer.registers[Register::C],
            _ => panic!("Invalid combo operand {}", self.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(isize),
    Bst(ComboOperand),
    Jnz(isize),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl Instruction {
    fn evaluate(&self, computer: &mut Computer) -> bool {
        let mut jumped = false;

        match *self {
            Instruction::Adv(operand) => {
                computer.registers[Register::A] /= 1 << operand.value(computer)
            }
            Instruction::Bxl(operand) => computer.registers[Register::B] ^= operand,
            Instruction::Bst(operand) => {
                computer.registers[Register::B] = operand.value(computer) & 0b111
            }
            Instruction::Jnz(operand) => {
                if computer.registers[Register::A] != 0 {
                    computer.instruction_pointer = operand as usize;
                    jumped = true;
                }
            }
            Instruction::Bxc => computer.registers[Register::B] ^= computer.registers[Register::C],
            Instruction::Out(operand) => computer.output.push(operand.value(computer) & 0b111),
            Instruction::Bdv(operand) => {
                computer.registers[Register::B] =
                    computer.registers[Register::A] / (1 << operand.value(computer))
            }
            Instruction::Cdv(operand) => {
                computer.registers[Register::C] =
                    computer.registers[Register::A] / (1 << operand.value(computer))
            }
        }

        jumped
    }
}

struct Computer {
    registers: Registers,
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
    output: Vec<isize>,
    instruction_counter: usize,
}

impl Computer {
    fn new(registers: Registers, instructions: Vec<Instruction>) -> Self {
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
            let jumped = self.instructions[self.instruction_pointer]
                .clone()
                .evaluate(self);

            if !jumped {
                self.instruction_pointer += 1;
            }

            self.instruction_counter += 1;
        }

        debug!("Ran {} instructions.", self.instruction_counter);
        self.output.iter().join(",")
    }
}

#[derive(Debug)]
struct Input {
    registers: Registers,
    instructions: Vec<Instruction>,
}
type Output = String;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    enum State {
        Registers(Registers),
        Instructions(Registers, Vec<Instruction>),
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
                        .map(|n| n.parse::<isize>().unwrap())
                        .tuple_windows()
                        .map(|(instruction, operand)| match instruction {
                            0 => Instruction::Adv(ComboOperand(operand)),
                            1 => Instruction::Bxl(operand),
                            2 => Instruction::Bst(ComboOperand(operand)),
                            3 => Instruction::Jnz(operand),
                            4 => Instruction::Bxc,
                            5 => Instruction::Out(ComboOperand(operand)),
                            6 => Instruction::Bdv(ComboOperand(operand)),
                            7 => Instruction::Cdv(ComboOperand(operand)),
                            _ => panic!("Unknown instruction {instruction}"),
                        }),
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
    let mut computer = Computer::new(input.registers, input.instructions.clone());

    computer.run()
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
        let mut computer = Computer::new(
            Registers([0, 0, 9]),
            vec![Instruction::Bst(ComboOperand(6))],
        );
        computer.run();

        assert_eq!(computer.registers[Register::B], 1);
    }

    #[test]
    fn example_2() {
        let mut computer = Computer::new(
            Registers([10, 0, 0]),
            vec![
                Instruction::Out(ComboOperand(0)),
                Instruction::Out(ComboOperand(1)),
                Instruction::Out(ComboOperand(4)),
            ],
        );
        let result = computer.run();

        assert_eq!(result, "0,1,2");
    }

    #[test]
    fn example_3() {
        let mut computer = Computer::new(
            Registers([2024, 0, 0]),
            vec![
                Instruction::Adv(ComboOperand(1)),
                Instruction::Out(ComboOperand(4)),
                Instruction::Jnz(0),
            ],
        );
        let result = computer.run();

        assert_eq!(result, "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(computer.registers[Register::A], 0);
    }

    #[test]
    fn example_4() {
        let mut computer = Computer::new(Registers([0, 29, 0]), vec![Instruction::Bxl(7)]);
        computer.run();

        assert_eq!(computer.registers[Register::B], 26);
    }

    #[test]
    fn example_5() {
        let mut computer = Computer::new(Registers([0, 2024, 43690]), vec![Instruction::Bxc]);
        computer.run();

        assert_eq!(computer.registers[Register::B], 44354);
    }
}

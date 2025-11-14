use std::{
    collections::VecDeque,
    ops::{Add, Sub},
};

use log::trace;

pub trait InputOutput {
    fn pop(&mut self) -> Option<i64>;
    fn push(&mut self, value: i64);
    fn input_iter(&self) -> impl Iterator<Item = &i64>;
    fn output_iter(&self) -> impl Iterator<Item = &i64>;
}

impl InputOutput for VecDeque<i64> {
    fn pop(&mut self) -> Option<i64> {
        self.pop_front()
    }

    fn push(&mut self, value: i64) {
        self.push_back(value);
    }

    fn input_iter(&self) -> impl Iterator<Item = &i64> {
        self.iter()
    }

    fn output_iter(&self) -> impl Iterator<Item = &i64> {
        self.iter()
    }
}

pub struct NullIO;
impl InputOutput for NullIO {
    fn pop(&mut self) -> Option<i64> {
        None
    }

    fn push(&mut self, _: i64) {}

    fn input_iter(&self) -> impl Iterator<Item = &i64> {
        std::iter::empty()
    }

    fn output_iter(&self) -> impl Iterator<Item = &i64> {
        std::iter::empty()
    }
}

pub struct SplitIO<'a, IO: InputOutput> {
    input: &'a mut IO,
    output: &'a mut IO,
}

impl<'a, IO: InputOutput> SplitIO<'a, IO> {
    pub fn new(input: &'a mut IO, output: &'a mut IO) -> Self {
        Self { input, output }
    }
}

impl<'a, IO: InputOutput> InputOutput for SplitIO<'a, IO> {
    fn pop(&mut self) -> Option<i64> {
        self.input.pop()
    }

    fn push(&mut self, value: i64) {
        self.output.push(value);
    }

    fn input_iter(&self) -> impl Iterator<Item = &i64> {
        self.input.input_iter()
    }

    fn output_iter(&self) -> impl Iterator<Item = &i64> {
        self.output.output_iter()
    }
}

#[derive(Debug, Clone)]
pub struct Computer {
    memory: Vec<i64>,
    relative_base: usize,
    pub instruction_pointer: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    Addition,
    Multiplication,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Terminate,
    AdjustRelativeBase,
}

impl OpCode {
    fn memory_used(&self) -> usize {
        match *self {
            OpCode::Addition => 4,
            OpCode::Multiplication => 4,
            OpCode::Input => 2,
            OpCode::Output => 2,
            OpCode::JumpIfTrue => 3,
            OpCode::JumpIfFalse => 3,
            OpCode::LessThan => 4,
            OpCode::Equals => 4,
            OpCode::AdjustRelativeBase => 2,
            OpCode::Terminate => 1,
        }
    }

    fn evaluate<IO: InputOutput>(
        &self,
        computer: &mut Computer,
        parameter_modes: &[ParameterMode],
        io: &mut IO,
    ) -> Result<usize, OpCode> {
        use OpCode::*;

        trace!(
            "IP={}; opcode={self:?}; param_modes={parameter_modes:?}; relative_base={}",
            computer.instruction_pointer, computer.relative_base,
        );

        trace!(
            "Memory: {:#?}",
            &computer.memory()
                [computer.instruction_pointer..computer.instruction_pointer + self.memory_used()]
        );

        match *self {
            Addition => {
                let params @ [a, b] = computer.get_parameters(parameter_modes);
                let output =
                    computer.read_address(computer.instruction_pointer + 3, &parameter_modes[2]);
                let result = a + b;

                trace!("Addition: params={params:?}; result={result}; output={output}");

                computer.write(output, result);

                Ok(computer.instruction_pointer + 4)
            }
            Multiplication => {
                let params @ [a, b] = computer.get_parameters(parameter_modes);
                let output =
                    computer.read_address(computer.instruction_pointer + 3, &parameter_modes[2]);
                let result = a * b;

                trace!("Multiplication: params={params:?}; result={result}; output={output}");

                computer.write(output, result);

                Ok(computer.instruction_pointer + 4)
            }
            Input => {
                let output =
                    computer.read_address(computer.instruction_pointer + 1, &parameter_modes[0]);
                let result = io.pop();

                trace!("Input: result={result:?}; output={output}");

                if let Some(result) = result {
                    computer.write(output, result);

                    Ok(computer.instruction_pointer + 2)
                } else {
                    Err(*self)
                }
            }
            Output => {
                let params @ [value] = computer.get_parameters(parameter_modes);

                trace!("Output: params={params:?}");

                io.push(value);

                Ok(computer.instruction_pointer + 2)
            }
            JumpIfTrue => {
                let params @ [boolean, jump_address] = computer.get_parameters(parameter_modes);
                let result = boolean != 0;

                trace!("JumpIfTrue: params={params:?}; result={result}");

                Ok(if result {
                    jump_address as usize
                } else {
                    computer.instruction_pointer + 3
                })
            }
            JumpIfFalse => {
                let params @ [boolean, jump_address] = computer.get_parameters(parameter_modes);
                let result = boolean == 0;

                trace!("JumpIfFalse: params={params:?}; result={result}");

                Ok(if result {
                    jump_address as usize
                } else {
                    computer.instruction_pointer + 3
                })
            }
            LessThan => {
                let params @ [a, b] = computer.get_parameters(parameter_modes);
                let output =
                    computer.read_address(computer.instruction_pointer + 3, &parameter_modes[2]);
                let result = a < b;

                trace!("LessThan: params={params:?}; result={result}; output={output}");

                computer.write(output, if result { 1 } else { 0 });

                Ok(computer.instruction_pointer + 4)
            }
            Equals => {
                let params @ [a, b] = computer.get_parameters(parameter_modes);
                let output =
                    computer.read_address(computer.instruction_pointer + 3, &parameter_modes[2]);
                let result = a == b;

                trace!("Equals: params={params:?}; result={result}; output={output}");

                computer.write(output, if result { 1 } else { 0 });

                Ok(computer.instruction_pointer + 4)
            }
            AdjustRelativeBase => {
                let params @ [base_offset] = computer.get_parameters(parameter_modes);

                computer.relative_base = match base_offset.signum() {
                    -1 => computer
                        .relative_base
                        .sub(base_offset.unsigned_abs() as usize),
                    _ => computer.relative_base.add(base_offset as usize),
                };

                trace!(
                    "AdjustRelativeBase: params={params:?}; result={}",
                    computer.relative_base
                );

                Ok(computer.instruction_pointer + 2)
            }
            Terminate => Err(*self),
        }
    }
}

#[derive(Debug)]
pub enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug)]
struct Instruction {
    op_code: OpCode,
    parameter_modes: Vec<ParameterMode>,
}

trait Digits {
    fn reversed_digits(&self) -> Vec<u8>;
}

impl Digits for i64 {
    fn reversed_digits(&self) -> Vec<u8> {
        if self == &0 {
            vec![0]
        } else {
            let mut copy = self.abs();

            let mut result = vec![];

            while copy != 0 {
                result.push((copy % 10) as u8);
                copy /= 10;
            }

            result
        }
    }
}

impl Instruction {
    fn read(n: i64) -> Self {
        let op_code = match n % 100 {
            1 => OpCode::Addition,
            2 => OpCode::Multiplication,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            9 => OpCode::AdjustRelativeBase,
            99 => OpCode::Terminate,
            invalid => panic!("Invalid opcode {invalid} (instruction: {n}"),
        };

        let mut parameter_modes: Vec<_> = (n / 100)
            .reversed_digits()
            .into_iter()
            .map(|digit| match digit {
                0 => ParameterMode::Position,
                1 => ParameterMode::Immediate,
                2 => ParameterMode::Relative,
                _ => panic!("Unknown parameter mode {digit} (instruction: {n})"),
            })
            .collect();

        parameter_modes.resize_with(op_code.memory_used(), || ParameterMode::Position);

        Self {
            op_code,
            parameter_modes,
        }
    }

    fn evaluate<IO: InputOutput>(&self, computer: &mut Computer, io: &mut IO) -> Option<OpCode> {
        let jump = self.op_code.evaluate(computer, &self.parameter_modes, io);

        if let Ok(jump_address) = jump {
            computer.instruction_pointer = jump_address;
        }

        jump.err()
    }
}

impl Computer {
    pub fn new(memory: Vec<i64>) -> Self {
        Computer {
            memory,
            instruction_pointer: 0,
            relative_base: 0,
        }
    }

    pub fn parse(memory_line: &str) -> Self {
        Self::new(
            memory_line
                .split(',')
                .map(|n| n.parse().expect("Invalid i64"))
                .collect(),
        )
    }

    pub fn run<IO: InputOutput>(&mut self, io: &mut IO) -> OpCode {
        loop {
            if let Some(op_code) = self.step(io) {
                return op_code;
            }
        }
    }

    pub fn step<IO: InputOutput>(&mut self, io: &mut IO) -> Option<OpCode> {
        let instruction = Instruction::read(self.memory[self.instruction_pointer]);

        instruction.evaluate(self, io)
    }

    pub fn read(&self, address: usize, parameter_mode: &ParameterMode) -> i64 {
        let value = *self.memory.get(address).unwrap_or(&0);

        match *parameter_mode {
            ParameterMode::Position => self.read(value as usize, &ParameterMode::Immediate),
            ParameterMode::Immediate => value,
            ParameterMode::Relative => {
                let address = match value.signum() {
                    -1 => self.relative_base.sub(value.unsigned_abs() as usize),
                    _ => self.relative_base.add(value as usize),
                };
                self.read(address, &ParameterMode::Immediate)
            }
        }
    }

    pub fn write(&mut self, address: usize, value: i64) {
        if address >= self.memory.len() {
            self.memory
                .extend(std::iter::repeat_n(0, address - self.memory.len() + 1));
        }
        self.memory[address] = value;
    }

    pub fn memory(&self) -> &[i64] {
        &self.memory
    }

    fn read_address(&self, address: usize, parameter_mode: &ParameterMode) -> usize {
        self.read(address, &ParameterMode::Immediate) as usize
            + match parameter_mode {
                ParameterMode::Relative => self.relative_base,
                _ => 0,
            }
    }

    pub fn diagnostic_code<IO: InputOutput>(&self, io: &IO) -> i64 {
        io.output_iter().find(|n| **n != 0).copied().unwrap_or(0)
    }

    fn get_parameters<const N: usize>(&self, parameter_modes: &[ParameterMode]) -> [i64; N] {
        let mut result = [0i64; N];
        for n in 0..N {
            result[n] = self.read(self.instruction_pointer + n + 1, &parameter_modes[n]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_computer() {
        let results = [
            "1,0,0,0,99",
            "2,3,0,3,99",
            "2,4,4,5,99,0",
            "1,1,1,4,99,5,6,0,99",
        ]
        .into_iter()
        .map(Computer::parse)
        .map(|mut computer| {
            let mut io = NullIO;
            computer.run(&mut io);
            computer.memory().to_owned()
        })
        .collect::<Vec<_>>();

        assert_eq!(
            results,
            vec![
                vec![2, 0, 0, 0, 99],
                vec![2, 3, 0, 6, 99],
                vec![2, 4, 4, 5, 99, 9801],
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            ]
        );
    }

    #[test]
    fn day5_equals_8_position() {
        let computer_equals_8 = Computer::parse("3,9,8,9,10,9,4,9,99,-1,8");

        let mut test_computer = computer_equals_8.clone();
        let mut io = VecDeque::from([8]);
        test_computer.run(&mut io);
        assert_eq!(test_computer.diagnostic_code(&io), 1);

        let mut test_computer = computer_equals_8.clone();
        let mut io = VecDeque::from([1]);
        test_computer.run(&mut io);
        assert_eq!(test_computer.diagnostic_code(&io), 0);
    }

    #[test]
    fn day5_equals_8_immediate() {
        let computer_equals_8 = Computer::parse("3,3,1108,-1,8,3,4,3,99");

        let mut test_computer = computer_equals_8.clone();
        let mut io = VecDeque::from([8]);
        test_computer.run(&mut io);
        assert_eq!(test_computer.diagnostic_code(&io), 1);

        let mut test_computer = computer_equals_8.clone();
        let mut io = VecDeque::from([1]);
        test_computer.run(&mut io);
        assert_eq!(test_computer.diagnostic_code(&io), 0);
    }

    #[test]
    fn day5_less_than_8_position() {
        let computer_less_than_8 = Computer::parse("3,9,7,9,10,9,4,9,99,-1,8");

        let mut test_computer = computer_less_than_8.clone();
        let mut io = VecDeque::from([7]);
        test_computer.run(&mut io);
        assert_eq!(test_computer.diagnostic_code(&io), 1);

        let mut test_computer = computer_less_than_8.clone();
        let mut io = VecDeque::from([8]);
        test_computer.run(&mut io);
        assert_eq!(test_computer.diagnostic_code(&io), 0);

        let mut test_computer = computer_less_than_8.clone();
        let mut io = VecDeque::from([9]);
        test_computer.run(&mut io);
        assert_eq!(test_computer.diagnostic_code(&io), 0);
    }

    #[test]
    fn day5_less_than_8_immediate() {
        let computer_less_than_8 = Computer::parse("3,3,1107,-1,8,3,4,3,99");

        let mut test_computer = computer_less_than_8.clone();
        let mut io = VecDeque::from([7]);
        test_computer.run(&mut io);
        assert_eq!(test_computer.diagnostic_code(&io), 1);

        let mut test_computer = computer_less_than_8.clone();
        let mut io = VecDeque::from([8]);
        test_computer.run(&mut io);
        assert_eq!(test_computer.diagnostic_code(&io), 0);

        let mut test_computer = computer_less_than_8.clone();
        let mut io = VecDeque::from([9]);
        test_computer.run(&mut io);
        assert_eq!(test_computer.diagnostic_code(&io), 0);
    }

    #[test]
    fn day5_large_example() {
        let computer = Computer::parse(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        );

        for (input, expected) in [(7, 999), (8, 1000), (9, 1001)] {
            let mut test_computer = computer.clone();
            let mut io = VecDeque::from([input]);
            test_computer.run(&mut io);
            assert_eq!(test_computer.diagnostic_code(&io), expected);
        }
    }

    #[test]
    fn day9_quine() {
        let program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut computer = Computer::parse(program);
        let mut io = VecDeque::new();

        computer.run(&mut io);

        assert_eq!(
            io,
            program
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn day9_large_number_output() {
        let program = "1102,34915192,34915192,7,4,7,99,0";
        let mut computer = Computer::parse(program);
        let mut io = VecDeque::new();

        computer.run(&mut io);

        assert_eq!(io[0].reversed_digits().len(), 16);
    }

    #[test]
    fn day9_large_number_from_memory() {
        let program = "104,1125899906842624,99";
        let mut computer = Computer::parse(program);
        let mut io = VecDeque::new();

        computer.run(&mut io);

        assert_eq!(io[0], 1125899906842624);
    }

    #[test]
    fn test_reversed_digits() {
        assert_eq!(25.reversed_digits(), [5, 2]);
        assert_eq!(1234.reversed_digits(), [4, 3, 2, 1]);
        assert_eq!((-1234i64).reversed_digits(), [4, 3, 2, 1]);
    }
}

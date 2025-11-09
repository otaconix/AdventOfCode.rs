use std::collections::VecDeque;

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

#[derive(Debug, Clone)]
pub struct Computer {
    memory: Vec<i64>,
    instruction_pointer: usize,
}

#[derive(Debug, Clone, Copy)]
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
}

impl OpCode {
    fn memory_used(&self) -> usize {
        match *self {
            OpCode::Addition => 4,
            OpCode::Multiplication => 4,
            OpCode::Input => 2,
            OpCode::Output => 3,
            OpCode::JumpIfTrue => 4,
            OpCode::JumpIfFalse => 4,
            OpCode::LessThan => 4,
            OpCode::Equals => 4,
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
            "IP: {}, opcode: {self:?}, param_modes: {parameter_modes:?}",
            computer.instruction_pointer
        );

        trace!(
            "Memory: {:#?}",
            &computer.memory()
                [computer.instruction_pointer..computer.instruction_pointer + self.memory_used()]
        );

        match *self {
            Addition => {
                let params: [i64; 2] = computer.get_parameters(parameter_modes);
                let output = computer.read_address(computer.instruction_pointer + 3);
                let result = params.iter().sum();

                trace!("Addition: params={params:?}; output={output}; result={result}");

                computer.write(output, result);

                Ok(computer.instruction_pointer + 4)
            }
            Multiplication => {
                let params: [i64; 2] = computer.get_parameters(parameter_modes);
                let output = computer.read_address(computer.instruction_pointer + 3);
                let result = params.iter().product();

                trace!("Multiplication: params={params:?}; output={output}; result={result}");

                computer.write(output, result);

                Ok(computer.instruction_pointer + 4)
            }
            Input => {
                let output = computer.read_address(computer.instruction_pointer + 1);
                let result = computer.pop(io);

                trace!("Input: output={output}; result={result:?}");

                if let Some(result) = result {
                    computer.write(output, result);

                    Ok(computer.instruction_pointer + 2)
                } else {
                    Err(*self)
                }
            }
            Output => {
                let params: [i64; 1] = computer.get_parameters(parameter_modes);

                trace!("Output: params={params:?}");

                computer.push(params[0], io);

                Ok(computer.instruction_pointer + 2)
            }
            JumpIfTrue => {
                let params: [i64; 2] = computer.get_parameters(parameter_modes);

                Ok(if params[0] != 0 {
                    params[1] as usize
                } else {
                    computer.instruction_pointer + 3
                })
            }
            JumpIfFalse => {
                let params: [i64; 2] = computer.get_parameters(parameter_modes);

                Ok(if params[0] == 0 {
                    params[1] as usize
                } else {
                    computer.instruction_pointer + 3
                })
            }
            LessThan => {
                let params: [i64; 2] = computer.get_parameters(parameter_modes);
                let output = computer.read_address(computer.instruction_pointer + 3);

                if params[0] < params[1] {
                    computer.write(output, 1);
                } else {
                    computer.write(output, 0);
                }

                Ok(computer.instruction_pointer + 4)
            }
            Equals => {
                let params: [i64; 2] = computer.get_parameters(parameter_modes);
                let output = computer.read_address(computer.instruction_pointer + 3);

                if params[0] == params[1] {
                    computer.write(output, 1);
                } else {
                    computer.write(output, 0);
                }

                Ok(computer.instruction_pointer + 4)
            }
            Terminate => Err(*self),
        }
    }
}

#[derive(Debug)]
pub enum ParameterMode {
    Position,
    Immediate,
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
            99 => OpCode::Terminate,
            invalid => panic!("Invalid opcode {invalid} (instruction: {n}"),
        };

        let parameter_modes = (n / 100)
            .reversed_digits()
            .into_iter()
            .map(|digit| match digit {
                0 => ParameterMode::Position,
                1 => ParameterMode::Immediate,
                _ => panic!("Unknown parameter mode {digit} (instruction: {n})"),
            })
            .collect();

        Self {
            op_code,
            parameter_modes,
        }
    }

    fn evaluate<IO: InputOutput>(&self, computer: &mut Computer, io: &mut IO) -> Option<OpCode> {
        let jump = self.op_code.evaluate(computer, &self.parameter_modes, io);

        computer.instruction_pointer = jump.unwrap_or(0);

        jump.err()
    }
}

impl Computer {
    pub fn new(memory: Vec<i64>) -> Self {
        Computer {
            memory,
            instruction_pointer: 0,
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
            let instruction = Instruction::read(self.memory[self.instruction_pointer]);

            if let Some(op_code) = instruction.evaluate(self, io) {
                return op_code;
            }
        }
    }

    pub fn offset_read(&self, offset: usize, parameter_mode: &ParameterMode) -> i64 {
        self.read(self.instruction_pointer + offset, parameter_mode)
    }

    pub fn read(&self, address: usize, parameter_mode: &ParameterMode) -> i64 {
        match *parameter_mode {
            ParameterMode::Immediate => self.memory[address],
            ParameterMode::Position => self.memory[self.memory[address] as usize],
        }
    }

    pub fn write(&mut self, address: usize, value: i64) {
        self.memory[address] = value;
    }

    pub fn memory(&self) -> &[i64] {
        &self.memory
    }

    pub fn push<IO: InputOutput>(&mut self, value: i64, io: &mut IO) {
        io.push(value);
    }

    pub fn pop<IO: InputOutput>(&mut self, io: &mut IO) -> Option<i64> {
        io.pop()
    }

    fn read_address(&self, address: usize) -> usize {
        self.read(address, &ParameterMode::Immediate) as usize
    }

    pub fn diagnostic_code<IO: InputOutput>(&self, io: &IO) -> i64 {
        io.output_iter().find(|n| **n != 0).copied().unwrap_or(0)
    }

    fn get_parameters<const N: usize>(&self, parameter_modes: &[ParameterMode]) -> [i64; N] {
        let mut result = [0i64; N];
        for n in 0..N {
            result[n] = self.offset_read(
                n + 1,
                parameter_modes.get(n).unwrap_or(&ParameterMode::Position),
            );
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
    fn test_reversed_digits() {
        assert_eq!(25.reversed_digits(), [5, 2]);
        assert_eq!(1234.reversed_digits(), [4, 3, 2, 1]);
        assert_eq!((-1234i64).reversed_digits(), [4, 3, 2, 1]);
    }
}

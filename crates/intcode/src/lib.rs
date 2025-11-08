#[derive(Clone)]
pub struct Computer {
    memory: Vec<u64>,
    instruction_pointer: usize,
}

impl Computer {
    pub fn new(memory: Vec<u64>) -> Self {
        Computer {
            memory,
            instruction_pointer: 0,
        }
    }

    pub fn parse(memory_line: &str) -> Self {
        Self::new(
            memory_line
                .split(',')
                .map(|n| n.parse().expect("Invalid u64"))
                .collect(),
        )
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.memory[self.instruction_pointer];

            self.instruction_pointer = match instruction {
                99 => break,
                1 => self.addition(),
                2 => self.multiplication(),
                _ => panic!("Unknown opcode {instruction}"),
            }
        }
    }

    fn addition(&mut self) -> usize {
        let a = self.indirect_read(self.instruction_pointer + 1);
        let b = self.indirect_read(self.instruction_pointer + 2);
        let result_address = self.read_address(self.instruction_pointer + 3);

        self.memory[result_address] = a + b;

        self.instruction_pointer + 4
    }

    fn multiplication(&mut self) -> usize {
        let a = self.indirect_read(self.instruction_pointer + 1);
        let b = self.indirect_read(self.instruction_pointer + 2);
        let result_address = self.read_address(self.instruction_pointer + 3);

        self.memory[result_address] = a * b;

        self.instruction_pointer + 4
    }

    pub fn read(&self, address: usize) -> u64 {
        self.memory[address]
    }

    pub fn write(&mut self, address: usize, value: u64) {
        self.memory[address] = value;
    }

    pub fn memory(&self) -> &[u64] {
        &self.memory
    }

    #[inline]
    fn read_address(&self, address: usize) -> usize {
        self.read(address) as usize
    }

    fn indirect_read(&self, address: usize) -> u64 {
        self.memory[self.read_address(address)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 4] = [
        "1,0,0,0,99",
        "2,3,0,3,99",
        "2,4,4,5,99,0",
        "1,1,1,4,99,5,6,0,99",
    ];

    #[test]
    fn test_run_computer() {
        let results = INPUT
            .into_iter()
            .map(Computer::parse)
            .map(|mut input| {
                input.run();
                input.memory().to_owned()
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
}

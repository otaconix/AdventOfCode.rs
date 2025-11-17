use std::collections::VecDeque;
use std::io;

use aoc_timing::trace::log_run;
use intcode::Computer;
use intcode::OpCode;
use intcode::SplitIO;

type Input = Computer;
type Output1 = i64;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(mut input: I) -> Input {
    let line = input.next().unwrap();

    Computer::parse(line.as_ref())
}

fn spring_droid(computer: &Computer, program: &[&str]) -> i64 {
    let mut computer = computer.clone();
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();

    let mut program = program.join("\n");
    program.push('\n');
    program
        .bytes()
        .for_each(|program_byte| input.push_back(program_byte as i64));

    assert_eq!(
        computer.run(&mut SplitIO::new(&mut input, &mut output)),
        OpCode::Terminate
    );

    let result = output.pop_back().expect("No hull damage output.");

    // If the last output was in ASCII range, then we ran into an error.
    if result <= 127 {
        println!(
            "{}",
            String::from_utf8(output.iter().map(|&i| i as u8).collect())
                .expect("Invalid UTF-8 from SpringDroid.")
        );
    }

    result
}

fn part_1(input: &Input) -> Output1 {
    spring_droid(
        input,
        &[
            "NOT A J", "NOT B T", "AND D T", "OR T J", "NOT C T", "AND D T", "OR T J", "WALK",
        ],
    )
}

fn part_2(input: &Input) -> Output2 {
    spring_droid(
        input,
        &[
            "NOT C J", "AND H J", "NOT B T", "OR T J", "NOT A T", "OR T J", "AND D J", "RUN",
        ],
    )
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

use std::io;
use std::iter::once;
use std::str::FromStr;

enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = s.split_ascii_whitespace().collect();

        match words.first().ok_or("Empty line")? {
            &"noop" => Ok(Instruction::Noop),
            &"addx" => words
                .get(1)
                .ok_or("No operand for addx".to_string())
                .and_then(|operand| {
                    operand
                        .parse()
                        .map_err(|e| format!("Can't parse operand {operand}: {e}"))
                        .map(|operand| Instruction::AddX(operand))
                }),
            _ => Err(format!("Unknown instruction: {s}")),
        }
    }
}

fn main() {
    let instructions: Vec<Instruction> = once(Instruction::Noop)
        .chain(
            io::stdin()
                .lines()
                .map(|result| result.expect("I/O error"))
                .map(|line| line.parse().unwrap()),
        )
        .collect();

    let states: Vec<_> = (1..)
        .zip(
            instructions
                .iter()
                .flat_map(|instruction| match instruction {
                    Instruction::Noop => vec![instruction],
                    Instruction::AddX(_) => vec![&Instruction::Noop, instruction],
                })
                .scan(1i32, |x, instruction| {
                    match instruction {
                        Instruction::Noop => {}
                        Instruction::AddX(operand) => {
                            *x += operand;
                        }
                    };

                    Some(x.clone())
                }),
        )
        .collect();

    let part_1: i32 = states
        .iter()
        .take(220)
        .filter(|(i, _)| *i >= 20 && (i - 20) % 40 == 0)
        .map(|(i, x)| *i as i32 * x)
        .sum();

    println!("Part 1: {part_1}");

    states
        .iter()
        .map(|(n, x)| {
            if x.abs_diff((n - 1) % 40) <= 1 {
                '#'
            } else {
                '.'
            }
        })
        .collect::<Vec<_>>()
        .chunks(40)
        .map(|chunk| chunk.into_iter().collect::<String>())
        .for_each(|line| println!("{line}"));
}

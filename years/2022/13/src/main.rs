use aoc_timing::trace::log_run;
use std::{cmp::Ordering, io};
use types::Input;

mod types {
    use pom::char_class::digit;
    use pom::parser::{Parser, call, sym, is_a};
    use std::str::FromStr;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Input {
        List(Vec<Input>),
        Number(u32),
    }

    impl FromStr for Input {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            list().parse(s.as_bytes()).map_err(|e| format!("{e}"))
        }
    }

    impl PartialOrd for Input {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Input {
        fn to_list(&self) -> Self {
            match *self {
                Input::List(_) => self.clone(),
                Input::Number(_) => Input::List(vec![self.clone()]),
            }
        }
    }

    impl Ord for Input {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            use Input::{List, Number};

            match (self, other) {
                (List(xs), List(ys)) => xs.cmp(ys),
                (List(_), Number(_)) => self.cmp(&other.to_list()),
                (Number(_), List(_)) => self.to_list().cmp(other),
                (Number(x), Number(y)) => x.cmp(y),
            }
        }
    }

    fn list<'a>() -> Parser<'a, u8, Input> {
        let element = call(list) | number();

        sym(b'[') * pom::parser::list(element, sym(b',')).map(Input::List) - sym(b']')
    }

    fn number<'a>() -> Parser<'a, u8, Input> {
        is_a(digit).repeat(1..).map(|digits| {
            Input::Number(
                digits
                    .into_iter()
                    .fold(0u32, |result, digit| result * 10 + u32::from(digit)),
            )
        })
    }
}

fn main() {
    env_logger::init();

    let inputs: Vec<_> = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse::<Input>().unwrap())
        .collect::<Vec<_>>();

    let part_1: usize = log_run("Part 1", || {
        (1..)
            .zip(
                inputs
                    .chunks(2)
                    .map(|pair| (pair[0].clone(), pair[1].clone())),
            )
            .filter(|(_, (a, b))| Ordering::is_le(a.cmp(b)))
            .map(|(index, _)| index)
            .sum()
    });
    println!("Part 1: {part_1}");

    let part_2: usize = log_run("Part 2", || {
        let divider_packets: [Input; 2] = ["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];
        let mut part_2 = divider_packets
            .iter()
            .chain(inputs.iter())
            .collect::<Vec<_>>();
        part_2.sort();

        (1..)
            .zip(part_2.iter())
            .filter(|(_, input)| divider_packets.contains(input))
            .map(|(index, _)| index)
            .product()
    });
    println!("Part 2: {part_2}");
}

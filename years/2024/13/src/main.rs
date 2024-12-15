use std::borrow::Borrow;
use std::io;

use aoc_timing::trace::log_run;

#[derive(Debug, Clone, Copy)]
struct PrizeLocation {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Button {
    delta_x: isize,
    delta_y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize_location: PrizeLocation,
}

type Input = Vec<Machine>;
type Output = isize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    enum ParseState {
        Empty,
        ButtonA(Button),
        ButtonB(Button, Button),
    }

    use ParseState::*;

    input
        .fold(
            (Vec::new(), ParseState::Empty),
            |(mut result, state), line| {
                let line = line.as_ref();

                if line.is_empty() {
                    (result, state)
                } else {
                    let (_, right) = line.split_once(':').unwrap();
                    let (x, y) = right.split_once(',').unwrap();
                    let split_char = match state {
                        Empty | ButtonA(_) => '+',
                        ButtonB(_, _) => '=',
                    };
                    let x = x.split_once(split_char).unwrap().1.trim().parse().unwrap();
                    let y = y.split_once(split_char).unwrap().1.trim().parse().unwrap();

                    match state {
                        Empty => (
                            result,
                            ButtonA(Button {
                                delta_x: x,
                                delta_y: y,
                            }),
                        ),
                        ButtonA(button_a) => (
                            result,
                            ButtonB(
                                button_a,
                                Button {
                                    delta_x: x,
                                    delta_y: y,
                                },
                            ),
                        ),
                        ButtonB(button_a, button_b) => {
                            result.push(Machine {
                                button_a,
                                button_b,
                                prize_location: PrizeLocation { x, y },
                            });

                            (result, Empty)
                        }
                    }
                }
            },
        )
        .0
}

/// We're trying to solve a system of two linear equations here.
///
/// To reach the prize location, we need the press button A `a` times, and button B `b` times.
/// So, we're left with the following two equations:
/// ```
/// button_a_delta_x*a + button_b_delta_x*b = prize_location_x
/// button_a_delta_y*a + button_b_delta_y*b = prize_location_y
/// ```
///
/// We can isolate `a` in the first equation like to:
/// ```
/// a = (prize_location_x - button_b_delta_x*b) / button_a_delta_x
/// ```
///
/// Then substitute that into the second equation:
/// ```
/// button_a_delta_y*((prize_location_x - button_b_delta_x*b) / button_a_delta_x) + button_b_delta_y*b = prize_location_y
/// ```
///
/// If this has an integral solution, plug that back into the first equation.
///
/// Finally, if _that_ has an integral solution, the machine is "solvable", and we're done.
fn solve_machine<M>(machine: M) -> Option<(isize, isize)>
where
    M: Borrow<Machine>, // Don't really care whether I take ownership or not
{
    let machine: &Machine = machine.borrow();

    let b_multiplier = machine.button_b.delta_y * machine.button_a.delta_x
        - machine.button_a.delta_y * machine.button_b.delta_x;
    let cumulative_distance_b = machine.prize_location.y * machine.button_a.delta_x
        - machine.prize_location.x * machine.button_a.delta_y;

    if cumulative_distance_b % b_multiplier == 0 {
        let b_presses = cumulative_distance_b / b_multiplier;

        let numerator = machine.prize_location.x - machine.button_b.delta_x * b_presses;
        let denominator = machine.button_a.delta_x;

        if numerator % denominator == 0 {
            return Some((numerator / denominator, b_presses));
        }
    }

    None
}

fn part_1(input: &Input) -> Output {
    input
        .iter()
        .flat_map(solve_machine)
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn part_2(input: &Input) -> Output {
    input
        .iter()
        .map(|machine| Machine {
            prize_location: PrizeLocation {
                x: machine.prize_location.x + 10000000000000,
                y: machine.prize_location.y + 10000000000000,
            },
            ..*machine
        })
        .flat_map(solve_machine)
        .map(|(a, b)| a * 3 + b)
        .sum()
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

        assert_eq!(result, 480);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 875318608908);
    }
}

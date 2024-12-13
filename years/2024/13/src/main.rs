use std::io;
use std::iter::successors;

use aoc_timing::trace::log_run;

#[derive(Debug)]
struct PrizeLocation {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Button {
    delta_x: usize,
    delta_y: usize,
}

#[derive(Debug)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize_location: PrizeLocation,
}

type Input = Vec<Machine>;
type Output = usize;

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

fn solve_machine(machine: &Machine) -> Option<usize> {
    successors(Some((0, (0, 0))), |(cost, (x, y))| {
        Some((
            cost + 3,
            (x + machine.button_a.delta_x, y + machine.button_a.delta_y),
        ))
    })
    .take_while(|(_, (x, y))| *x <= machine.prize_location.x && *y <= machine.prize_location.y)
    .flat_map(|(cost, (x, y))| {
        let x_distance = machine.prize_location.x - x;
        let y_distance = machine.prize_location.y - y;

        let (bx, rest_x) = (
            x_distance / machine.button_b.delta_x,
            x_distance % machine.button_b.delta_x,
        );
        let (by, rest_y) = (
            y_distance / machine.button_b.delta_y,
            y_distance % machine.button_b.delta_y,
        );

        if rest_x == 0 && rest_y == 0 && bx == by {
            Some(cost + bx)
        } else {
            None
        }
    })
    .min()
}

fn part_1(input: &Input) -> Output {
    input.iter().flat_map(solve_machine).sum()
}

fn part_2(input: &Input) -> Output {
    todo!()
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

        assert_eq!(result, 0);
    }
}

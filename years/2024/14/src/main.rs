use std::io;

use aoc_timing::trace::log_run;

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

#[derive(Clone, Copy)]
struct Robot {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl Robot {
    fn move_for(self, duration: isize, width: isize, height: isize) -> Self {
        Self {
            x: (self.x + self.vx * duration).rem_euclid(width),
            y: (self.y + self.vy * duration).rem_euclid(height),
            ..self
        }
    }

    fn move_once(self) -> Self {
        self.move_for(1, WIDTH, HEIGHT)
    }
}

type Input = Vec<Robot>;
type Output = usize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();

            let (position, velocity) = line.split_once(' ').unwrap();
            let (x, y) = position.split_once('=').unwrap().1.split_once(',').unwrap();
            let (vx, vy) = velocity.split_once('=').unwrap().1.split_once(',').unwrap();

            Robot {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                vx: vx.parse().unwrap(),
                vy: vy.parse().unwrap(),
            }
        })
        .collect()
}

fn part_1(input: &Input, width: isize, height: isize) -> Output {
    let (top, bottom): (Vec<_>, Vec<_>) = input
        .iter()
        .copied()
        .map(|robot| robot.move_for(100, width, height))
        .map(|Robot { x, y, .. }| (x, y))
        .filter(|(x, y)| *x != width / 2 && *y != height / 2)
        .partition(|(_, y)| *y < height / 2);

    let (top_left, top_right): (Vec<_>, Vec<_>) =
        top.into_iter().partition(|(x, _)| *x < width / 2);
    let (bottom_left, bottom_right): (Vec<_>, Vec<_>) =
        bottom.into_iter().partition(|(x, _)| *x < width / 2);

    top_left.len() * top_right.len() * bottom_left.len() * bottom_right.len()
}

fn print_image(input: &Input) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if input.iter().any(|robot| robot.x == x && robot.y == y) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn part_2(input: &Input) -> Output {
    let mut input = input.clone();

    for seconds in 0.. {
        // Silly heuristics that happen to work for my input
        if (0..WIDTH).any(|x| {
            input.iter().filter(|Robot { x: rx, .. }| *rx == x).count() as isize >= HEIGHT / 3
        }) && (0..HEIGHT).any(|y| {
            input.iter().filter(|Robot { y: ry, .. }| *ry == y).count() as isize >= WIDTH / 4
        }) {
            print_image(&input);
            return seconds as usize;
        }

        input = input.into_iter().map(Robot::move_once).collect();
    }

    panic!()
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        let part_1 = log_run("Part 1", || part_1(&input, WIDTH, HEIGHT));
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
        let result = part_1(&input, 11, 7);

        assert_eq!(result, 12);
    }
}

use std::io;

use aoc_timing::trace::log_run;
use grid::Grid;
use itertools::Itertools;

type Layer = Grid<u8>;

struct Image {
    layers: Vec<Layer>,
}

type Input = Image;
type Output1 = usize;
type Output2 = String;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(mut input: I, width: usize, height: usize) -> Input {
    let line = input.next().unwrap();
    Image {
        layers: line
            .as_ref()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .chunks(width * height)
            .into_iter()
            .map(|layer_iter| {
                layer_iter
                    .chunks(width)
                    .into_iter()
                    .map(|line| line.collect())
                    .collect()
            })
            .collect(),
    }
}

fn part_1(input: &Input) -> Output1 {
    let fewest_zeroes_layer = input
        .layers
        .iter()
        .min_by_key(|layer| {
            layer
                .coordinates()
                .map(|coord| layer.get_coord(coord).unwrap())
                .filter(|pixel| **pixel == 0)
                .count()
        })
        .unwrap();

    let pixel_counts = fewest_zeroes_layer
        .coordinates()
        .map(|coord| fewest_zeroes_layer.get_coord(coord).unwrap())
        .into_grouping_map_by(|x| **x)
        .fold(0usize, |acc, _, _| acc + 1);

    pixel_counts[&1] * pixel_counts[&2]
}

fn part_2(input: &Input) -> Output2 {
    let final_image: Grid<u8> = input.layers[0]
        .coordinates()
        .map(|coord| {
            input
                .layers
                .iter()
                .map(|layer| layer.get_coord(coord).unwrap())
                .find(|color| **color != 2)
                .unwrap_or(&2)
        })
        .copied()
        .chunks(input.layers[0].width())
        .into_iter()
        .map(|row| row.collect())
        .collect();

    let mut result = String::new();

    println!(
        "final image: width={}; height={}",
        final_image.width(),
        final_image.height()
    );

    for row in 0..final_image.height() {
        for column in 0..final_image.width() {
            result.push(match final_image.get(column, row).unwrap() {
                1 => '#',
                _ => ' ',
            });
        }

        if row != final_image.height() - 1 {
            result.push('\n');
        }
    }

    result
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(
                io::stdin().lines().map(|result| result.expect("I/O error")),
                25,
                6,
            )
        });

        let part_1 = log_run("Part 1", || part_1(&input));
        println!("Part 1: {part_1}");

        let part_2 = log_run("Part 2", || part_2(&input));
        println!("Part 2:\n{part_2}");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = parse("123456789012".lines(), 3, 2);

        assert_eq!(input.layers.len(), 2);

        let result = part_1(&input);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_2() {
        let input = parse("0222112222120000".lines(), 2, 2);
        let result = part_2(&input);

        assert_eq!(result, " #\n# ");
    }
}

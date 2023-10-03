use std::collections::HashSet;
use std::io;

type Coords = (i32, i32, i32);

fn neighbors(coords: &Coords) -> [Coords; 6] {
    [
        (coords.0 - 1, coords.1, coords.2),
        (coords.0 + 1, coords.1, coords.2),
        (coords.0, coords.1 - 1, coords.2),
        (coords.0, coords.1 + 1, coords.2),
        (coords.0, coords.1, coords.2 - 1),
        (coords.0, coords.1, coords.2 + 1),
    ]
}

fn main() {
    let input: HashSet<_> = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| {
            let nums = line
                .split(',')
                .map(|num| num.parse::<i32>().expect("Not a number?"))
                .collect::<Vec<_>>();

            (nums[0], nums[1], nums[2])
        })
        .collect();

    let part1 = input
        .iter()
        .flat_map(|c| {
            neighbors(c)
                .into_iter()
                .filter(|neighbor| !input.contains(neighbor))
        })
        .count();

    println!("Part 1: {part1}");
}

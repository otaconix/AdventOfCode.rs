use std::io;

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Vec<String> {
    input
        .map(|line| line.to_string())
        .flat_map(|line| {
            line.split(',')
                .map(|step| step.to_owned())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn hash(step: &str) -> u8 {
    step.chars()
        .map(|c| c as u8)
        .fold(0, |hash, ascii| (hash.wrapping_add(ascii)).wrapping_mul(17))
}

fn part_1(input: &[String]) -> usize {
    input.iter().map(|step| hash(step) as usize).sum::<usize>()
}

fn main() {
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = part_1(&input);
    println!("Part 1: {part_1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 1320);
    }
}

use std::io;

#[derive(Debug)]
enum InputLine {
    Calories(u32),
    Separator,
}

fn main() {
    let mut input = io::stdin()
        .lines()
        .map(|result| result.expect("IO error"))
        .map(|line| {
            line.parse::<u32>()
                .map(InputLine::Calories)
                .unwrap_or(InputLine::Separator)
        })
        .fold(
            (Vec::new(), Vec::new()),
            |(mut result, mut elf), input_line| match input_line {
                InputLine::Calories(n) => (result, {
                    elf.push(n);
                    elf
                }),
                InputLine::Separator => (
                    {
                        result.push(elf);
                        result
                    },
                    vec![],
                ),
            },
        );

    let input = if !input.1.is_empty() {
        input.0.push(input.1);
        input.0
    } else {
        input.0
    };

    let silver: u32 = input
        .iter()
        .map(|elf| elf.iter().sum())
        .max()
        .expect("Was there no input?");

    println!("Silver: {}", silver);

    let mut calories_per_elf: Vec<u32> = input.iter().map(|elf| elf.iter().sum()).collect();
    calories_per_elf.sort_by(|a, b| a.cmp(b).reverse());

    let gold: u32 = calories_per_elf.iter().take(3).sum();

    println!("Gold: {}", gold);
}

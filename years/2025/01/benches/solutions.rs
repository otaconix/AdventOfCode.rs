use divan::Bencher;

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("input");

#[divan::bench(sample_count = 10_000)]
fn parse(bencher: Bencher) {
    bencher.bench_local(move || {
        aoc_2025_01::parse(INPUT.lines());
    });
}

#[divan::bench(sample_count = 10_000)]
fn part_1(bencher: Bencher) {
    let input = aoc_2025_01::parse(INPUT.lines());

    bencher.bench_local(move || {
        aoc_2025_01::part_1(&input);
    });
}

#[divan::bench(sample_count = 1000)]
fn part_2(bencher: Bencher) {
    let input = aoc_2025_01::parse(INPUT.lines());

    bencher.bench_local(move || {
        aoc_2025_01::part_2(&input);
    });
}

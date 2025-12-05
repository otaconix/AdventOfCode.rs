use divan::Bencher;

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("input");

#[divan::bench(sample_count = 10_000)]
fn aoc_2025_02_parse(bencher: Bencher) {
    bencher.bench_local(move || {
        aoc_2025_02::parse(INPUT.lines());
    });
}

#[divan::bench(sample_count = 10_000)]
fn aoc_2025_02_part_1(bencher: Bencher) {
    let input = aoc_2025_02::parse(INPUT.lines());

    bencher.bench_local(move || {
        aoc_2025_02::part_1(&input);
    });
}

#[divan::bench(sample_count = 100)]
fn aoc_2025_02_part_2(bencher: Bencher) {
    let input = aoc_2025_02::parse(INPUT.lines());

    bencher.bench_local(move || {
        aoc_2025_02::part_2(&input);
    });
}

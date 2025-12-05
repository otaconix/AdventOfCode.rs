{% assign current_year = "now" | date: "%Y" -%}
{% assign today = "now" | date: "%d" -%}
{% capture crate %}aoc_{{ year | default: current_year }}_{{ day | default: today }}{% endcapture -%}
use divan::Bencher;

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../src/test-input");

#[divan::bench(sample_count = 10_000)]
fn {{ crate }}_parse(bencher: Bencher) {
    bencher.bench_local(move || {
        {{ crate }}::parse(INPUT.lines());
    });
}

#[divan::bench(sample_count = 10_000)]
fn {{ crate }}_part_1(bencher: Bencher) {
    let input = {{ crate }}::parse(INPUT.lines());

    bencher.bench_local(move || {
        {{ crate }}::part_1(&input);
    });
}

#[divan::bench(sample_count = 10_000)]
fn {{ crate }}_part_2(bencher: Bencher) {
    let input = {{ crate }}::parse(INPUT.lines());

    bencher.bench_local(move || {
        {{ crate }}::part_2(&input);
    });
}


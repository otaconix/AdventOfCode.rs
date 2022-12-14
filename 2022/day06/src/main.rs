use std::io;

fn find_first_string_of_unique_characters(s: &str, length: usize) -> Option<usize> {
    s.chars()
        .collect::<Vec<_>>()
        .windows(length)
        .enumerate()
        .find(|(_, window)| {
            let mut deduped = window.iter().collect::<Vec<_>>();
            deduped.sort();
            deduped.dedup();

            deduped.len() == length
        })
        .map(|p| p.0)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("I/O error");

    let part_1 = find_first_string_of_unique_characters(&input, 4)
        .expect("Didn't find window of 4 unique characters.")
        + 4;
    let part_2 = find_first_string_of_unique_characters(&input, 14)
        .expect("Didn't find window of 14 unique characters.")
        + 14;

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

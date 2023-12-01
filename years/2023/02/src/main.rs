use std::io;

fn main() {
    let input = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .collect::<Vec<_>>();

    println!("{input:#?}");
}

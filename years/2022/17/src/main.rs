use std::io;

use coord::Coordinate2D;

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

fn main() {
    let shapes: Vec<Vec<Coordinate2D>> = vec![
        vec![
            Coordinate2D::new(0, 0),
            Coordinate2D::new(1, 0),
            Coordinate2D::new(2, 0),
            Coordinate2D::new(3, 0),
        ],
        vec![
            Coordinate2D::new(1, 0),
            Coordinate2D::new(0, 1),
            Coordinate2D::new(1, 1),
            Coordinate2D::new(2, 1),
            Coordinate2D::new(1, 2),
        ],
        vec![
            Coordinate2D::new(2, 0),
            Coordinate2D::new(2, 1),
            Coordinate2D::new(2, 2),
            Coordinate2D::new(1, 2),
            Coordinate2D::new(0, 2),
        ],
        vec![
            Coordinate2D::new(0, 0),
            Coordinate2D::new(0, 1),
            Coordinate2D::new(0, 2),
            Coordinate2D::new(0, 3),
        ],
        vec![
            Coordinate2D::new(0, 0),
            Coordinate2D::new(1, 0),
            Coordinate2D::new(0, 1),
            Coordinate2D::new(1, 1),
        ],
    ];

    let input: Vec<_> = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .flat_map(|line| {
            line.chars()
                .map(|c| match c {
                    '<' => Jet::Left,
                    '>' => Jet::Right,
                    _ => panic!("Unexpected jet character: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    //println!("Input (length: {}): {input:#?}", input.len());
    println!("Input length: {}", input.len());
}

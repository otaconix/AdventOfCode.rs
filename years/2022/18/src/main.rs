use std::collections::HashSet;
use std::io;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct LavaBit(i8, i8, i8);

impl LavaBit {
    fn neighbors(&self) -> [Self; 6] {
        [
            Self(self.0 - 1, self.1, self.2),
            Self(self.0 + 1, self.1, self.2),
            Self(self.0, self.1 - 1, self.2),
            Self(self.0, self.1 + 1, self.2),
            Self(self.0, self.1, self.2 - 1),
            Self(self.0, self.1, self.2 + 1),
        ]
    }
}

/// This is basically an implementation of a flood fill algorithm.
///
/// By starting outside the droplet (assuming -1,-1,-1 is outside), we flood fill
/// the 3D space, stopping at lava bits in the droplet.
fn exterior(lava_bits: &HashSet<LavaBit>) -> HashSet<LavaBit> {
    let x_range = -1..=lava_bits.iter().map(|lava_bit| lava_bit.0).max().unwrap() + 1;
    let y_range = -1..=lava_bits.iter().map(|lava_bit| lava_bit.1).max().unwrap() + 1;
    let z_range = -1..=lava_bits.iter().map(|lava_bit| lava_bit.2).max().unwrap() + 1;

    let mut flooded = HashSet::new();
    let mut queue = vec![LavaBit(-1, -1, -1)];

    while let Some(n @ LavaBit(x, y, z)) = queue.pop() {
        if x_range.contains(&x)
            && y_range.contains(&y)
            && z_range.contains(&z)
            && !flooded.contains(&n)
            && !lava_bits.contains(&n)
        {
            queue.extend(n.neighbors());
            flooded.insert(n);
        }
    }

    flooded
}

fn main() {
    let input: HashSet<_> = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| {
            let nums = line
                .split(',')
                .map(|num| num.parse::<i8>().expect("Not a number?"))
                .collect::<Vec<_>>();

            LavaBit(nums[0], nums[1], nums[2])
        })
        .collect();

    let part1 = input
        .iter()
        .flat_map(|lava_bit| {
            lava_bit
                .neighbors()
                .into_iter()
                .filter(|neighbor| !input.contains(neighbor))
        })
        .count();

    println!("Part 1: {part1}");

    let exterior = exterior(&input);
    let part2 = input
        .iter()
        .flat_map(|lava_bit| {
            lava_bit
                .neighbors()
                .into_iter()
                .filter(|neighbor| exterior.contains(neighbor))
        })
        .count();

    println!("Part 2: {part2}");
}

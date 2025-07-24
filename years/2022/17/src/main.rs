mod solution {
    use std::collections::HashMap;
    use std::fmt::{Debug, Display};

    #[derive(Debug, Clone, Copy)]
    pub enum Jet {
        Left,
        Right,
    }

    impl TryFrom<char> for Jet {
        type Error = String;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '<' => Ok(Jet::Left),
                '>' => Ok(Jet::Right),
                _ => Err(format!("Invalid jet character: {value}")),
            }
        }
    }

    pub struct JetIterator<'a> {
        jets: &'a Vec<Jet>,
        index: usize,
    }

    impl<'a> JetIterator<'a> {
        pub fn new(jets: &'a Vec<Jet>) -> Self {
            JetIterator { jets, index: 0 }
        }
    }

    impl Iterator for JetIterator<'_> {
        type Item = Jet;

        fn next(&mut self) -> Option<Self::Item> {
            let result = self.jets[self.index];
            self.index = (self.index + 1) % self.jets.len();

            Some(result)
        }
    }

    const LEFT_WALL: u32 = 0x40404040;
    const RIGHT_WALL: u32 = 0x01010101;

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Rock(u32);

    impl Rock {
        fn all() -> [Self; 5] {
            [
                // 00011110
                // 00000000
                // 00000000
                // 00000000
                Rock(0x0000001E),
                // 00001000
                // 00011100
                // 00001000
                // 00000000
                Rock(0x00081C08),
                // 00000100
                // 00000100
                // 00011100
                // 00000000
                Rock(0x0004041C),
                // 00010000
                // 00010000
                // 00010000
                // 00010000
                Rock(0x10101010),
                // 00011000
                // 00011000
                // 00000000
                // 00000000
                Rock(0x00001818),
            ]
        }

        fn move_sideways(&mut self, jet: &Jet, top_of_well: u32) {
            let mut moved = self.0;

            match *jet {
                Jet::Left if self.0 & LEFT_WALL == 0 => moved <<= 1,
                Jet::Right if self.0 & RIGHT_WALL == 0 => moved >>= 1,
                _ => (),
            }

            if moved & top_of_well == 0 {
                self.0 = moved;
            }
        }

        fn bytes(&self) -> impl Iterator<Item = u8> + use<> {
            // `to_le_bytes()` because reasons...
            // I know of endianness, but everytime I have to think about it, my brain
            // hurts. There's a relation to the endianness here, and how we've had to
            // represent the shape of the rocks a little higher up, and probably also
            // how we end up doing collision detection.
            self.0.to_le_bytes().into_iter().take_while(|b| b != &0)
        }
    }

    #[derive(Default)]
    pub struct Well {
        settled_rocks: Vec<u8>,
    }

    impl Well {
        /// Gives 4 rows combined into a `u32`, starting at level `level`.
        fn settled_at_level(&self, level: usize) -> u32 {
            self.settled_rocks
                .iter()
                .skip(level)
                .take(4)
                .rev()
                .fold(0, |acc, row| (acc << 8) | u32::from(*row))
        }

        /// Returns the distance from top to the topmost settled rock per column of the
        /// well.
        ///
        /// Since we've implemented the well in such a way that there can't be a set bit in
        /// the first index, we can omit it here.
        fn canopy(&self) -> Vec<usize> {
            [2, 4, 8, 16, 32, 64, 128]
                .iter()
                .rev()
                .map(|x| {
                    self.settled_rocks
                        .iter()
                        .rev()
                        .enumerate()
                        .find(|(_, row)| *row & x != 0)
                        .unwrap_or((0, &0))
                        .0
                })
                .collect()
        }

        pub fn drop_rocks(jets: &mut JetIterator, rocks_to_drop: usize) -> u64 {
            let mut rocks = Rock::all().into_iter().cycle();
            let mut well = Well::default();
            let mut seen_states = HashMap::new();
            let mut dropped_rocks = 0;
            let mut repeats_height = 0;

            while dropped_rocks < rocks_to_drop {
                let rock = rocks.next().unwrap();
                well.drop_rock(jets, rock);
                dropped_rocks += 1;

                let state = (well.canopy(), rock, jets.index);

                if let Some((prev_dropped_rocks, height)) = seen_states.get(&state) {
                    let repeat_len: usize = dropped_rocks - prev_dropped_rocks;
                    let repeats: usize = (rocks_to_drop - dropped_rocks) / repeat_len;

                    dropped_rocks += repeats * repeat_len;
                    repeats_height += repeats * (well.settled_rocks.len() - height);

                    seen_states.clear();
                    continue;
                }

                seen_states.insert(state, (dropped_rocks, well.settled_rocks.len()));
            }

            log::info!("Well:\n{well}");
            well.settled_rocks.len() as u64 + repeats_height as u64
        }

        fn drop_rock(&mut self, jets: &mut JetIterator, mut rock: Rock) {
            let mut level = self.settled_rocks.len() + 3;

            loop {
                let settled = self.settled_at_level(level);
                let jet = jets.next().unwrap();

                rock.move_sideways(&jet, settled);

                if level > self.settled_rocks.len() {
                    level -= 1;
                    continue;
                }

                if level == 0 || rock.0 & self.settled_at_level(level - 1) != 0 {
                    for byte in rock.bytes().enumerate() {
                        if level + byte.0 >= self.settled_rocks.len() {
                            self.settled_rocks.push(byte.1);
                        } else {
                            self.settled_rocks[level + byte.0] |= byte.1;
                        }
                    }
                    break;
                }

                level -= 1;
            }
        }
    }

    impl Display for Well {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for row in self.settled_rocks.iter().rev() {
                for x in [128, 64, 32, 16, 8, 4, 2, 1] {
                    write!(f, "{}", if row & x != 0 { '#' } else { '.' })?;
                }
                writeln!(f)?;
            }

            Ok(())
        }
    }
}

use aoc_timing::trace::log_run;
use solution::{Jet, Well, JetIterator};
use std::io;

fn main() {
    env_logger::init();

    let input: Vec<_> = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .flat_map(|line| line.chars().map(Jet::try_from).collect::<Vec<_>>())
        .collect::<Result<Vec<_>, String>>()
        .expect("Invalid input");

    let part_1 = log_run("Part 1", || {
        Well::drop_rocks(&mut JetIterator::new(&input), 2022)
    });
    println!("Part 1: {part_1}");
    let part_2 = log_run("Part 2", || {
        Well::drop_rocks(&mut JetIterator::new(&input), 1_000_000_000_000)
    });
    println!("Part 2: {part_2}");
}

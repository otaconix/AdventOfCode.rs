use std::io;

const PRIORITIES: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

struct Rucksack {
    items: String,
}

impl Rucksack {
    /// Returns the first item that appears in both the left and right compartments of the rucksack.
    fn item_in_both_compartments(&self) -> Option<char> {
        let item_count = self.items.len();
        assert_eq!(
            item_count % 2,
            0,
            "Rucksacks must contain an even amount of items."
        );

        for item in self.items[0..item_count / 2].chars() {
            if self.items[item_count / 2..].contains(item) {
                return Option::Some(item);
            }
        }

        Option::None
    }

    /// Returns the first common item among this rucksack and the ones passed in as a slice of rucksacks.
    fn common_item(&self, others: &[Rucksack]) -> Option<char> {
        for item in self.items.chars() {
            if others.iter().all(|rucksack| rucksack.items.contains(item)) {
                return Option::Some(item);
            }
        }

        Option::None
    }
}

trait ItemPriority {
    fn priority(&self) -> u32;
}

impl ItemPriority for char {
    fn priority(&self) -> u32 {
        1 + PRIORITIES
            .chars()
            .position(|c| &c == self)
            .expect(&format!("Unknown item: {}", self)) as u32
    }
}

fn main() {
    let rucksacks: Vec<Rucksack> = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| Rucksack { items: line })
        .collect();

    let silver: u32 = rucksacks
        .iter()
        .map(|rucksack| {
            rucksack
                .item_in_both_compartments()
                .expect("No item found in both compartments!")
                .priority()
        })
        .sum();

    println!("Silver: {}", silver);

    let gold: u32 = rucksacks
        .chunks(3)
        .map(|rucksacks| {
            rucksacks[0]
                .common_item(&rucksacks[1..])
                .expect("No common item found")
                .priority()
        })
        .sum();

    println!("Gold: {}", gold);
}

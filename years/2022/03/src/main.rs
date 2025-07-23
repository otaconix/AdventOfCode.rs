use aoc_timing::trace::log_run;
use std::io;

const PRIORITIES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

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

        self.items[..item_count / 2]
            .chars()
            .find(|&item| self.items[item_count / 2..].contains(item))
    }

    /// Returns the first common item among this rucksack and the ones passed in as a slice of rucksacks.
    fn common_item(&self, others: &[Rucksack]) -> Option<char> {
        self.items
            .chars()
            .find(|&item| others.iter().all(|rucksack| rucksack.items.contains(item)))
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
            .unwrap_or_else(|| panic!("Unknown item: {self}")) as u32
    }
}

fn main() {
    env_logger::init();

    let rucksacks: Vec<Rucksack> = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| Rucksack { items: line })
        .collect();

    let silver: u32 = log_run("Part 1", || {
        rucksacks
            .iter()
            .map(|rucksack| {
                rucksack
                    .item_in_both_compartments()
                    .expect("No item found in both compartments!")
                    .priority()
            })
            .sum()
    });

    println!("Silver: {silver}");

    let gold: u32 = log_run("Part 2", || {
        rucksacks
            .chunks(3)
            .map(|rucksacks| {
                rucksacks[0]
                    .common_item(&rucksacks[1..])
                    .expect("No common item found")
                    .priority()
            })
            .sum()
    });

    println!("Gold: {gold}");
}

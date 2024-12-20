use aoc_timing::trace::log_run;

#[derive(Debug, Clone)]
enum MonkeyOperation {
    MultiplicationSelf,
    Multiplication(u64),
    Addition(u64),
}

impl MonkeyOperation {
    fn apply(&self, n: &u64) -> u64 {
        match self {
            MonkeyOperation::MultiplicationSelf => n * n,
            MonkeyOperation::Multiplication(x) => n * x,
            MonkeyOperation::Addition(x) => n + x,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: MonkeyOperation,
    test_divisor: u64,
    true_destination: usize,
    false_destination: usize,
    inspections: usize,
}

impl Monkey {
    fn new_item(&self, old_item: &u64) -> u64 {
        self.operation.apply(old_item)
    }
}

fn play_round<F>(monkeys: &mut [Monkey], common_divisor: u64, f: F)
where
    F: Fn(u64) -> u64,
{
    for monkey_index in 0..monkeys.len() {
        let monkey = &mut monkeys[monkey_index];
        monkey.inspections += monkey.items.len();

        let items = &monkeys[monkey_index].items;

        for item_index in 0..items.len() {
            let monkey = &monkeys[monkey_index];
            let new_item = f(monkey.new_item(&monkey.items[item_index]));
            let destination_index = if new_item % monkey.test_divisor == 0 {
                monkey.true_destination
            } else {
                monkey.false_destination
            };

            monkeys[destination_index]
                .items
                .push(new_item % common_divisor);
        }

        monkeys[monkey_index].items.clear();
    }
}

fn main() {
    env_logger::init();

    let monkeys = vec![
        Monkey {
            items: vec![66, 79],
            operation: MonkeyOperation::Multiplication(11),
            test_divisor: 7,
            true_destination: 6,
            false_destination: 7,
            inspections: 0,
        },
        Monkey {
            items: vec![84, 94, 94, 81, 98, 75],
            operation: MonkeyOperation::Multiplication(17),
            test_divisor: 13,
            true_destination: 5,
            false_destination: 2,
            inspections: 0,
        },
        Monkey {
            items: vec![85, 79, 59, 64, 79, 95, 67],
            operation: MonkeyOperation::Addition(8),
            test_divisor: 5,
            true_destination: 4,
            false_destination: 5,
            inspections: 0,
        },
        Monkey {
            items: vec![70],
            operation: MonkeyOperation::Addition(3),
            test_divisor: 19,
            true_destination: 6,
            false_destination: 0,
            inspections: 0,
        },
        Monkey {
            items: vec![57, 69, 78, 78],
            operation: MonkeyOperation::Addition(4),
            test_divisor: 2,
            true_destination: 0,
            false_destination: 3,
            inspections: 0,
        },
        Monkey {
            items: vec![65, 92, 60, 74, 72],
            operation: MonkeyOperation::Addition(7),
            test_divisor: 11,
            true_destination: 3,
            false_destination: 4,
            inspections: 0,
        },
        Monkey {
            items: vec![77, 91, 91],
            operation: MonkeyOperation::MultiplicationSelf,
            test_divisor: 17,
            true_destination: 1,
            false_destination: 7,
            inspections: 0,
        },
        Monkey {
            items: vec![76, 58, 57, 55, 67, 77, 54, 99],
            operation: MonkeyOperation::Addition(6),
            test_divisor: 3,
            true_destination: 2,
            false_destination: 1,
            inspections: 0,
        },
    ];
    let common_divisor: u64 = monkeys.iter().map(|monkey| monkey.test_divisor).product();

    let part_1 = log_run("Part 1", || {
        let mut part_1_monkeys = monkeys.clone();
        for _round in 0..20 {
            play_round(&mut part_1_monkeys, common_divisor, |item| item / 3);
        }
        part_1_monkeys.sort_by_cached_key(|monkey| monkey.inspections);
        part_1_monkeys.reverse();

        part_1_monkeys[0].inspections * part_1_monkeys[1].inspections
    });
    println!("Part 1: {part_1}");

    let mut part_2_monkeys = monkeys.clone();
    let part_2 = log_run("Part 2", || {
        for _round in 0..10_000 {
            play_round(&mut part_2_monkeys, common_divisor, |item| item);
        }
        part_2_monkeys.sort_by_cached_key(|monkey| monkey.inspections);
        part_2_monkeys.reverse();

        part_2_monkeys[0].inspections * part_2_monkeys[1].inspections
    });
    println!("Part 2: {part_2}");
}

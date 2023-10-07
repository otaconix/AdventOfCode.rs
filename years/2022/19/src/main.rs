mod solution {
    use pom::utf8::*;
    use std::str::FromStr;

    #[derive(Debug)]
    enum Resource {
        Ore,
        Clay,
        Obsidian,
        Geode,
    }

    #[derive(Default, Debug)]
    struct Cost {
        ore: u8,
        clay: u8,
        obsidian: u8,
    }

    fn number_parser<'a>() -> Parser<'a, u8> {
        is_a(|c| c.is_ascii_digit())
            .repeat(1..)
            .collect()
            .convert(|digits| digits.parse::<u8>())
    }

    fn resource_parser<'a>() -> Parser<'a, Resource> {
        let ore = seq("ore").map(|_| Resource::Ore);
        let clay = seq("clay").map(|_| Resource::Clay);
        let obsidian = seq("obsidian").map(|_| Resource::Obsidian);
        let geode = seq("geode").map(|_| Resource::Geode);

        (ore | clay | obsidian | geode).name("resource")
    }

    impl Cost {
        fn parser<'a>() -> Parser<'a, Self> {
            let single_cost = || number_parser() - sym(' ') + resource_parser();
            let costs = single_cost() + (seq(" and ") * single_cost()).repeat(0..);

            costs
                .map(|(first, rest)| {
                    let mut result = Cost::default();

                    for item in [first].iter().chain(rest.iter()) {
                        match item.1 {
                            Resource::Ore => result.ore = item.0,
                            Resource::Clay => result.clay = item.0,
                            Resource::Obsidian => result.obsidian = item.0,
                            Resource::Geode => (),
                        }
                    }

                    result
                })
                .name("cost")
        }
    }

    #[derive(Debug)]
    pub struct Blueprint {
        id: u8,
        ore_robot_cost: Cost,
        clay_robot_cost: Cost,
        obsidian_robot_cost: Cost,
        geode_robot_cost: Cost,
    }

    impl Blueprint {
        fn new(id: u8) -> Self {
            Blueprint {
                id,
                ore_robot_cost: Cost::default(),
                clay_robot_cost: Cost::default(),
                obsidian_robot_cost: Cost::default(),
                geode_robot_cost: Cost::default(),
            }
        }

        fn parser<'a>() -> Parser<'a, Self> {
            let robot_cost_parser = seq("Each ")
                * (resource_parser() + (seq(" robot costs ") * Cost::parser())
                    - (sym('.') + sym(' ').opt()));
            let parser =
                seq("Blueprint ") * number_parser() + (seq(": ") * robot_cost_parser.repeat(1..));

            parser.map(|x| {
                let mut blueprint = Blueprint::new(x.0);

                for (resource, cost) in x.1 {
                    match resource {
                        Resource::Ore => blueprint.ore_robot_cost = cost,
                        Resource::Clay => blueprint.clay_robot_cost = cost,
                        Resource::Obsidian => blueprint.obsidian_robot_cost = cost,
                        Resource::Geode => blueprint.geode_robot_cost = cost,
                    }
                }

                blueprint
            })
        }
    }

    impl FromStr for Blueprint {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Blueprint::parser()
                .parse_str(s)
                .map_err(|err| err.to_string())
        }
    }
}

use solution::*;
use std::io;

fn main() {
    let blueprints: Vec<Blueprint> = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| line.parse().expect("Failed to parse blueprint"))
        .collect();

    println!("Blueprints:\n{blueprints:#?}");
}

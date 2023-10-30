use super::*;
use pom::utf8::*;

fn number_parser<'a>() -> Parser<'a, u32> {
    is_a(|c| c.is_ascii_digit())
        .repeat(1..)
        .collect()
        .convert(|digits| digits.parse())
}

fn resource_parser<'a>() -> Parser<'a, Resource> {
    let ore = seq("ore").map(|_| Resource::Ore);
    let clay = seq("clay").map(|_| Resource::Clay);
    let obsidian = seq("obsidian").map(|_| Resource::Obsidian);
    let geode = seq("geode").map(|_| Resource::Geode);

    (ore | clay | obsidian | geode).name("resource")
}

fn resources_parser<'a>() -> Parser<'a, Resources> {
    let single_cost = || number_parser() - sym(' ') + resource_parser();
    let costs = single_cost() + (seq(" and ") * single_cost()).repeat(0..);

    costs
        .map(|(first, rest)| {
            let mut result = Resources::default();

            for item in [first].iter().chain(rest.iter()) {
                result[&item.1] = item.0;
            }

            result
        })
        .name("cost")
}

pub(crate) fn blueprint_parser<'a>() -> Parser<'a, Blueprint> {
    let robot_cost_parser = seq("Each ")
        * (resource_parser() + (seq(" robot costs ") * resources_parser())
            - (sym('.') + sym(' ').opt()));
    let parser = seq("Blueprint ") * number_parser() + (seq(": ") * robot_cost_parser.repeat(1..));

    parser.map(|x| Blueprint::new(x.0, x.1.into_iter().collect()))
}

use itertools::Itertools;
use rapidhash::RapidHashSet;

type JunctionBox = (i64, i64, i64);
type Input = Vec<JunctionBox>;
type Output1 = usize;
type Output2 = Output1;

pub fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();
            let coords = line
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>();

            (coords[0], coords[1], coords[2])
        })
        .collect()
}

fn euclidean_distance(a: JunctionBox, b: JunctionBox) -> i64 {
    ((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)).isqrt()
}

fn merge_groups(
    group_a: &mut RapidHashSet<JunctionBox>,
    group_b: &mut RapidHashSet<JunctionBox>,
) -> bool {
    if group_b.iter().any(|b| group_a.contains(b)) {
        group_a.extend(group_b.iter());
        true
    } else {
        false
    }
}

fn part_1_parameterized(input: &Input, junctions_to_connect: usize) -> Output1 {
    let mut groups = (0..input.len())
        .flat_map(|index_a| {
            (index_a + 1..input.len()).map(move |index_b| (input[index_a], input[index_b]))
        })
        .sorted_by_key(|(a, b)| euclidean_distance(*a, *b))
        .take(junctions_to_connect)
        .fold(vec![], |mut groups: Vec<RapidHashSet<_>>, (a, b)| {
            if let Some(existing_group) = groups
                .iter_mut()
                .find(|group| group.contains(&a) || group.contains(&b))
            {
                existing_group.insert(a);
                existing_group.insert(b);
            } else {
                let mut new_group = RapidHashSet::default();
                new_group.insert(a);
                new_group.insert(b);
                groups.push(new_group);
            }

            groups
        })
        .into_iter()
        .collect_vec();

    loop {
        let mut any_merged = false;
        let mut i = 0;

        loop {
            if i >= groups.len() {
                break;
            }

            let mut j = 0;
            loop {
                let (groups_left, groups_right) = groups.split_at_mut(i + 1);
                if j >= groups_right.len() {
                    break;
                }
                if merge_groups(&mut groups_left[i], &mut groups_right[j]) {
                    any_merged = true;
                    groups.remove(i + j + 1);
                } else {
                    j += 1;
                }
            }

            i += 1;
        }

        if !any_merged {
            break;
        }
    }

    groups
        .into_iter()
        .map(|group| group.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

pub fn part_1(input: &Input) -> Output1 {
    part_1_parameterized(input, 1000)
}

pub fn part_2(input: &Input) -> Output2 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1_parameterized(&input, 10);

        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 0);
    }
}

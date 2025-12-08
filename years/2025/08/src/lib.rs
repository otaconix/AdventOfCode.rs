use itertools::Itertools;
use rapidhash::RapidHashSet;

type JunctionBox = (i64, i64, i64);
pub struct Input {
    junction_box_count: usize,
    junction_box_pairs: Vec<(JunctionBox, JunctionBox)>,
}
type Output1 = usize;
type Output2 = Output1;

pub fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let junction_boxes = input
        .map(|line| {
            let line = line.as_ref();
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple::<(_, _, _)>()
                .unwrap()
        })
        .collect_vec();

    let mut junction_box_pairs = junction_boxes
        .iter()
        .combinations(2)
        .map(|pair| (*pair[0], *pair[1]))
        .collect_vec();
    junction_box_pairs.sort_unstable_by_key(|(a, b)| squared_euclidean_distance(*a, *b));

    Input {
        junction_box_count: junction_boxes.len(),
        junction_box_pairs,
    }
}

#[inline]
fn squared_euclidean_distance(a: JunctionBox, b: JunctionBox) -> i64 {
    let x = a.0 - b.0;
    let x = x * x;
    let y = a.1 - b.1;
    let y = y * y;
    let z = a.2 - b.2;
    let z = z * z;

    x + y + z
}

fn merge_groups(
    group_a: &mut RapidHashSet<JunctionBox>,
    group_b: &mut RapidHashSet<JunctionBox>,
) -> bool {
    if group_b.iter().any(|b| group_a.contains(b)) {
        group_a.reserve(group_b.len());
        group_a.extend(group_b.drain());
        true
    } else {
        false
    }
}

fn merge_groups_into(into_index: usize, groups: &mut Vec<RapidHashSet<JunctionBox>>) {
    if (0..groups.len()).fold(false, |any_merged, current_index| {
        if current_index == into_index {
            any_merged
        } else {
            let [into, from] = groups
                .get_disjoint_mut([into_index, current_index])
                .unwrap();

            merge_groups(into, from) || any_merged
        }
    }) {
        groups.retain(|group| !group.is_empty());
    }
}

fn part_1_parameterized(input: &Input, junctions_to_connect: usize) -> Output1 {
    input
        .junction_box_pairs
        .iter()
        .take(junctions_to_connect)
        .fold(vec![], |mut groups: Vec<RapidHashSet<_>>, (a, b)| {
            let mut new_group = RapidHashSet::default();
            new_group.insert(*a);
            new_group.insert(*b);
            groups.push(new_group);
            merge_groups_into(groups.len() - 1, &mut groups);

            groups
        })
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
    let mut groups: Vec<RapidHashSet<JunctionBox>> = vec![];
    let mut pairs = input.junction_box_pairs.iter().rev().copied().collect_vec();

    while let Some((a, b)) = pairs.pop() {
        if let Some(existing_group) = groups
            .iter_mut()
            .find(|group| group.contains(&a) || group.contains(&b))
        {
            existing_group.insert(a);
            existing_group.insert(b);

            merge_groups_into(groups.len() - 1, &mut groups);
        } else {
            let mut new_group = RapidHashSet::default();
            new_group.insert(a);
            new_group.insert(b);
            groups.push(new_group);
        }

        if groups.len() == 1 && groups[0].len() == input.junction_box_count {
            return (a.0 * b.0).try_into().unwrap();
        }
    }

    panic!("Never reached end state?")
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

        assert_eq!(result, 25272);
    }
}

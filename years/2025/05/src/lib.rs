use std::ops::RangeInclusive;

type Input = (Vec<RangeInclusive<usize>>, Vec<usize>);
type Output1 = usize;
type Output2 = Output1;

pub fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    enum State {
        Ranges(Vec<RangeInclusive<usize>>),
        Ids(Vec<RangeInclusive<usize>>, Vec<usize>),
    }

    use State::*;

    let end_state = input.fold(Ranges(vec![]), |state, line| match state {
        Ranges(mut ranges) => {
            let line = line.as_ref();
            if line.is_empty() {
                ranges.sort_unstable_by_key(|range| *range.start());
                Ids(ranges, vec![])
            } else {
                let (start, end) = line.split_once('-').unwrap();
                let start = start.parse().unwrap();
                let end = end.parse().unwrap();

                ranges.push(RangeInclusive::new(start, end));

                Ranges(ranges)
            }
        }
        Ids(ranges, mut ids) => {
            let line = line.as_ref();

            ids.push(line.parse().unwrap());

            Ids(ranges, ids)
        }
    });

    match end_state {
        Ranges(_) => panic!("Parsing failed (only got ranges?)"),
        Ids(ranges, mut ids) => {
            ids.sort_unstable();
            (ranges, ids)
        }
    }
}

pub fn part_1((ranges, ids): &Input) -> Output1 {
    let mut range_index = 0;
    let mut valids = 0;

    for id in ids {
        if let Some(new_range_index) = ranges[range_index..]
            .iter()
            .position(|range| range.contains(id))
        {
            valids += 1;
            range_index = new_range_index;
        }
    }

    valids
}

pub fn part_2((ranges, _): &Input) -> Output2 {
    ranges
        .iter()
        .fold((0, 0), |(total, current_min), range| {
            let start = range.start().max(&current_min);

            if start > range.end() {
                (total, current_min)
            } else {
                (total + range.end() - start + 1, range.end() + 1)
            }
        })
        .0
}

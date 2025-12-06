type Input = (String, Vec<String>);
type Output1 = u64;
type Output2 = Output1;

pub fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let mut lines = input
        .map(|line| line.as_ref().to_string())
        .collect::<Vec<_>>();

    let operators = lines.pop().unwrap();

    (operators, lines)
}

pub fn part_1((operators, operands): &Input) -> Output1 {
    let operands = operands
        .iter()
        .map(|operands| {
            operands
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    operators
        .split_whitespace()
        .enumerate()
        .map(|(index, operator)| {
            let operands = operands.iter().map(|operands| operands[index]);

            match operator {
                "*" => operands.product::<u64>(),
                "+" => operands.sum::<u64>(),
                _ => panic!("Invalid operator {operator}"),
            }
        })
        .sum()
}

pub fn part_2((operators, operands): &Input) -> Output2 {
    operators
        .chars()
        .enumerate()
        .filter(|(_, c)| *c != ' ')
        .map(|(column, operator)| {
            let operands = (column..)
                .map(|column| {
                    operands
                        .iter()
                        .flat_map(|o| o.chars().nth(column).and_then(|d| d.to_digit(10)))
                        .fold(0u64, |o, n| o * 10 + u64::from(n))
                })
                .take_while(|n| n != &0);

            match operator {
                '*' => operands.product::<Output2>(),
                '+' => operands.sum::<Output2>(),
                _ => panic!("Invalid operator"),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 3263827);
    }
}

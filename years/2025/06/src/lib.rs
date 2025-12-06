use grid::Grid;

#[derive(PartialEq, PartialOrd)]
pub enum Cell {
    Multiply,
    Add,
    Number(u64),
}

type Input = Vec<String>;
type Output1 = u64;
type Output2 = Output1;

pub fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input.map(|line| line.as_ref().to_string()).collect()
}

trait ColCalc {
    fn column_calc(&self, column: usize) -> Output1;
}

impl ColCalc for Grid<Cell> {
    fn column_calc(&self, column: usize) -> Output1 {
        let mut column_cells = self.column(column).rev();
        let operation = column_cells.next().unwrap();
        let operands = column_cells.map(|c| match c {
            Cell::Number(n) => n,
            _ => panic!("Operation found in unexpected row of column {column}"),
        });
        match operation {
            Cell::Multiply => operands.product(),
            Cell::Add => operands.sum(),
            _ => panic!("Number found in unexpected row of column {column}"),
        }
    }
}

pub fn part_1(input: &Input) -> Output1 {
    let grid: Grid<Cell> = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| match n {
                    "*" => Cell::Multiply,
                    "+" => Cell::Add,
                    n => Cell::Number(n.parse().unwrap()),
                })
                .collect()
        })
        .collect();
    (0..grid.width())
        .map(|column| grid.column_calc(column))
        .sum()
}

pub fn part_2(input: &Input) -> Output2 {
    let operators = &input[input.len() - 1];
    let operands = &input[..input.len() - 1];

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

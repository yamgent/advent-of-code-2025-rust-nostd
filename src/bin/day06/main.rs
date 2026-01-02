const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/06/input.txt");

enum Equation {
    Add(Vec<i64>),
    Mul(Vec<i64>),
}

impl Equation {
    fn calc(&self) -> i64 {
        match self {
            Self::Add(nums) => nums.iter().sum(),
            Self::Mul(nums) => nums.iter().product(),
        }
    }
}

fn parse_input_p1(input: &str) -> Vec<Equation> {
    let numbers = input
        .trim()
        .lines()
        .rev()
        .skip(1)
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().expect("a number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    input
        .trim()
        .lines()
        .next_back()
        .expect("at least two lines, last line should contain the symbols")
        .split_whitespace()
        .enumerate()
        .map(|(column_idx, symbol)| {
            let column_numbers = numbers.iter().map(|row| row[column_idx]).collect();

            if symbol == "+" {
                Equation::Add(column_numbers)
            } else if symbol == "*" {
                Equation::Mul(column_numbers)
            } else {
                panic!("expect + or *, found {}", symbol);
            }
        })
        .collect()
}

fn p1(input: &str) -> String {
    parse_input_p1(input)
        .into_iter()
        .map(|entry| entry.calc())
        .sum::<i64>()
        .to_string()
}

fn parse_input_p2(input: &str) -> Vec<Equation> {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let equation_operators = input
        .trim()
        .lines()
        .next_back()
        .expect("at least two lines, last line should contain the symbols")
        .chars()
        .enumerate()
        .filter(|(_, ch)| *ch == '+' || *ch == '*')
        .collect::<Vec<_>>();

    equation_operators
        .iter()
        .zip(
            equation_operators
                .iter()
                .skip(1)
                .chain([(input.len(), ' ')].iter()),
        )
        .map(|(from, to)| {
            let numbers = (from.0..to.0)
                .map(|column_idx| {
                    grid.iter().fold(0, |acc, row| {
                        if let Some(digit) = row.get(column_idx).and_then(|ch| ch.to_digit(10)) {
                            acc * 10 + digit as i64
                        } else {
                            acc
                        }
                    })
                })
                .filter(|number| *number > 0)
                .collect();

            match from.1 {
                '+' => Equation::Add(numbers),
                '*' => Equation::Mul(numbers),
                _ => panic!("not possible, we checked for + and * earlier"),
            }
        })
        .collect()
}

fn p2(input: &str) -> String {
    parse_input_p2(input)
        .into_iter()
        .map(|entry| entry.calc())
        .sum::<i64>()
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "4277556");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "4722948564882");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "3263827");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "9581313737063");
    }
}

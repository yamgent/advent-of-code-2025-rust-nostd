const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/01/input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Left(u32),
    Right(u32),
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let direction = line.chars().next().expect("a character");
        let amount = line
            .chars()
            .skip(1)
            .collect::<String>()
            .parse()
            .expect("a number");

        match direction {
            'L' => Self::Left(amount),
            'R' => Self::Right(amount),
            _ => {
                panic!("expect L or R");
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.trim().lines().map(Instruction::parse).collect()
}

const START_NUMBER: i32 = 50;
const TOTAL_NUMBERS: i32 = 100;

fn p1(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .fold((START_NUMBER, 0u32), |mut acc, line| {
            let amount = match line {
                Instruction::Left(amount) => -(amount as i32),
                Instruction::Right(amount) => amount as i32,
            };

            acc.0 = (acc.0 + amount).rem_euclid(TOTAL_NUMBERS);

            if acc.0 == 0 {
                acc.1 += 1;
            }

            acc
        })
        .1
        .to_string()
}

fn p2(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .fold((START_NUMBER, 0u32), |mut acc, line| {
            match line {
                Instruction::Left(amount) => {
                    let amount = amount as i32;
                    if acc.0 == amount {
                        acc.1 += 1;
                    } else if acc.0 < amount {
                        let remaining = amount - acc.0;
                        acc.1 +=
                            if acc.0 == 0 { 0 } else { 1 } + (remaining / TOTAL_NUMBERS) as u32;
                    }
                    acc.0 = (acc.0 - amount).rem_euclid(TOTAL_NUMBERS);
                }
                Instruction::Right(amount) => {
                    let amount = amount as i32;
                    acc.1 += ((acc.0 + amount) / TOTAL_NUMBERS) as u32;
                    acc.0 = (acc.0 + amount).rem_euclid(TOTAL_NUMBERS);
                }
            }

            acc
        })
        .1
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instructions() {
        assert_eq!(
            parse_input(
                r"
L20
R30
"
            ),
            vec![Instruction::Left(20), Instruction::Right(30)]
        );
    }

    const SAMPLE_INPUT: &str = r"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "3");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1066");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "6");
    }

    #[test]
    fn test_p2_additional() {
        [
            ("L50", 1),
            ("L50\nL99", 1),
            ("L50\nL100", 2),
            ("L50\nL101", 2),
            ("L50\nL199", 2),
            ("L50\nL200", 3),
            ("L50\nL201", 3),
            ("L50\nR99", 1),
            ("L50\nR100", 2),
            ("L50\nR101", 2),
            ("L50\nR199", 2),
            ("L50\nR200", 3),
            ("L50\nR201", 3),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            assert_eq!(
                p2(input),
                expected.to_string(),
                "{}, {}",
                input.replace("\n", ";"),
                expected
            );
        });
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "6223");
    }
}

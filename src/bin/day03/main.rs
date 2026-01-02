const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/03/input.txt");

fn total_digits(number: u64) -> u32 {
    if number == 0 {
        return 1;
    }

    let mut remaining = number;
    let mut count = 0;

    while remaining > 0 {
        remaining /= 10;
        count += 1;
    }

    count
}

// empty strings can be represented as 0 (eg: parts = [4, 0, 5] => 45)
// fortunately, for the puzzle, 0 will never appear as part of a bank's battery joltage
fn construct_number(parts: &[u64]) -> u64 {
    parts
        .iter()
        .filter(|part| **part != 0)
        .fold(0, |acc, part| acc * 10_u64.pow(total_digits(*part)) + *part)
}

fn digitize(line: &str) -> Vec<u64> {
    line.trim()
        .chars()
        .map(|ch| ch.to_digit(10).expect("a digit") as u64)
        .collect()
}

fn solve_bank(digits: &[u64], total_needed: usize) -> u64 {
    // e.g.
    //      bank = "1634"
    //      total_needed = 2
    //
    //      f(3, 0) = ""
    //      f(2, 0) = ""
    //      f(1, 0) = ""
    //      f(0, 0) = ""
    //
    //      f(3, 1) = "4"
    //      f(2, 1) = max("3" + f(3, 0), f(3, 1)) = max("3", "4") = "4"
    //      f(1, 1) = max("6" + f(2, 0), f(2, 1)) = max("6", "4") = "6"
    //      f(0, 1) = max("1" + f(1, 0), f(1, 1)) = max("1", "6") = "6" <- observe this is not used
    //
    //      f(2, 2) = "34"
    //      f(1, 2) = max("6" + f(2, 1), f(2, 2)) = max("64", "34") = "64"
    //      f(0, 2) = max("1" + f(1, 1), f(1, 2)) = max("16", "64") = "64"
    //
    //      since total_needed = 2, answer is in f(0, 2) = "64"
    //
    let mut current = std::iter::repeat_n(0u64, digits.len()).collect::<Vec<_>>();

    (1..=total_needed).for_each(|level| {
        let mut next = std::iter::repeat_n(0u64, digits.len()).collect::<Vec<_>>();

        let base_case_idx = digits.len() - level;
        next[base_case_idx] = construct_number(&digits[base_case_idx..]);

        (0..base_case_idx).rev().for_each(|current_idx| {
            next[current_idx] = construct_number(&[digits[current_idx], current[current_idx + 1]])
                .max(next[current_idx + 1]);
        });

        current = next;
    });
    current[0]
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            let digits = digitize(line);

            // this is actually the same logic as solve_bank(&digits, 2)
            // but we are preserving it because this is the first solution we came up
            // with when we initially solve this part
            let maxs_reversed = digits.iter().rev().fold(vec![], |mut acc, entry| {
                acc.push(*acc.last().unwrap_or(entry).max(entry));
                acc
            });

            digits
                .iter()
                .zip(maxs_reversed.iter().rev().skip(1))
                .map(|(a, b)| a * 10 + b)
                .max()
                .expect("not empty array")
        })
        .sum::<u64>()
        .to_string()
}

fn p2(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| solve_bank(&digitize(line), 12))
        .sum::<u64>()
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
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_total_digits() {
        assert_eq!(total_digits(0), 1);
        assert_eq!(total_digits(1), 1);
        assert_eq!(total_digits(9), 1);
        assert_eq!(total_digits(10), 2);
        assert_eq!(total_digits(11), 2);
        assert_eq!(total_digits(99), 2);
        assert_eq!(total_digits(100), 3);
        assert_eq!(total_digits(101), 3);
    }

    #[test]
    fn test_construct_number() {
        assert_eq!(construct_number(&[0]), 0);
        assert_eq!(construct_number(&[0, 0]), 0);
        assert_eq!(construct_number(&[4, 0]), 4);
        assert_eq!(construct_number(&[4, 0, 5]), 45);

        assert_eq!(construct_number(&[4]), 4);
        assert_eq!(construct_number(&[4, 5]), 45);
        assert_eq!(construct_number(&[4, 567]), 4567);
        assert_eq!(construct_number(&[41, 567]), 41567);
        assert_eq!(construct_number(&[412, 567]), 412567);
        assert_eq!(construct_number(&[412, 5]), 4125);
    }

    #[test]
    fn test_digitize() {
        assert_eq!(digitize("123"), vec![1, 2, 3]);
    }

    #[test]
    fn test_solve_bank() {
        assert_eq!(solve_bank(&digitize("987654321111111"), 12), 987654321111);
        assert_eq!(solve_bank(&digitize("811111111111119"), 12), 811111111119);
        assert_eq!(solve_bank(&digitize("234234234234278"), 12), 434234234278);
        assert_eq!(solve_bank(&digitize("818181911112111"), 12), 888911112111);
    }

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "357");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "17376");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "3121910778619");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "172119830406258");
    }
}

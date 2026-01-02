const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/02/input.txt");

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(",")
        .map(|entry| entry.split_once("-").expect("format to be x-x"))
        .map(|entry| {
            (
                entry.0.parse().expect("a number"),
                entry.1.parse().expect("a number"),
            )
        })
        .collect()
}

fn decompose(id: u64) -> Vec<u8> {
    let mut number = id;
    let mut digits = vec![];
    while number > 0 {
        digits.push((number % 10) as u8);
        number /= 10;
    }
    digits
}

fn is_invalid(id: u64) -> bool {
    let digits = decompose(id);

    if !digits.len().is_multiple_of(2) {
        return false;
    }

    digits
        .iter()
        .take(digits.len() / 2)
        .zip(digits.iter().skip(digits.len() / 2))
        .all(|(a, b)| a == b)
}

fn p1(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .map(|entry| {
            (entry.0..=entry.1)
                .filter(|id| is_invalid(*id))
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string()
}

fn is_invalid_p2(id: u64) -> bool {
    let digits = decompose(id);

    (1..digits.len())
        .filter(|group_size| digits.len().is_multiple_of(*group_size))
        .any(|group_size| {
            (0..group_size).all(|i| {
                let mut j = i + group_size;
                while j < digits.len() {
                    if digits[j] != digits[i] {
                        return false;
                    }
                    j += group_size;
                }
                true
            })
        })
}

fn p2(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .map(|entry| {
            (entry.0..=entry.1)
                .filter(|id| is_invalid_p2(*id))
                .sum::<u64>()
        })
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

    const SAMPLE_INPUT: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "1227775554");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "15873079081");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "4174379265");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "22617871034");
    }
}

use std::collections::{HashMap, HashSet};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/07/input.txt");

struct Input {
    start_pos: (i64, i64),
    splitters: HashSet<(i64, i64)>,
    total_rows: i64,
}

impl Input {
    fn parse(input: &str) -> Self {
        let start_pos = (
            input
                .trim()
                .lines()
                .next()
                .expect("at least one row")
                .chars()
                .enumerate()
                .find(|(_, ch)| *ch == 'S')
                .expect("the starter should always be on the first line")
                .0 as i64,
            0,
        );

        let splitters = input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch == '^')
                    .map(move |(x, _)| (x as i64, y as i64))
            })
            .collect();

        let total_rows = input.trim().lines().count() as i64;

        Self {
            start_pos,
            splitters,
            total_rows,
        }
    }
}

fn p1(input: &str) -> String {
    let input = Input::parse(input);

    (input.start_pos.1..input.total_rows)
        .fold(
            (HashSet::from([input.start_pos.0]), 0_i64),
            |(current_xs, mut count), current_y| {
                let mut new_xs = HashSet::new();

                current_xs.into_iter().for_each(|x| {
                    if input.splitters.contains(&(x, current_y)) {
                        new_xs.insert(x - 1);
                        new_xs.insert(x + 1);
                        count += 1;
                    } else {
                        new_xs.insert(x);
                    }
                });

                (new_xs, count)
            },
        )
        .1
        .to_string()
}

fn p2(input: &str) -> String {
    let input = Input::parse(input);

    (input.start_pos.1..input.total_rows)
        .fold(
            HashMap::from([(input.start_pos.0, 1)]),
            |current_xs, current_y| {
                let mut new_xs = HashMap::new();

                current_xs.into_iter().for_each(|(x, count)| {
                    if input.splitters.contains(&(x, current_y)) {
                        *new_xs.entry(x - 1).or_default() += count;
                        *new_xs.entry(x + 1).or_default() += count;
                    } else {
                        *new_xs.entry(x).or_default() += count;
                    }
                });

                new_xs
            },
        )
        .values()
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
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "21");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1541");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "40");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "80158285728929");
    }
}

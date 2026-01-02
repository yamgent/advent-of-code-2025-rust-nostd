use std::collections::{HashMap, HashSet};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/04/input.txt");

const DIRECTIONS_DELTA: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn parse_map(input: &str) -> HashMap<(i64, i64), usize> {
    let papers: HashSet<_> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '@')
                .map(move |(x, _)| (x as i64, y as i64))
        })
        .collect::<HashSet<_>>();

    papers
        .iter()
        .map(|coord| {
            (
                *coord,
                DIRECTIONS_DELTA
                    .into_iter()
                    .map(|delta| (coord.0 + delta.0, coord.1 + delta.1))
                    .filter(|neighbour| papers.contains(neighbour))
                    .count(),
            )
        })
        .collect::<HashMap<_, _>>()
}

const ACCESSIBLE_MAX: usize = 3;

fn p1(input: &str) -> String {
    parse_map(input)
        .values()
        .filter(|neighbours_count| **neighbours_count <= ACCESSIBLE_MAX)
        .count()
        .to_string()
}

fn p2(input: &str) -> String {
    let mut papers = parse_map(input);

    let mut count = 0;
    let mut to_remove = papers
        .iter()
        .filter(|(_, neighbours_count)| **neighbours_count <= ACCESSIBLE_MAX)
        .map(|(coord, _)| *coord)
        .collect::<HashSet<_>>();

    while !to_remove.is_empty() {
        count += to_remove.len();

        let mut new_candidates = HashSet::new();

        to_remove.iter().for_each(|coord| {
            DIRECTIONS_DELTA
                .into_iter()
                .map(|delta| (coord.0 + delta.0, coord.1 + delta.1))
                .for_each(|neighbour| {
                    if let Some(value) = papers.get_mut(&neighbour) {
                        *value -= 1;
                        new_candidates.insert(neighbour);
                    }
                });
            papers.remove(coord);
        });

        to_remove = new_candidates
            .into_iter()
            .filter(|coord| {
                papers
                    .get(coord)
                    .map(|neighbours_count| *neighbours_count <= ACCESSIBLE_MAX)
                    .unwrap_or(false)
            })
            .collect();
    }

    count.to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "13");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1474");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "43");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "8910");
    }
}

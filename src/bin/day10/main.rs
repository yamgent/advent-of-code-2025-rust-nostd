use std::collections::HashSet;

use z3::{Optimize, SatResult, ast::Int};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/10/input.txt");

#[derive(PartialEq, Eq, Debug)]
struct MachineRequirements {
    expected_indicator_lights_state: u16,
    total_lights: usize,
    button_wirings: Vec<Vec<usize>>,
    joltage_requirements: Vec<u64>,
}

impl MachineRequirements {
    fn parse_line(line: &str) -> Self {
        let parts = line.split(" ").collect::<Vec<_>>();

        let expected_indicator_lights_state = parts
            .first()
            .expect("have at least three parts")
            .replace("[", "")
            .replace("]", "")
            .chars()
            .fold(0, |acc, ch| match ch {
                '.' => acc * 2,
                '#' => acc * 2 + 1,
                _ => panic!("Invalid input, only expected . or #"),
            });

        let total_lights = parts
            .first()
            .expect("have at least three parts")
            .replace("[", "")
            .replace("]", "")
            .chars()
            .count();

        let button_wirings = parts
            .iter()
            .skip(1)
            .take(parts.len() - 2)
            .map(|part| {
                part.replace("(", "")
                    .replace(")", "")
                    .split(",")
                    .map(|number| number.parse().expect("a number"))
                    .collect()
            })
            .collect();

        let joltage_requirements = parts
            .last()
            .expect("have at least three part")
            .replace("{", "")
            .replace("}", "")
            .split(",")
            .map(|number| number.parse().expect("a number"))
            .collect();

        Self {
            expected_indicator_lights_state,
            total_lights,
            button_wirings,
            joltage_requirements,
        }
    }
}

fn press_button(current_state: u16, button_wiring: &[usize], total_lights: usize) -> u16 {
    button_wiring.iter().fold(current_state, |acc, wire| {
        acc ^ (1 << (total_lights - 1 - *wire))
    })
}

fn solve_lights(requirement: &MachineRequirements) -> u64 {
    let mut current_level = 0;
    let mut current_level_states = vec![0];

    let mut visited: HashSet<u16> = HashSet::new();

    while current_level_states
        .iter()
        .all(|state| *state != requirement.expected_indicator_lights_state)
    {
        visited.extend(current_level_states.iter());

        let next_level_states = current_level_states
            .iter()
            .flat_map(|state| {
                requirement
                    .button_wirings
                    .iter()
                    .map(|button_wiring| {
                        press_button(*state, button_wiring, requirement.total_lights)
                    })
                    .filter(|next_state| !visited.contains(next_state))
            })
            .collect();

        current_level += 1;
        current_level_states = next_level_states;
    }

    current_level
}

fn parse_input(input: &str) -> Vec<MachineRequirements> {
    input
        .trim()
        .lines()
        .map(MachineRequirements::parse_line)
        .collect()
}

fn p1(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .map(|requirements| solve_lights(&requirements))
        .sum::<u64>()
        .to_string()
}

// the Rust z3 crate has very little documentation...
// thankfully, u/ojoelescalon also used z3 (who also just used LLM to convert python's z3 to Rust lol)
// so we can refer to their solution for assistance
// post: https://www.reddit.com/r/adventofcode/comments/1pity70/comment/nta1om6/
fn solve_counters(requirement: &MachineRequirements) -> u64 {
    let optimizer = Optimize::new();
    let presses = (0..requirement.button_wirings.len())
        .map(|button_idx| Int::fresh_const(&format!("button_{}", button_idx)))
        .collect::<Vec<_>>();
    let total_presses = Int::fresh_const("total");

    presses
        .iter()
        .for_each(|press| optimizer.assert(&press.ge(0)));

    requirement
        .joltage_requirements
        .iter()
        .enumerate()
        .for_each(|(counter_idx, joltage)| {
            optimizer.assert(
                &Int::add(
                    &requirement
                        .button_wirings
                        .iter()
                        .enumerate()
                        .filter(|(_, button_wiring)| button_wiring.contains(&counter_idx))
                        .map(|(button_idx, _)| presses[button_idx].clone())
                        .collect::<Vec<_>>(),
                )
                .eq(Int::from_u64(*joltage)),
            );
        });

    optimizer.assert(&total_presses.eq(Int::add(&presses)));
    optimizer.minimize(&total_presses);

    if let SatResult::Sat = optimizer.check(&[]) {
        optimizer
            .get_model()
            .expect("a solution")
            .eval(&total_presses, true)
            .and_then(|total_presses| total_presses.as_u64())
            .expect("a solution")
    } else {
        panic!("Fail to find solution");
    }
}

fn p2(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .map(|requirements| solve_counters(&requirements))
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
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn test_parse_machine_requirements() {
        assert_eq!(
            MachineRequirements::parse_line("[.##..] (3) (1,3) (2) (0,2) {3,5,4,7,11}"),
            MachineRequirements {
                expected_indicator_lights_state: 12,
                total_lights: 5,
                button_wirings: vec![vec![3], vec![1, 3], vec![2], vec![0, 2],],
                joltage_requirements: vec![3, 5, 4, 7, 11]
            }
        );
    }

    #[test]
    fn test_press_button() {
        assert_eq!(press_button(0b0000, &[0], 4), 0b1000);
        assert_eq!(press_button(0b0000, &[1], 4), 0b0100);
        assert_eq!(press_button(0b0000, &[2], 4), 0b0010);
        assert_eq!(press_button(0b0000, &[3], 4), 0b0001);

        assert_eq!(press_button(0b1000, &[0], 4), 0b0000);
        assert_eq!(press_button(0b0100, &[1], 4), 0b0000);
        assert_eq!(press_button(0b0010, &[2], 4), 0b0000);
        assert_eq!(press_button(0b0001, &[3], 4), 0b0000);

        assert_eq!(press_button(0b1111, &[0, 3], 4), 0b0110);
        assert_eq!(press_button(0b1111, &[1, 2, 3], 4), 0b1000);
    }

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "7");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "417");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "33");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "16765");
    }
}

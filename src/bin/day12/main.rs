use std::{collections::HashSet, fmt::Display};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/12/input.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Present {
    spots: u16,
}

impl Present {
    const PRESENT_SIZE: usize = 3;
    const COORD_TO_SPOT: [[u16; Self::PRESENT_SIZE]; Self::PRESENT_SIZE] = [
        [0b1, 0b10, 0b100],
        [0b10_000_000, 0b100_000_000, 0b1_000],
        [0b1_000_000, 0b_100_000, 0b10_000],
    ];

    fn from_str<T: AsRef<str>>(str: T) -> Self {
        Self {
            spots: str
                .as_ref()
                .trim()
                .lines()
                .enumerate()
                .fold(0, |acc, (y, line)| {
                    if y >= Self::COORD_TO_SPOT.len() {
                        panic!("too big in height");
                    }

                    line.trim()
                        .chars()
                        .enumerate()
                        .filter(|(_, ch)| *ch == '#')
                        .fold(acc, |acc, (x, _)| {
                            if x >= Self::COORD_TO_SPOT[y].len() {
                                panic!("too big in width");
                            }

                            acc ^ Self::COORD_TO_SPOT[y][x]
                        })
                }),
        }
    }

    fn rotate_cw90(&self) -> Self {
        let center = self.spots & Self::COORD_TO_SPOT[1][1];
        let back = (self.spots & 0b11_000_000) >> 6;
        let front = (self.spots & 0b00_111_111) << 2;
        Self {
            spots: center | back | front,
        }
    }

    fn flip_updown(&self) -> Self {
        Self {
            spots: (0..Self::PRESENT_SIZE).fold(0, |acc, y| {
                (0..Self::PRESENT_SIZE).fold(acc, |acc, x| {
                    if (self.spots & Self::COORD_TO_SPOT[y][x]) != 0 {
                        acc | Self::COORD_TO_SPOT[Self::PRESENT_SIZE - y - 1][x]
                    } else {
                        acc
                    }
                })
            }),
        }
    }

    fn permutations(&self) -> Vec<Self> {
        [
            *self,
            self.rotate_cw90(),
            self.rotate_cw90().rotate_cw90(),
            self.rotate_cw90().rotate_cw90().rotate_cw90(),
            self.flip_updown(),
            self.flip_updown().rotate_cw90(),
            self.flip_updown().rotate_cw90().rotate_cw90(),
            self.flip_updown().rotate_cw90().rotate_cw90().rotate_cw90(),
        ]
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
    }

    fn area(&self) -> usize {
        self.spots.count_ones() as usize
    }
}

impl Display for Present {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..Self::PRESENT_SIZE)
            .flat_map(|y| {
                let mut inner_results = (0..Self::PRESENT_SIZE)
                    .map(|x| {
                        if (self.spots & Self::COORD_TO_SPOT[y][x]) != 0 {
                            write!(f, "#")?;
                        } else {
                            write!(f, ".")?;
                        }
                        Ok(())
                    })
                    .collect::<Vec<_>>();

                if y != Self::PRESENT_SIZE - 1 {
                    let new_line_result = writeln!(f);
                    inner_results.push(new_line_result);
                }

                inner_results
            })
            .find(|r| r.is_err())
            .unwrap_or(Ok(()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Region {
    size: (usize, usize),
    presents_count: Vec<usize>,
}

impl Region {
    fn from_str<T: AsRef<str>>(str: T) -> Self {
        let (size, presents_count) = str.as_ref().trim().split_once(": ").expect("_x_: _ _ _");
        Self {
            size: size
                .split_once("x")
                .map(|(w, h)| (w.parse().expect("a number"), h.parse().expect("a number")))
                .expect("_x_"),
            presents_count: presents_count
                .split(" ")
                .map(|x| x.parse().expect("a number"))
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Input {
    presents: Vec<Present>,
    regions: Vec<Region>,
}

impl Input {
    fn parse(input: &str) -> Self {
        let input = input.trim().split("\n\n").collect::<Vec<_>>();

        Self {
            presents: input
                .iter()
                .take(input.len() - 1)
                .map(|present_str| {
                    Present::from_str(present_str.split_once("\n").expect("at least two lines").1)
                })
                .collect(),

            regions: input
                .iter()
                .last()
                .expect("at least two sections")
                .trim()
                .lines()
                .map(Region::from_str)
                .collect(),
        }
    }
}

// luckily, our puzzle input never exceeds 64 in width
type RegionGridRow = u64;

#[derive(Debug, PartialEq, Eq, Clone)]
struct RegionGrid {
    x_size: usize,
    grid: Vec<RegionGridRow>,
}

impl RegionGrid {
    const MAX_X_SIZE: usize = std::mem::size_of::<RegionGridRow>() * 8;

    #[allow(dead_code)]
    fn from_str<T: AsRef<str>>(str: T) -> Self {
        let x_size = str
            .as_ref()
            .trim()
            .lines()
            .next()
            .expect("at least one line")
            .len();

        assert!(
            x_size <= Self::MAX_X_SIZE,
            "our assumption that width is maximum of 64 is wrong"
        );

        let grid = str
            .as_ref()
            .trim()
            .lines()
            .map(|line| {
                if line.trim().len() != x_size {
                    panic!(
                        "uneven grid, something wrong with input. row in question: {}",
                        line
                    );
                }

                line.chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch == '#')
                    .fold(0, |acc, (x, _)| acc ^ (1 << x))
            })
            .collect::<Vec<_>>();

        RegionGrid { x_size, grid }
    }
}

impl Display for RegionGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row_cells)| {
                let mut inner_results = (0..self.x_size)
                    .map(|x| {
                        if row_cells & (1 << x) != 0 {
                            write!(f, "#")?;
                        } else {
                            write!(f, ".")?;
                        }
                        Ok(())
                    })
                    .collect::<Vec<_>>();

                if y != self.grid.len() - 1 {
                    let new_line_result = writeln!(f);
                    inner_results.push(new_line_result);
                }

                inner_results
            })
            .find(|r| r.is_err())
            .unwrap_or(Ok(()))
    }
}

impl RegionGrid {
    fn new(size: (usize, usize)) -> Self {
        assert!(
            size.0 <= Self::MAX_X_SIZE,
            "our assumption that width is maximum of 64 is wrong"
        );

        RegionGrid {
            x_size: size.0,
            grid: std::iter::repeat_n(0, size.1).collect(),
        }
    }

    fn can_plant(&self, present: &Present, coord: (usize, usize)) -> bool {
        (coord.0 + Present::PRESENT_SIZE <= self.x_size)
            && (coord.1 + Present::PRESENT_SIZE <= self.grid.len())
            && (0..Present::PRESENT_SIZE).all(|y| {
                let row_cell = self.grid[coord.1 + y];

                (0..Present::PRESENT_SIZE).all(|x| {
                    (present.spots & Present::COORD_TO_SPOT[y][x] == 0)
                        || (row_cell & (1 << (coord.0 + x)) == 0)
                })
            })
    }

    fn plant(&mut self, present: &Present, coord: (usize, usize)) {
        if !self.can_plant(present, coord) {
            panic!("Something is in the way of our planting; cannot plant");
        }

        (0..Present::PRESENT_SIZE).for_each(|y| {
            let row_idx = coord.1 + y;
            (0..Present::PRESENT_SIZE)
                .filter(|x| present.spots & Present::COORD_TO_SPOT[y][*x] != 0)
                .for_each(|x| {
                    let col_idx = coord.0 + x;
                    self.grid[row_idx] |= 1 << col_idx;
                });
        });
    }

    fn can_unplant(&self, present: &Present, coord: (usize, usize)) -> bool {
        (coord.0 + Present::PRESENT_SIZE <= self.x_size)
            && (coord.1 + Present::PRESENT_SIZE <= self.grid.len())
            && (0..Present::PRESENT_SIZE).all(|y| {
                let row_cell = self.grid[coord.1 + y];

                (0..Present::PRESENT_SIZE).all(|x| {
                    (present.spots & Present::COORD_TO_SPOT[y][x] == 0)
                        || (row_cell & (1 << (coord.0 + x)) != 0)
                })
            })
    }

    fn unplant(&mut self, present: &Present, coord: (usize, usize)) {
        if !self.can_unplant(present, coord) {
            panic!(
                "Spot is missing existing present, cannot unplant. You should only use unplant() to reverse a plant()"
            );
        }

        (0..Present::PRESENT_SIZE).for_each(|y| {
            let row_idx = coord.1 + y;
            (0..Present::PRESENT_SIZE)
                .filter(|x| present.spots & Present::COORD_TO_SPOT[y][*x] != 0)
                .for_each(|x| {
                    let col_idx = coord.0 + x;
                    self.grid[row_idx] ^= 1 << col_idx;
                });
        });
    }
}

fn can_fit(region: &Region, presents: &[Present], is_troll_input: bool) -> bool {
    let presents_needed_area = presents
        .iter()
        .map(|present| present.area())
        .zip(region.presents_count.iter())
        .map(|(area, count)| area * (*count))
        .sum::<usize>();

    if region.size.0 * region.size.1 < presents_needed_area {
        return false;
    }

    // part 1 actual input is a troll input...
    // the input is carefully crafted such that, as long as the region's area is bigger
    // than the area of all presents combined, there will always be a solution, so there's
    // no need to go through DFS, just return true here. This obviously is not always true,
    // hence part 1 actual input is troll because we aren't told about this special behavior,
    // so we are tricked to implement DFS to handle it.
    //
    // note that part 1 sample input is not troll input, so will still need the DFS
    // logic
    if is_troll_input {
        return true;
    }

    fn try_fit_next(
        region: &Region,
        presents: &[Present],
        region_grid: &mut RegionGrid,
        current_presents_count: &mut Vec<usize>,
    ) -> bool {
        match current_presents_count
            .iter()
            .enumerate()
            .find(|(present_idx, present_count)| {
                **present_count < region.presents_count[*present_idx]
            })
            .map(|(present_idx, _)| present_idx)
        {
            None => true,
            Some(present_idx) => presents[present_idx].permutations().iter().any(|present| {
                (0..=(region.size.0 - Present::PRESENT_SIZE)).any(|x| {
                    (0..=(region.size.1 - Present::PRESENT_SIZE)).any(|y| {
                        let coord = (x, y);

                        if region_grid.can_plant(present, coord) {
                            region_grid.plant(present, coord);
                            current_presents_count[present_idx] += 1;

                            let result =
                                try_fit_next(region, presents, region_grid, current_presents_count);

                            region_grid.unplant(present, coord);
                            current_presents_count[present_idx] -= 1;

                            result
                        } else {
                            false
                        }
                    })
                })
            }),
        }
    }

    try_fit_next(
        region,
        presents,
        &mut RegionGrid::new(region.size),
        &mut std::iter::repeat_n(0, presents.len()).collect(),
    )
}

fn solve_p1(input: &str, is_troll_input: bool) -> String {
    let input = Input::parse(input);
    input
        .regions
        .iter()
        .filter(|region| can_fit(region, &input.presents, is_troll_input))
        .count()
        .to_string()
}

fn p1(input: &str) -> String {
    solve_p1(input, true)
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[test]
    fn test_present_from_str() {
        [
            (
                r"
...
...
...
",
                0,
            ),
            (
                r"
#..
...
...
",
                0b1,
            ),
            (
                r"
.#.
...
...
",
                0b10,
            ),
            (
                r"
..#
...
...
",
                0b100,
            ),
            (
                r"
...
..#
...
",
                0b1_000,
            ),
            (
                r"
...
...
..#
",
                0b10_000,
            ),
            (
                r"
...
...
.#.
",
                0b100_000,
            ),
            (
                r"
...
...
#..
",
                0b1_000_000,
            ),
            (
                r"
...
#..
...
",
                0b10_000_000,
            ),
            (
                r"
...
.#.
...
",
                0b100_000_000,
            ),
            (
                r"
..#
##.
..#
",
                0b110_010_100,
            ),
        ]
        .into_iter()
        .for_each(|(input, expected_spots)| {
            assert_eq!(
                Present::from_str(input),
                Present {
                    spots: expected_spots
                },
                "{} spots should be: {}",
                input,
                expected_spots
            );
        });
    }

    #[test]
    fn test_present_rotate_cw90() {
        [
            (
                Present::from_str(
                    r"
#..
...
...
",
                ),
                Present::from_str(
                    r"
..#
...
...
",
                ),
            ),
            (
                Present::from_str(
                    r"
.#.
...
...
",
                ),
                Present::from_str(
                    r"
...
..#
...
",
                ),
            ),
            (
                Present::from_str(
                    r"
..#
...
...
",
                ),
                Present::from_str(
                    r"
...
...
..#
",
                ),
            ),
            (
                Present::from_str(
                    r"
...
#..
...
",
                ),
                Present::from_str(
                    r"
.#.
...
...
",
                ),
            ),
            (
                Present::from_str(
                    r"
...
.#.
...
",
                ),
                Present::from_str(
                    r"
...
.#.
...
",
                ),
            ),
            (
                Present::from_str(
                    r"
...
..#
...
",
                ),
                Present::from_str(
                    r"
...
...
.#.
",
                ),
            ),
            (
                Present::from_str(
                    r"
...
...
#..
",
                ),
                Present::from_str(
                    r"
#..
...
...
",
                ),
            ),
            (
                Present::from_str(
                    r"
...
...
.#.
",
                ),
                Present::from_str(
                    r"
...
#..
...
",
                ),
            ),
            (
                Present::from_str(
                    r"
...
...
..#
",
                ),
                Present::from_str(
                    r"
...
...
#..
",
                ),
            ),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            assert_eq!(
                input.rotate_cw90(),
                expected,
                "\n{}\n--to--\n{}\n",
                input,
                expected
            );
        });
    }

    #[test]
    fn test_present_flip_updown() {
        [
            (
                Present::from_str(
                    r"
#..
...
...
",
                ),
                Present::from_str(
                    r"
...
...
#..
",
                ),
            ),
            (
                Present::from_str(
                    r"
.#.
...
...
",
                ),
                Present::from_str(
                    r"
...
...
.#.
",
                ),
            ),
            (
                Present::from_str(
                    r"
..#
...
...
",
                ),
                Present::from_str(
                    r"
...
...
..#
",
                ),
            ),
            (
                Present::from_str(
                    r"
...
#..
...
",
                ),
                Present::from_str(
                    r"
...
#..
...
",
                ),
            ),
            (
                Present::from_str(
                    r"
...
.#.
...
",
                ),
                Present::from_str(
                    r"
...
.#.
...
",
                ),
            ),
            (
                Present::from_str(
                    r"
...
..#
...
",
                ),
                Present::from_str(
                    r"
...
..#
...
",
                ),
            ),
            (
                Present::from_str(
                    r"
...
...
#..
",
                ),
                Present::from_str(
                    r"
#..
...
...
",
                ),
            ),
            (
                Present::from_str(
                    r"
...
...
.#.
",
                ),
                Present::from_str(
                    r"
.#.
...
...
",
                ),
            ),
            (
                Present::from_str(
                    r"
...
...
..#
",
                ),
                Present::from_str(
                    r"
..#
...
...
",
                ),
            ),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            assert_eq!(
                input.flip_updown(),
                expected,
                "\n{}\n--to--\n{}\n",
                input,
                expected
            );
        });
    }

    #[test]
    fn test_present_permutations() {
        [
            (
                Present::from_str(
                    r"
#..
##.
...
",
                ),
                vec![
                    Present::from_str(
                        r"
#..
##.
...
",
                    ),
                    Present::from_str(
                        r"
.##
.#.
...
",
                    ),
                    Present::from_str(
                        r"
...
.##
..#
",
                    ),
                    Present::from_str(
                        r"
...
.#.
##.
",
                    ),
                    Present::from_str(
                        r"
..#
.##
...
",
                    ),
                    Present::from_str(
                        r"
...
.#.
.##
",
                    ),
                    Present::from_str(
                        r"
...
##.
#..
",
                    ),
                    Present::from_str(
                        r"
##.
.#.
...
",
                    ),
                ],
            ),
            (
                Present::from_str(
                    r"
...
.#.
...
",
                ),
                vec![Present::from_str(
                    r"
...
.#.
...
",
                )],
            ),
            (
                Present::from_str(
                    r"
.#.
.#.
.#.
",
                ),
                vec![
                    Present::from_str(
                        r"
.#.
.#.
.#.
",
                    ),
                    Present::from_str(
                        r"
...
###
...
",
                    ),
                ],
            ),
            (
                Present::from_str(
                    r"
...
###
...
",
                ),
                vec![
                    Present::from_str(
                        r"
.#.
.#.
.#.
",
                    ),
                    Present::from_str(
                        r"
...
###
...
",
                    ),
                ],
            ),
            (
                Present::from_str(
                    r"
#..
.#.
..#
",
                ),
                vec![
                    Present::from_str(
                        r"
#..
.#.
..#
",
                    ),
                    Present::from_str(
                        r"
..#
.#.
#..
",
                    ),
                ],
            ),
            (
                Present::from_str(
                    r"
.#.
.#.
...
",
                ),
                vec![
                    Present::from_str(
                        r"
.#.
.#.
...
",
                    ),
                    Present::from_str(
                        r"
...
.##
...
",
                    ),
                    Present::from_str(
                        r"
...
.#.
.#.
",
                    ),
                    Present::from_str(
                        r"
...
##.
...
",
                    ),
                ],
            ),
            (
                Present::from_str(
                    r"
.#.
...
...
",
                ),
                vec![
                    Present::from_str(
                        r"
.#.
...
...
",
                    ),
                    Present::from_str(
                        r"
...
..#
...
",
                    ),
                    Present::from_str(
                        r"
...
...
.#.
",
                    ),
                    Present::from_str(
                        r"
...
#..
...
",
                    ),
                ],
            ),
            (
                Present::from_str(
                    r"
..#
...
...
",
                ),
                vec![
                    Present::from_str(
                        r"
..#
...
...
",
                    ),
                    Present::from_str(
                        r"
...
...
..#
",
                    ),
                    Present::from_str(
                        r"
...
...
#..
",
                    ),
                    Present::from_str(
                        r"
#..
...
...
",
                    ),
                ],
            ),
            (
                Present::from_str(
                    r"
...
...
...
",
                ),
                vec![Present::from_str(
                    r"
...
...
...
",
                )],
            ),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let expected = expected.iter().copied().collect::<HashSet<_>>();
            let actual = input.permutations();

            assert_eq!(actual.len(), expected.len(), "permutation count");
            assert_eq!(
                actual.into_iter().collect::<HashSet<_>>(),
                expected,
                "\n{}\n\n expects: {:?}",
                input,
                expected,
            );
        });
    }

    #[test]
    fn test_present_area() {
        [
            (
                r"
...
...
...
",
                0,
            ),
            (
                r"
#..
...
...
",
                1,
            ),
            (
                r"
.#.
...
...
",
                1,
            ),
            (
                r"
..#
...
...
",
                1,
            ),
            (
                r"
...
#..
...
",
                1,
            ),
            (
                r"
...
.#.
...
",
                1,
            ),
            (
                r"
...
..#
...
",
                1,
            ),
            (
                r"
...
...
#..
",
                1,
            ),
            (
                r"
...
...
.#.
",
                1,
            ),
            (
                r"
...
...
..#
",
                1,
            ),
            (
                r"
###
###
###
",
                9,
            ),
            (
                r"
###
###
##.
",
                8,
            ),
            (
                r"
.##
###
###
",
                8,
            ),
            (
                r"
.##
###
##.
",
                7,
            ),
            (
                r"
.#.
###
.#.
",
                5,
            ),
            (
                r"
.#.
#.#
.#.
",
                4,
            ),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            assert_eq!(Present::from_str(input).area(), expected, "{}", input);
        });
    }

    #[test]
    fn test_present_to_string() {
        [
            r"
...
...
...
",
            r"
#..
...
...
",
            r"
.#.
...
...
",
            r"
..#
...
...
",
            r"
...
#..
...
",
            r"
...
.#.
...
",
            r"
...
..#
...
",
            r"
...
...
#..
",
            r"
...
...
.#.
",
            r"
...
...
..#
",
            r"
..#
##.
..#
",
        ]
        .into_iter()
        .for_each(|input| {
            assert_eq!(
                Present::from_str(input).to_string(),
                input.trim().to_string(),
                "{}",
                input,
            );
        });
    }

    #[test]
    fn test_region_from_str() {
        assert_eq!(
            Region::from_str("12x5: 1 0 1 0 2 2"),
            Region {
                size: (12, 5),
                presents_count: vec![1, 0, 1, 0, 2, 2]
            }
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            Input::parse(SAMPLE_INPUT),
            Input {
                presents: vec![
                    Present::from_str(
                        r"
###
##.
##.
"
                    ),
                    Present::from_str(
                        r"
###
##.
.##
"
                    ),
                    Present::from_str(
                        r"
.##
###
##.
"
                    ),
                    Present::from_str(
                        r"
##.
###
##.
"
                    ),
                    Present::from_str(
                        r"
###
#..
###
"
                    ),
                    Present::from_str(
                        r"
###
.#.
###
"
                    ),
                ],
                regions: vec![
                    Region::from_str("4x4: 0 0 0 0 2 0"),
                    Region::from_str("12x5: 1 0 1 0 2 2"),
                    Region::from_str("12x5: 1 0 1 0 3 2"),
                ]
            }
        );
    }

    #[test]
    fn test_region_grid_from_str() {
        [
            (
                r"
...
...
...
",
                RegionGrid {
                    x_size: 3,
                    grid: vec![0, 0, 0],
                },
            ),
            (
                r"
#...
....
....
",
                RegionGrid {
                    x_size: 4,
                    grid: vec![0b1, 0, 0],
                },
            ),
            (
                r"
.#..
....
....
",
                RegionGrid {
                    x_size: 4,
                    grid: vec![0b10, 0, 0],
                },
            ),
            (
                r"
..#.
....
....
",
                RegionGrid {
                    x_size: 4,
                    grid: vec![0b100, 0, 0],
                },
            ),
            (
                r"
...#
....
....
",
                RegionGrid {
                    x_size: 4,
                    grid: vec![0b1_000, 0, 0],
                },
            ),
            (
                r"
....
#...
....
",
                RegionGrid {
                    x_size: 4,
                    grid: vec![0, 0b1, 0],
                },
            ),
            (
                r"
....
.#..
....
",
                RegionGrid {
                    x_size: 4,
                    grid: vec![0, 0b10, 0],
                },
            ),
            (
                r"
....
..#.
....
",
                RegionGrid {
                    x_size: 4,
                    grid: vec![0, 0b100, 0],
                },
            ),
            (
                r"
....
...#
....
",
                RegionGrid {
                    x_size: 4,
                    grid: vec![0, 0b1_000, 0],
                },
            ),
            (
                r"
.##.
#..#
.###
",
                RegionGrid {
                    x_size: 4,
                    grid: vec![0b110, 0b1_001, 0b1_110],
                },
            ),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            assert_eq!(
                RegionGrid::from_str(input),
                expected,
                "{}, expected:\n{:?}",
                input,
                expected
            );
        });
    }

    #[test]
    fn test_region_grid_to_string() {
        [
            r"
....
....
....
",
            r"
#...
....
....
",
            r"
.#..
....
....
",
            r"
..#.
....
....
",
            r"
...#
....
....
",
            r"
....
#...
....
",
            r"
....
.#..
....
",
            r"
....
..#.
....
",
            r"
....
...#
....
",
            r"
.##.
#..#
.###
",
        ]
        .into_iter()
        .for_each(|input| {
            assert_eq!(
                RegionGrid::from_str(input).to_string(),
                input.trim(),
                "{}",
                input,
            );
        });
    }

    #[test]
    fn test_region_grid_new() {
        assert_eq!(
            RegionGrid::new((3, 4)),
            RegionGrid::from_str(
                r"
...
...
...
...
"
            )
        );
    }

    #[test]
    fn test_region_grid_can_plant() {
        [
            (
                r"
....
....
....
....
",
                r"
###
###
###
",
                (0, 0),
                true,
            ),
            (
                r"
....
....
....
....
",
                r"
###
###
###
",
                (0, 1),
                true,
            ),
            (
                r"
....
....
....
....
",
                r"
###
###
###
",
                (1, 0),
                true,
            ),
            (
                r"
....
....
....
....
",
                r"
###
###
###
",
                (0, 2),
                false,
            ),
            (
                r"
....
....
....
....
",
                r"
###
###
###
",
                (2, 0),
                false,
            ),
            (
                r"
....
....
....
....
",
                r"
###
###
###
",
                (1, 1),
                true,
            ),
            (
                r"
....
....
....
....
",
                r"
###
###
###
",
                (2, 2),
                false,
            ),
            (
                r"
....
....
....
....
",
                r"
###
###
###
",
                (2, 1),
                false,
            ),
            (
                r"
....
....
....
....
",
                r"
###
###
###
",
                (1, 2),
                false,
            ),
            (
                r"
...
...
...
",
                r"
###
###
###
",
                (0, 0),
                true,
            ),
            (
                r"
#..
...
...
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
.#.
...
...
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
..#
...
...
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
...
#..
...
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
...
.#.
...
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
...
..#
...
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
...
...
#..
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
...
...
.#.
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
...
...
..#
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
###
###
###
",
                r"
...
...
...
",
                (0, 0),
                true,
            ),
            (
                r"
#..
...
...
",
                r"
.##
###
###
",
                (0, 0),
                true,
            ),
            (
                r"
.#.
...
...
",
                r"
#.#
###
###
",
                (0, 0),
                true,
            ),
            (
                r"
..#
...
...
",
                r"
##.
###
###
",
                (0, 0),
                true,
            ),
            (
                r"
...
#..
...
",
                r"
###
.##
###
",
                (0, 0),
                true,
            ),
            (
                r"
...
.#.
...
",
                r"
###
#.#
###
",
                (0, 0),
                true,
            ),
            (
                r"
...
..#
...
",
                r"
###
##.
###
",
                (0, 0),
                true,
            ),
            (
                r"
...
...
#..
",
                r"
###
###
.##
",
                (0, 0),
                true,
            ),
            (
                r"
...
...
.#.
",
                r"
###
###
#.#
",
                (0, 0),
                true,
            ),
            (
                r"
...
...
..#
",
                r"
###
###
##.
",
                (0, 0),
                true,
            ),
            (
                r"
#...
#...
#...
",
                r"
###
###
###
",
                (1, 0),
                true,
            ),
            (
                r"
###
...
...
...
",
                r"
###
###
###
",
                (0, 1),
                true,
            ),
            (
                r"
#####
##.##
#...#
##.##
#####
",
                r"
.#.
###
.#.
",
                (1, 1),
                true,
            ),
            (
                r"
#####
##.##
#.#.#
##.##
#####
",
                r"
.#.
###
.#.
",
                (1, 1),
                false,
            ),
        ]
        .into_iter()
        .for_each(|(grid, present, coord, expected)| {
            assert_eq!(
                RegionGrid::from_str(grid).can_plant(&Present::from_str(present), coord),
                expected,
                "region: {}\npresent: {}\ncoord: ({}, {})",
                grid,
                present,
                coord.0,
                coord.1,
            );
        });
    }

    #[test]
    fn test_region_grid_plant() {
        [
            (
                r"
......
......
......
......
......
",
                r"
###
###
###
",
                (1, 2),
                r"
......
......
.###..
.###..
.###..
",
            ),
            (
                r"
......
......
......
......
......
",
                r"
.##
##.
.#.
",
                (1, 2),
                r"
......
......
..##..
.##...
..#...
",
            ),
        ]
        .into_iter()
        .for_each(|(grid, present, coord, expected)| {
            let mut region_grid = RegionGrid::from_str(grid);
            region_grid.plant(&Present::from_str(present), coord);

            assert_eq!(
                region_grid.to_string(),
                expected.trim(),
                "region: {}\npresent: {}\ncoord: ({}, {})",
                grid,
                present,
                coord.0,
                coord.1,
            );
        });
    }

    #[test]
    fn test_region_grid_can_unplant() {
        [
            (
                r"
###.
###.
###.
....
",
                r"
###
###
###
",
                (0, 0),
                true,
            ),
            (
                r"
....
###.
###.
###.
",
                r"
###
###
###
",
                (0, 1),
                true,
            ),
            (
                r"
.###
.###
.###
....
",
                r"
###
###
###
",
                (1, 0),
                true,
            ),
            (
                r"
####
####
####
####
",
                r"
###
###
###
",
                (0, 2),
                false,
            ),
            (
                r"
####
####
####
####
",
                r"
###
###
###
",
                (2, 0),
                false,
            ),
            (
                r"
####
####
####
####
",
                r"
###
###
###
",
                (1, 1),
                true,
            ),
            (
                r"
####
####
####
####
",
                r"
###
###
###
",
                (2, 2),
                false,
            ),
            //
            (
                r"
####
####
####
####
",
                r"
###
###
###
",
                (2, 1),
                false,
            ),
            (
                r"
####
####
####
####
",
                r"
###
###
###
",
                (1, 2),
                false,
            ),
            (
                r"
###
###
###
",
                r"
###
###
###
",
                (0, 0),
                true,
            ),
            (
                r"
.##
###
###
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
#.#
###
###
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
##.
###
###
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
###
.##
###
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
###
#.#
###
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
###
##.
###
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
###
###
.##
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
###
###
#.#
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
###
###
##.
",
                r"
###
###
###
",
                (0, 0),
                false,
            ),
            (
                r"
###
###
###
",
                r"
...
...
...
",
                (0, 0),
                true,
            ),
            (
                r"
.##
###
###
",
                r"
.##
###
###
",
                (0, 0),
                true,
            ),
            (
                r"
#.#
###
###
",
                r"
#.#
###
###
",
                (0, 0),
                true,
            ),
            (
                r"
##.
###
###
",
                r"
##.
###
###
",
                (0, 0),
                true,
            ),
            (
                r"
###
.##
###
",
                r"
###
.##
###
",
                (0, 0),
                true,
            ),
            (
                r"
###
#.#
###
",
                r"
###
#.#
###
",
                (0, 0),
                true,
            ),
            (
                r"
###
##.
###
",
                r"
###
##.
###
",
                (0, 0),
                true,
            ),
            (
                r"
###
###
.##
",
                r"
###
###
.##
",
                (0, 0),
                true,
            ),
            (
                r"
###
###
#.#
",
                r"
###
###
#.#
",
                (0, 0),
                true,
            ),
            (
                r"
###
###
##.
",
                r"
###
###
##.
",
                (0, 0),
                true,
            ),
            (
                r"
.###
.###
.###
",
                r"
###
###
###
",
                (1, 0),
                true,
            ),
            (
                r"
...
###
###
###
",
                r"
###
###
###
",
                (0, 1),
                true,
            ),
            (
                r"
.....
..#..
.###.
..#..
.....
",
                r"
.#.
###
.#.
",
                (1, 1),
                true,
            ),
            (
                r"
.....
..#..
.#.#.
..#..
.....
",
                r"
.#.
###
.#.
",
                (1, 1),
                false,
            ),
        ]
        .into_iter()
        .for_each(|(grid, present, coord, expected)| {
            assert_eq!(
                RegionGrid::from_str(grid).can_unplant(&Present::from_str(present), coord),
                expected,
                "region: {}\npresent: {}\ncoord: ({}, {})",
                grid,
                present,
                coord.0,
                coord.1,
            );
        });
    }

    #[test]
    fn test_region_grid_unplant() {
        [
            (
                r"
######
######
######
######
######
",
                r"
###
###
###
",
                (1, 2),
                r"
######
######
#...##
#...##
#...##
",
            ),
            (
                r"
######
######
######
######
######
",
                r"
.##
##.
.#.
",
                (1, 2),
                r"
######
######
##..##
#..###
##.###
",
            ),
            (
                r"
......
......
..##..
.##...
..#...
",
                r"
.##
##.
.#.
",
                (1, 2),
                r"
......
......
......
......
......
",
            ),
        ]
        .into_iter()
        .for_each(|(grid, present, coord, expected)| {
            let mut region_grid = RegionGrid::from_str(grid);
            region_grid.unplant(&Present::from_str(present), coord);

            assert_eq!(
                region_grid.to_string(),
                expected.trim(),
                "region: {}\npresent: {}\ncoord: ({}, {})",
                grid,
                present,
                coord.0,
                coord.1,
            );
        });
    }

    #[test]
    fn test_can_fit() {
        let input = Input::parse(SAMPLE_INPUT);

        // test with is_troll_input: false, because sample input
        // is not troll input
        assert!(can_fit(&input.regions[0], &input.presents, false));
        assert!(can_fit(&input.regions[1], &input.presents, false));
        assert!(!can_fit(&input.regions[2], &input.presents, false));
    }

    #[test]
    fn test_p1_sample() {
        // test with is_troll_input: false, because sample input
        // is not troll input
        assert_eq!(solve_p1(SAMPLE_INPUT, false), "2");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "427");
    }

    #[test]
    fn test_p1_actual_treat_as_non_troll() {
        // to ensure the correctness of our implementation,
        // treat actual input as non-troll, even though
        // it is
        assert_eq!(solve_p1(ACTUAL_INPUT, false), "427");
    }
}

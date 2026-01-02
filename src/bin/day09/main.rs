const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/09/input.txt");

type Point = (i64, i64);

fn parse_input(input: &str) -> Vec<Point> {
    input
        .trim()
        .lines()
        .map(|line| line.split_once(",").expect("x,x"))
        .map(|coord| {
            (
                coord.0.parse().expect("a number"),
                coord.1.parse().expect("a number"),
            )
        })
        .collect()
}

fn area(a: &Point, b: &Point) -> i64 {
    ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1)
}

fn p1(input: &str) -> String {
    let coords = parse_input(input);
    coords
        .iter()
        .map(|coord| {
            coords
                .iter()
                .map(|other| area(coord, other))
                .max()
                .expect("always have at least one area")
        })
        .max()
        .expect("always have an answer")
        .to_string()
}

#[derive(Debug)]
struct Rect {
    // preserved for debugging purposes, so that
    // we can track back the original points
    a: Point,
    b: Point,

    // computed from a & b
    left: i64,
    right: i64,
    top: i64,
    bottom: i64,
}

impl Rect {
    fn new(a: &Point, b: &Point) -> Self {
        Self {
            a: *a,
            b: *b,
            left: a.0.min(b.0),
            right: a.0.max(b.0),
            top: a.1.min(b.1),
            bottom: a.1.max(b.1),
        }
    }

    fn has_line(&self, line: &Line) -> bool {
        match line {
            Line::Horizontal {
                y: line_y,
                left: line_left,
                right: line_right,
            } => {
                !(*line_y <= self.top
                    || *line_y >= self.bottom
                    || *line_right < self.left
                    || *line_left > self.right)
            }
            Line::Vertical {
                x: line_x,
                top: line_top,
                bottom: line_bottom,
            } => {
                !(*line_x <= self.left
                    || *line_x >= self.right
                    || *line_bottom < self.top
                    || *line_top > self.bottom)
            }
        }
    }

    fn area(&self) -> i64 {
        area(&self.a, &self.b)
    }
}

enum Line {
    Horizontal { y: i64, left: i64, right: i64 },
    Vertical { x: i64, top: i64, bottom: i64 },
}

impl Line {
    fn new(a: &Point, b: &Point) -> Self {
        if a.1 == b.1 {
            Self::Horizontal {
                y: a.1,
                left: a.0.min(b.0),
                right: a.0.max(b.0),
            }
        } else if a.0 == b.0 {
            Self::Vertical {
                x: a.0,
                top: a.1.min(b.1),
                bottom: a.1.max(b.1),
            }
        } else {
            panic!("Cannot handle slanted lines");
        }
    }
}

fn lines(coords: &[Point]) -> Vec<Line> {
    coords
        .iter()
        .zip(coords.iter().skip(1).chain(coords.iter().take(1)))
        .map(|(a, b)| Line::new(a, b))
        .collect()
}

fn p2(input: &str) -> String {
    let coords = parse_input(input);
    let lines = lines(&coords);

    dbg!(
        coords
            .iter()
            .flat_map(|coord| {
                coords
                    .iter()
                    .filter(|other| *other != coord)
                    .map(|other| Rect::new(coord, other))
                    .filter(|rect| lines.iter().all(|line| !rect.has_line(line)))
                    // .map(|rect| rect.area())
                    .max_by(|x, y| x.area().cmp(&y.area()))
                // .max()
            })
            // .max()
            .max_by(|x, y| x.area().cmp(&y.area()))
            .expect("always have an answer")
    )
    .area()
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
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "50");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "4741848414");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "24");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1508918480");
    }
}

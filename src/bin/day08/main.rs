use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use union_find::{QuickFindUf, QuickUnionUf, UnionBySize, UnionFind};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/08/input.txt");

type Coord = (i64, i64, i64);

fn parse_input(input: &str) -> Vec<Coord> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut iter = line.split(",").map(|x| x.parse().expect("a number"));

            (
                iter.next().expect("3 entries"),
                iter.next().expect("3 entries"),
                iter.next().expect("3 entries"),
            )
        })
        .collect()
}

fn dist_squared(a: &Coord, b: &Coord) -> i64 {
    (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2)
}

// a heap is faster than a sorted vec, because not all contents of the heap
// will be consumed. So if we only consume k -> O(n) for heapify + O(k lg n),
// whereas a vec will cost O(n lg n) even if we only consume k.
fn compute_dists(coords: &[Coord]) -> BinaryHeap<(Reverse<i64>, usize, usize)> {
    coords
        .iter()
        .enumerate()
        .flat_map(|(current_idx, current)| {
            coords
                .iter()
                .enumerate()
                .skip(current_idx + 1)
                .map(move |(entry_idx, entry)| {
                    (
                        Reverse(dist_squared(current, entry)),
                        current_idx,
                        entry_idx,
                    )
                })
        })
        .collect()
}

fn solve_p1(input: &str, connections: usize) -> i64 {
    let coords = parse_input(input);
    let mut dists = compute_dists(&coords);
    let mut ufs = QuickFindUf::<UnionBySize>::new(coords.len());

    (0..connections).for_each(|_| {
        let next = dists.pop().expect("still have a candidate");
        ufs.union(next.1, next.2);
    });

    let mut collections_size = HashMap::new();

    (0..coords.len()).for_each(|idx| {
        let collection = ufs.find(idx);
        *collections_size.entry(collection).or_insert(0) += 1;
    });

    // because sizes is expected to be small, it doesn't matter if we use a vec or
    // a heap here - there's no observable speedup even if we switch to a heap
    let mut sizes = collections_size.values().map(Reverse).collect::<Vec<_>>();
    sizes.sort_unstable();
    sizes.into_iter().take(3).map(|size| size.0).product()
}

fn p1(input: &str) -> String {
    solve_p1(input, 1000).to_string()
}

fn p2(input: &str) -> String {
    let coords = parse_input(input);
    let mut dists = compute_dists(&coords);
    let mut ufs = QuickUnionUf::<UnionBySize>::new(coords.len());

    let mut used = HashSet::new();

    loop {
        let next = dists
            .pop()
            .expect("puzzle should have a solution before we exhaust everything");

        used.insert(next.1);
        used.insert(next.2);

        ufs.union(next.1, next.2);

        if used.len() == coords.len() {
            return (coords[next.1].0 * coords[next.2].0).to_string();
        }
    }
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(solve_p1(SAMPLE_INPUT, 10), 40);
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "24360");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "25272");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "2185817796");
    }
}

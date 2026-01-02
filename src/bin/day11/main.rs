use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2025/11/input.txt");

struct Graph(HashMap<String, Vec<String>>);
struct ReverseDAG(HashMap<String, Vec<String>>);

fn parse_input(input: &str) -> Graph {
    Graph(
        input
            .trim()
            .lines()
            .map(|line| {
                let (node, children) = line.split_once(": ").expect("xx: xx xx");
                (
                    node.to_string(),
                    children.split(" ").map(|child| child.to_string()).collect(),
                )
            })
            .collect(),
    )
}

fn reverse_graph(graph: &Graph) -> ReverseDAG {
    let mut new_graph: HashMap<String, Vec<String>> = HashMap::new();

    graph.0.iter().for_each(|(node, children)| {
        children.iter().for_each(|child| {
            new_graph
                .entry(child.to_string())
                .or_default()
                .push(node.to_string());
        });
    });

    ReverseDAG(new_graph)
}

fn traverse_path(graph: &ReverseDAG, path: &[&str]) -> u64 {
    fn traverse_a_to_b(graph: &ReverseDAG, src: &str, src_count: u64, dest: &str) -> u64 {
        fn visit<'a>(
            graph: &'a ReverseDAG,
            node_count: &mut HashMap<&'a str, u64>,
            current_node: &'a str,
        ) -> u64 {
            if let Some(count) = node_count.get(current_node) {
                return *count;
            }

            let count = {
                match graph.0.get(current_node) {
                    Some(children) => children
                        .iter()
                        .map(|child| visit(graph, node_count, child))
                        .sum(),
                    None => 0,
                }
            };

            node_count.insert(current_node, count);
            count
        }

        visit(graph, &mut [(src, src_count)].into_iter().collect(), dest)
    }

    path.iter()
        .zip(path.iter().skip(1))
        .fold(1, |acc, (src, dest)| traverse_a_to_b(graph, src, acc, dest))
}

const YOU: &str = "you";
const SVR: &str = "svr";
const DAC: &str = "dac";
const FFT: &str = "fft";
const OUT: &str = "out";

fn p1(input: &str) -> String {
    let graph = reverse_graph(&parse_input(input));
    traverse_path(&graph, &[YOU, OUT]).to_string()
}

fn p2(input: &str) -> String {
    let graph = reverse_graph(&parse_input(input));

    [[SVR, DAC, FFT, OUT], [SVR, FFT, DAC, OUT]]
        .into_iter()
        .map(|path| traverse_path(&graph, &path))
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

    const SAMPLE_INPUT_P1: &str = r"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT_P1), "5");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "566");
    }

    const SAMPLE_INPUT_P2: &str = r"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT_P2), "2");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "331837854931968");
    }
}

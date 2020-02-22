use regex::Regex;
use std::collections::HashMap;
use std::fs;

struct Edge {
    name: String,
    distance: u32,
}

type Graph = HashMap<String, Vec<Edge>>;

fn add_edge(graph: &mut Graph, from: String, to: String, distance: u32) {
    let edge = graph.entry(from).or_insert(Vec::new());
    (*edge).push(Edge { name: to, distance });
}

fn create_graph(input: &str) -> Graph {
    let mut graph: Graph = Graph::new();

    let re = Regex::new(r"^(?P<node_1>\w+) to (?P<node_2>\w+) = (?P<distance>\d+)$").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let node_1 = caps["node_1"].to_string();
        let node_2 = caps["node_2"].to_string();
        let distance = caps["distance"].parse::<u32>().unwrap();

        add_edge(&mut graph, node_1.clone(), node_2.clone(), distance);
        add_edge(&mut graph, node_2.clone(), node_1.clone(), distance);
    }

    graph
}

fn traverse<ComparePolicy>(
    graph: &Graph,
    node_name: &str,
    visited: &mut Vec<String>,
    distance: u32,
    result: &mut u32,
    compare: &ComparePolicy,
) where
    ComparePolicy: Fn(u32, u32) -> bool,
{
    visited.push(String::from(node_name));
    if visited.len() == graph.len() {
        if compare(distance, *result) {
            *result = distance;
        }
    } else {
        let edges = graph.get(node_name).unwrap();

        edges
            .iter()
            .filter(|edge| !visited.contains(&edge.name))
            .for_each(|edge| {
                traverse(
                    &graph,
                    &edge.name,
                    &mut visited.clone(),
                    distance + edge.distance,
                    result,
                    &*compare,
                );
            });
    }
}

fn part_1(graph: &Graph) -> u32 {
    let less_than = |lhs: u32, rhs: u32| lhs < rhs;

    let mut shortest_path = std::u32::MAX;
    graph.keys().for_each(|starting_node| {
        traverse(
            &graph,
            &starting_node,
            &mut Vec::new(),
            0,
            &mut shortest_path,
            &less_than,
        );
    });
    shortest_path
}

fn part_2(graph: &Graph) -> u32 {
    let greater_than = |lhs: u32, rhs: u32| lhs > rhs;

    let mut longest_path = std::u32::MIN;
    graph.keys().for_each(|starting_node| {
        traverse(
            &graph,
            &starting_node,
            &mut Vec::new(),
            0,
            &mut longest_path,
            &greater_than,
        );
    });
    longest_path
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    let graph = create_graph(&content);

    assert_eq!(251, part_1(&graph));
    assert_eq!(898, part_2(&graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "London to Dublin = 464\n\
            London to Belfast = 518\n\
            Dublin to Belfast = 141\n";
        let graph = create_graph(&input);
        assert_eq!(605, part_1(&graph));
    }

    #[test]
    fn test_part_2() {
        let input = "London to Dublin = 464\n\
            London to Belfast = 518\n\
            Dublin to Belfast = 141\n";
        let graph = create_graph(&input);
        assert_eq!(982, part_2(&graph));
    }
}

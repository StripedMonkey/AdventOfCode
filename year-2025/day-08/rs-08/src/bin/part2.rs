use std::collections::{HashSet, VecDeque};

use itertools::Itertools as _;
fn parse(lines: &str) -> Vec<Coord> {
    lines
        .lines()
        .map(|line| {
            let values = line
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Coord {
                x: values[0],
                y: values[1],
                z: values[2],
            }
        })
        .collect::<Vec<Coord>>()
}

fn generate_edges(
    junction_boxes: &Vec<Coord>,
    edge_count: usize,
) -> impl Iterator<Item = (&Coord, &Coord)> {
    junction_boxes
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(a, b)| ((a, b), a.euclidean_distance(b)))
        .sorted_by(|(_edge_a, len_a), (_edge_b, len_b)| len_a.partial_cmp(len_b).unwrap())
        .map(|(e, _len)| e)
        .take(edge_count)
}

fn count_clusters(
    junction_boxes: &Vec<Coord>,
    edges: &HashSet<(&Coord, &Coord)>,
) -> Vec<HashSet<Coord>> {
    let mut clusters: Vec<HashSet<Coord>> = Vec::new();
    let mut working_set = junction_boxes.clone();
    loop {
        let mut queue = VecDeque::new();

        let Some(junction) = working_set.pop() else {
            break;
        };
        for cluster in &clusters {
            if cluster.contains(&junction) {
                continue;
            }
        }
        queue.push_back(junction);

        let mut cluster = HashSet::new();
        while let Some(current_junction) = queue.pop_back() {
            for (a, b) in edges.iter() {
                if **a == current_junction && !cluster.contains(*b) {
                    cluster.insert(**b);
                    queue.push_back(**b);
                    working_set.retain(|e| e != *b);
                }
                if **b == current_junction && !cluster.contains(a) {
                    cluster.insert(**a);
                    queue.push_back(**a);
                    working_set.retain(|e| e != *a);
                }
            }
        }
        cluster.insert(junction);
        clusters.push(cluster);
    }
    clusters
}

fn main() {
    let input = rs_2025_08::static_read("input1.txt");
    let junction_boxes = parse(input);

    let mut max_edges = 10;
    let answer = loop {
        let mut max_edges = loop {
            let edges = generate_edges(&junction_boxes, max_edges).collect::<HashSet<_>>();
            let clusters = count_clusters(&junction_boxes, &edges);
            if clusters.len() > 1 {
                max_edges *= 2;
                continue;
            }
            break max_edges;
        };
        let mut min_edges = max_edges / 2;
        println!("Starting search range: {} {}", max_edges, min_edges);
        let num_edges = loop {
            // Binary search for minimum edges
            let current_edges = min_edges + (max_edges - min_edges) / 2;
            let edges = generate_edges(&junction_boxes, current_edges).collect::<HashSet<_>>();
            let clusters = count_clusters(&junction_boxes, &edges);
            if clusters.len() > 1 {
                min_edges = current_edges;
            }
            if clusters.len() == 1 {
                max_edges = current_edges;
            }
            if max_edges - min_edges <= 1 {
                assert!(
                    clusters.len() == 1,
                    "Expected 1 cluster but got {} for edge_count {}",
                    clusters.len(),
                    current_edges
                );
                break max_edges;
            }
        };
        let edges = generate_edges(&junction_boxes, num_edges).collect::<HashSet<_>>();
        let clusters = count_clusters(&junction_boxes, &edges);
        println!("Min edges: {}", min_edges);
        println!("Clusters: {:?}", clusters.len());
        let (last_edge_a, last_edge_b) = generate_edges(&junction_boxes, num_edges).last().unwrap();
        println!("Edges used: {:?}", (last_edge_a, last_edge_b));
        break last_edge_a.x * last_edge_b.x;
    };
    println!("Answer: {}", answer);
    assert_eq!(answer, 1026594680);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl Coord {
    fn euclidean_distance(&self, other: &Coord) -> f64 {
        let dx = (self.x as isize - other.x as isize) as f64;
        let dy = (self.y as isize - other.y as isize) as f64;
        let dz = (self.z as isize - other.z as isize) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{count_clusters, generate_edges, parse};

    #[test]
    fn first_test() {
        let input = rs_2025_08::static_read("example1.txt");
        let junction_boxes = parse(input);

        let mut max_edges = 10;
        let answer = loop {
            let mut max_edges = loop {
                let edges = generate_edges(&junction_boxes, max_edges).collect::<HashSet<_>>();
                let clusters = count_clusters(&junction_boxes, &edges);
                if clusters.len() > 1 {
                    max_edges *= 2;
                    continue;
                }
                break max_edges;
            };
            let mut min_edges = max_edges / 2;
            println!("Starting search range: {} {}", max_edges, min_edges);
            let num_edges = loop {
                // Binary search for minimum edges
                let current_edges = min_edges + (max_edges - min_edges) / 2;
                let edges = generate_edges(&junction_boxes, current_edges).collect::<HashSet<_>>();
                let clusters = count_clusters(&junction_boxes, &edges);
                if clusters.len() > 1 {
                    min_edges = current_edges;
                }
                if clusters.len() == 1 {
                    max_edges = current_edges;
                }
                if max_edges - min_edges <= 1 {
                    assert!(
                        clusters.len() == 1,
                        "Expected 1 cluster but got {} for edge_count {}",
                        clusters.len(),
                        current_edges
                    );
                    break max_edges;
                }
            };
            let edges = generate_edges(&junction_boxes, num_edges).collect::<HashSet<_>>();
            let clusters = count_clusters(&junction_boxes, &edges);
            println!("Min edges: {}", min_edges);
            println!("Clusters: {:?}", clusters);
            let (last_edge_a, last_edge_b) =
                generate_edges(&junction_boxes, num_edges).last().unwrap();
            println!("Edges used: {:?}", (last_edge_a, last_edge_b));
            break last_edge_a.x * last_edge_b.x;
        };
        println!("Answer: {}", answer);
        assert_eq!(answer, 25272);
    }
}

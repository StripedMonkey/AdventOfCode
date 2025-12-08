use std::collections::{HashSet, VecDeque};

use itertools::Itertools as _;

fn main() {
    let input = rs_2025_08::static_read("input1.txt");
    let junction_boxes = input
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
        .collect::<Vec<Coord>>();

    let mut edges = HashSet::new();
    while edges.len() < 1000 {
        let edge_lengths = junction_boxes
            .iter()
            .tuple_combinations::<(_, _)>()
            .map(|(a, b)| ((a, b), a.euclidean_distance(b)))
            .sorted_by(|(_edge_a, len_a), (_edge_b, len_b)| len_a.partial_cmp(len_b).unwrap())
            .take(1000)
            .collect::<Vec<_>>();
        edges.extend(edge_lengths.iter().map(|(e, _len)| e));
    }
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
    for cluster in &clusters {
        println!("Clusters {}: {:?}", cluster.len(), cluster);
    }
    clusters.sort_by_key(|c| c.len());
    let answer = clusters[clusters.len() - 3..]
        .iter()
        .map(|c| c.len())
        .product::<usize>();
    println!("Answer: {}", answer);
    assert_eq!(answer, 63920);
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
    use std::collections::{HashSet, VecDeque};

    use itertools::Itertools;

    use crate::Coord;

    #[test]
    fn first_test() {
        let input = rs_2025_08::static_read("example1.txt");
        let junction_boxes = input
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
            .collect::<Vec<Coord>>();

        let mut edges = HashSet::new();
        while edges.len() < 10 {
            let edge_lengths = junction_boxes
                .iter()
                .tuple_combinations::<(_, _)>()
                .map(|(a, b)| ((a, b), a.euclidean_distance(b)))
                .sorted_by(|(_edge_a, len_a), (_edge_b, len_b)| len_a.partial_cmp(len_b).unwrap())
                .take(10)
                .collect::<Vec<_>>();
            edges.extend(edge_lengths.iter().map(|(e, _len)| e));
        }
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
        for cluster in &clusters {
            println!("Clusters {}: {:?}", cluster.len(), cluster);
        }
        clusters.sort_by_key(|c| c.len());
        let answer = clusters[clusters.len() - 3..]
            .iter()
            .map(|c| c.len())
            .product::<usize>();
        println!("Answer: {}", answer);
        assert_eq!(clusters.len(), 11);
        assert_eq!(answer, 40);
    }
}

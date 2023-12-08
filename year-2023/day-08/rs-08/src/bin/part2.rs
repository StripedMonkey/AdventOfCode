use num::integer;
use rs_08::*;

fn main() {
    let file = *INPUT_1;
    // let file = rs_08::static_read("input2.txt");
    let result = parse_network(file).unwrap().1;
    result
        .starting_nodes("A")
        .map(|start| result.find_cycle(start))
        .for_each(|cycle| {
            println!(
                "Cycle length: {}, Start: {}, num steps: {}",
                cycle.length,
                cycle.start,
                cycle.steps.len()
            );
        });
    let mut a = result
        .starting_nodes("A")
        .map(|start| result.find_cycle(start));
    if let Some(starting) = a.next() {
        let (start_lcm, len_lcm) = a.fold((starting.start, starting.length), |a, b| {
            (integer::lcm(a.0, b.start), integer::lcm(a.1, b.length))
        });
        println!("Cycle Length LCM: {len_lcm:?} Start LCM: {start_lcm:?}");
    }
}

#[cfg(test)]
mod test {
    use num::integer;

    use super::*;

    #[test]
    fn first_test() {
        let file = rs_08::static_read("example1.txt");
        let result = parse_network(&file).unwrap().1;
        let result = result
            .walk("AAA")
            .inspect(|i| {
                println!("Curent Node: {i:?}");
            })
            .take_while(|(idx, node)| *node != "ZZZ")
            .count();
        assert_eq!(result, 2);
        println!("{result:?}")
    }

    #[test]
    fn second_test() {
        let file = rs_08::static_read("example2.txt");
        let result = parse_network(&file).unwrap().1;
        let result = result
            .walk("AAA")
            .inspect(|i| {
                println!("Curent Node: {i:?}");
            })
            .take_while(|(idx, node)| *node != "ZZZ")
            .count();
        assert_eq!(result, 6);
        println!("{result:?}")
    }

    #[test]
    fn third_test() {
        let file = rs_08::static_read("example2.txt");
        let result = parse_network(&file).unwrap().1;
        let mut walkers: Vec<_> = result
            .starting_nodes("A")
            .map(|start_node| {
                result
                    .walk(start_node)
                    .map(|(i, node)| (node, node.ends_with("Z")))
            })
            .collect();
        let mut steps = 0;
        while walkers
            .iter_mut()
            .map(|x| x.next())
            .fold(true, |acc, x| !(acc && x.is_some_and(|(_, ending)| ending)))
        {
            steps += 1;
        }
        assert_eq!(steps, 6);
    }

    #[test]
    fn fourth_test() {
        let file = rs_08::static_read("example3.txt");
        let result = parse_network(&file).unwrap().1;
        result
            .starting_nodes("A")
            .map(|start| result.find_cycle(start))
            .for_each(|cycle| {
                println!(
                    "Cycle length: {}, Start: {}, num steps: {}",
                    cycle.length,
                    cycle.start,
                    cycle.steps.len()
                );
            });
        let mut a = result
            .starting_nodes("A")
            .map(|start| result.find_cycle(start));
        if let Some(starting) = a.next() {
            let (start_lcm, len_lcm) = a.fold((starting.start, starting.length), |a, b| {
                (integer::lcm(a.0, b.start), integer::lcm(a.1, b.length))
            });
            println!("Cycle Length LCM: {len_lcm:?} Start LCM: {start_lcm:?}");
        }
    }
}

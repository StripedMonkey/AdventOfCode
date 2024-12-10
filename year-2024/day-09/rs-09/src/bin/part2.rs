use std::collections::VecDeque;

use itertools::Itertools;
use rs_2024_09::*;

fn main() {
    let input = rs_2024_09::static_read("input1.txt");
    let mut blocks = parse(input);
    let mut result = VecDeque::new();
    'done: loop {
        let file = loop {
            match blocks.pop_back() {
                Some(Block::File(file)) => break file,
                Some(block @ Block::Free(_)) => {
                    result.push_front(block);
                },
                None => break 'done,
            }
        };
        for (idx, block) in blocks.iter_mut().enumerate() {
            match block {
                block @ &mut Block::Free(size) if size == file.size => {
                    *block = Block::File(file);
                    blocks.push_back(Block::Free(size));
                    continue 'done;
                }
                block @ &mut Block::Free( size) if size > file.size => {
                    *block = Block::Free(size - file.size);
                    blocks.push_back(Block::Free(file.size));
                    blocks.insert(idx, Block::File(file));
                    continue 'done;
                }
                _ => continue,
            }
        }
        result.push_front(Block::File(file));
    }
    // for block in &result {
    //     match block {
    //         Block::File(file) => {
    //             for _ in 0..file.size {
    //                 print!("{}", file.id);
    //             }
    //         },
    //         Block::Free(n) => {
    //             for _ in 0..*n {
    //                 print!(".");
    //             }
    //         },
    //     }
    // }
    println!();
    let result: usize = calculate_checksum(result);
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let input = rs_2024_09::static_read("example1.txt");
        let mut blocks = parse(input);
        let mut result = VecDeque::new();
        'done: loop {
            let file = loop {
                match blocks.pop_back() {
                    Some(Block::File(file)) => break file,
                    Some(block @ Block::Free(_)) => {
                        result.push_front(block);
                    },
                    None => break 'done,
                }
            };
            for (idx, block) in blocks.iter_mut().enumerate() {
                match block {
                    block @ &mut Block::Free(size) if size == file.size => {
                        *block = Block::File(file);
                        blocks.push_back(Block::Free(size));
                        continue 'done;
                    }
                    block @ &mut Block::Free( size) if size > file.size => {
                        *block = Block::Free(size - file.size);
                        blocks.push_back(Block::Free(file.size));
                        blocks.insert(idx, Block::File(file));
                        continue 'done;
                    }
                    _ => continue,
                }
            }
            result.push_front(Block::File(file));
        }
        for block in &result {
            match block {
                Block::File(file) => {
                    for _ in 0..file.size {
                        print!("{}", file.id);
                    }
                },
                Block::Free(n) => {
                    for _ in 0..*n {
                        print!(".");
                    }
                },
            }
        }
        println!();
        let result: usize = calculate_checksum(result);
        println!("{}", result);
        assert!(result == 2858);
    }
}

fn calculate_checksum(blocks: VecDeque<Block>) -> usize {
    let mut checksum = 0;
    let mut idx = 0;
    for block in blocks {
        match block {
            Block::File(file) => {
                for _ in 0..file.size {
                    checksum += file.id * idx;
                    idx += 1;
                }
            }
            Block::Free(free) => idx += free,
        }
    }
    checksum
}

struct File {
    id: usize,
    size: usize,
}

enum Block {
    File(File),
    Free(usize),
}

fn parse(input: &str) -> VecDeque<Block> {
    let (files, free) = input
        .chars()
        .filter(|c| char::is_numeric(*c))
        .enumerate()
        .fold(
            (Vec::new(), Vec::new()),
            |(mut files, mut free): (Vec<File>, Vec<usize>), (idx, size)| {
                if idx % 2 == 0 {
                    files.push(File {
                        id: files.len(),
                        size: size.to_string().parse().unwrap(),
                    });
                } else {
                    free.push(size.to_string().parse().unwrap());
                }
                (files, free)
            },
        );
    files
        .into_iter()
        .map(Block::File)
        .interleave(free.into_iter().map(Block::Free))
        .collect()
}
